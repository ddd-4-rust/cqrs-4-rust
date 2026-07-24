//! Shared projection tick engine used by Axum/Actix view managers.
//!
//! # Java 对应
//!
//! 将下列 Java 私有方法抽成可复用的共享内核（避免 Axum/Actix 各写一份）：
//!
//! | Java 方法 | 所属类 | Rust |
//! |---|---|---|
//! | `updateView(ViewExt)` | `QuarkusJpaViewManager` / `SpringJpaViewManager` | [`ViewProjector::tick`]（含锁） |
//! | `readStreamEvents(...)` | 同上 | [`ViewProjector::tick`] 主体 + [`ViewProjector::ensure_projection`] |
//! | `handleChunk(...)` | 同上 | tick 循环内 decode → `handle_events` → `update_projection_position` |
//! | `asEvents(...)` | 同上 | [`ViewProjector::decode_all`] + [`crate::EventDecoder`] |
//! | `ViewExt` | 同上内部类 | [`ManagedView`] |
//!
//! 流程：ensure projection → read chunk → decode → handle → update position。

use crate::{
    EventDecodeError, EventDecoder, ProjectionAdmin, ProjectionAdminError, ProjectionError,
    ProjectionPosition, ProjectionService, ProjectionStreamId,
};
use cqrs_4_rust_core::{JpaView, JpaViewError};
use ddd_4_rust_core::EventType;
use ddd_4_rust_core::event::Event;
use ddd_4_rust_esc::{CommonEvent, EventStore, EventStoreError};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Errors raised while projecting events into a view.
///
/// # Java 对应
///
/// Java `updateView` 捕获 `RuntimeException` 并记日志；Rust 向上返回结构化错误，
/// 由 View Manager 的 CRON 回调决定是否记录。
#[derive(Debug, thiserror::Error)]
pub enum ViewProjectorError {
    /// Projection administration failed.
    ///
    /// Java: `ProjectionAdminEventStore` 调用失败。
    #[error(transparent)]
    Admin(#[from] ProjectionAdminError),
    /// Projection position persistence failed.
    ///
    /// Java: `ProjectionService` 读写失败。
    #[error(transparent)]
    Projection(#[from] ProjectionError),
    /// Event store IO failed.
    ///
    /// Java: `EventStore.readAllEventsForward(...)` 失败。
    #[error(transparent)]
    EventStore(#[from] EventStoreError),
    /// Stored payload could not be decoded.
    ///
    /// Java: `asEvents` 中 `(Event) event.getData()` 失败。
    #[error(transparent)]
    Decode(#[from] EventDecodeError),
    /// View handler failed while applying events.
    ///
    /// Java: `JpaView.handleEvents(EntityManager, List<Event>)` 抛出的异常。
    #[error(transparent)]
    View(#[from] JpaViewError),
    /// Another projector failure.
    #[error("{0}")]
    Other(String),
}

/// One scheduled CQRS view with its exclusive update lock.
///
/// # Java 对应
///
/// `QuarkusJpaViewManager.ViewExt` / `SpringJpaViewManager.ViewExt`
/// （Javadoc: *"Extends the view with some necessary values used only by this class."*）
pub struct ManagedView {
    /// Projection view implementation.
    ///
    /// Java: `ViewExt.delegate`（`JpaView`）。
    pub view: Arc<dyn JpaView>,
    /// Derived projection stream id.
    ///
    /// Java: `ViewExt.projectionStreamId` /
    /// `ViewExt.getProjectionStreamId()`。
    pub stream_id: ProjectionStreamId,
    /// Mutual exclusion so overlapping CRON ticks do not run concurrently.
    ///
    /// Java: `ViewExt.lock`（`Semaphore(1)`），配合 `Utils4J.tryLocked(...)`。
    pub lock: Arc<Mutex<()>>,
}

impl ManagedView {
    /// Wraps a [`JpaView`] with its Adler-32 projection stream id and lock.
    ///
    /// # Java 对应
    ///
    /// `ViewExt(JpaView delegate)` 构造函数：
    /// ```java
    /// final Set<EventType> eventTypes = delegate.getEventTypes();
    /// final String name = delegate.getName() + "-" + CqrsUtils.calculateAdler32Checksum(eventTypes);
    /// projectionStreamId = new ProjectionStreamId(name);
    /// this.lock = new Semaphore(1);
    /// ```
    #[must_use]
    pub fn new(view: Arc<dyn JpaView>) -> Self {
        let event_types = view.event_types();
        let stream_id = ProjectionStreamId::from_view(view.name(), &event_types);
        Self {
            view,
            stream_id,
            lock: Arc::new(Mutex::new(())),
        }
    }
}

/// Runs the Java-equivalent projection update loop for one view.
///
/// # Java 对应
///
/// 聚合 `updateView` / `readStreamEvents` / `handleChunk` 的共享实现，
/// 供 `AxumJpaViewManager`（← `SpringJpaViewManager`）与
/// `ActixJpaViewManager`（← `QuarkusJpaViewManager`）复用。
pub struct ViewProjector {
    /// Event store used to read events.
    ///
    /// Java: `EventStore eventstore` 字段。
    event_store: Arc<dyn EventStore>,
    /// Projection admin used to ensure the projection exists.
    ///
    /// Java: `ProjectionAdminEventStore admin` 字段。
    admin: Arc<dyn ProjectionAdmin>,
    /// Tracks last-read projection position.
    ///
    /// Java: `ProjectionService projectionService` 字段。
    projection_service: Arc<dyn ProjectionService>,
    /// Decodes `CommonEvent` payloads into domain events.
    ///
    /// Java: `asEvents(...)` 中的 `(Event) event.getData()`。
    decoder: Arc<dyn EventDecoder>,
}

impl ViewProjector {
    /// Creates a projector over the shared event-store and projection services.
    ///
    /// # Java 对应
    ///
    /// 无同名构造；对应 View Manager 构造时注入的
    /// `eventstore` / `admin` / `projectionService`（外加 Rust 侧 `EventDecoder`）。
    #[must_use]
    pub fn new(
        event_store: Arc<dyn EventStore>,
        admin: Arc<dyn ProjectionAdmin>,
        projection_service: Arc<dyn ProjectionService>,
        decoder: Arc<dyn EventDecoder>,
    ) -> Self {
        Self {
            event_store,
            admin,
            projection_service,
            decoder,
        }
    }

    /// Ensures the projection exists, then reads and applies all pending chunks.
    ///
    /// # Java 对应
    ///
    /// 1. `updateView(ViewExt)` — 获取锁后调用 `readStreamEvents`
    /// 2. `readStreamEvents(...)` — ensure projection + `readAllEventsForward` + callback
    /// 3. `handleChunk(...)` — `handleEvents` + `updateProjectionPosition`
    ///
    /// Position tracks the global `$all` cursor (filter-in-process), matching the
    /// in-memory projection admin strategy (不依赖真实 Event Store 投影子系统)。
    ///
    /// # Errors
    ///
    /// Returns the first failure from admin, store, decode, view, or position update.
    pub async fn tick(&self, managed: &ManagedView) -> Result<u64, ViewProjectorError> {
        // Java: Utils4J.tryLocked(view.getLock(), () -> ...)
        let _guard = managed.lock.lock().await;

        let event_types = managed.view.event_types();
        // Java readStreamEvents: create projection if it does not exist
        self.ensure_projection(&managed.stream_id, &event_types)
            .await?;

        // Java: asTypeNames(view.getEventTypes()) — used here as in-process filter
        let type_filter: HashSet<String> = event_types
            .iter()
            .map(|event_type| event_type.as_str().to_owned())
            .collect();

        // Java: projectionService.readProjectionPosition(view.getProjectionStreamId())
        let mut position = self
            .projection_service
            .read_projection_position(managed.stream_id.as_str())
            .await?
            .map_or(0, |current| current.next_position);

        let chunk_size = managed.view.chunk_size();
        let mut applied = 0_u64;

        // Java: eventstore.readAllEventsForward(projectionStreamId, next, chunkSize, slice -> handleChunk)
        loop {
            let slice = self
                .event_store
                .read_all_events_forward(position, chunk_size)
                .await?;

            let raw_count = i64::try_from(slice.events.len()).unwrap_or(i64::MAX);
            // In-memory 策略：对 $all 按投影订阅类型过滤（真实 ES 则由投影流本身过滤）
            let filtered: Vec<CommonEvent> = slice
                .events
                .iter()
                .filter(|event| type_filter.contains(&event.event_type))
                .cloned()
                .collect();

            // Java handleChunk: view.handleEvents(em, asEvents(currentSlice.getEvents()))
            if !filtered.is_empty() {
                let domain_events = self.decode_all(&filtered).await?;
                managed.view.handle_events(&domain_events).await?;
                applied =
                    applied.saturating_add(u64::try_from(domain_events.len()).unwrap_or(u64::MAX));
            }

            let advanced = slice
                .next_event_number
                .unwrap_or_else(|| position.saturating_add(raw_count));

            // Java handleChunk: projectionService.updateProjectionPosition(..., currentSlice.getNextEventNumber())
            if advanced != position {
                self.projection_service
                    .update_projection_position(
                        managed.stream_id.as_str(),
                        &ProjectionPosition {
                            stream_id: managed.stream_id.as_str().to_owned(),
                            next_position: advanced,
                        },
                    )
                    .await?;
                position = advanced;
            } else if slice.is_end_of_stream {
                break;
            }

            if slice.is_end_of_stream {
                break;
            }
        }

        Ok(applied)
    }

    /// Creates the event store projection if it does not exist.
    ///
    /// # Java 对应
    ///
    /// `readStreamEvents` 中的：
    /// ```java
    /// if (!admin.projectionExists(view.getProjectionStreamId())) {
    ///     admin.createProjection(view.getProjectionStreamId(), true, typeNames);
    /// }
    /// ```
    async fn ensure_projection(
        &self,
        stream_id: &ProjectionStreamId,
        event_types: &[EventType],
    ) -> Result<(), ViewProjectorError> {
        if !self.admin.projection_exists(stream_id).await? {
            self.admin
                .create_projection(stream_id, true, event_types)
                .await?;
        }
        Ok(())
    }

    /// Decodes a batch of stored events into domain events.
    ///
    /// # Java 对应
    ///
    /// `asEvents(List<CommonEvent> events)`：
    /// `events.stream().map(event -> (Event) event.getData()).toList()`
    async fn decode_all(
        &self,
        events: &[CommonEvent],
    ) -> Result<Vec<Box<dyn Event>>, ViewProjectorError> {
        let mut decoded = Vec::with_capacity(events.len());
        for event in events {
            decoded.push(self.decoder.decode(event).await?);
        }
        Ok(decoded)
    }
}
