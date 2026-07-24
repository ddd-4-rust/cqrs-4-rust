//! Axum view manager migrated from the Java Spring Boot implementation.
//!
//! # Java 对应
//!
//! `org.fuin.cqrs4j.springboot.view.SpringJpaViewManager`
//!
//! Java source doc: *"Creates scheduler update tasks for all classes implementing the
//! {@link View} interface. Avoids boilerplate code: Instead of having a separated
//! \"Projector\", \"EventDispatcher\" and a \"ChunkHandler\" class for each view,
//! there is only one simplified \"View\" class now."*

#![allow(clippy::doc_markdown)]

use cqrs_4_rust_core::JpaView;
use cqrs_4_rust_esc::{
    EventDecoder, ManagedView, ProjectionAdmin, ProjectionService, ViewProjector,
    ViewProjectorError,
};
use ddd_4_rust_esc::EventStore;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

/// Creates scheduler update tasks for all classes implementing [`JpaView`] / `View`.
///
/// Avoids boilerplate: instead of a separate Projector / EventDispatcher / ChunkHandler
/// per view, there is one simplified view type driven by this manager.
///
/// # Java 对应
///
/// `org.fuin.cqrs4j.springboot.view.SpringJpaViewManager`
/// （`SchedulingConfigurer` + `ApplicationListener<ContextClosedEvent>`）
///
/// | Java | Rust |
/// |---|---|
/// | `SpringJpaViewManager(...)` 构造 | [`AxumJpaViewManager::new`] |
/// | `configureTasks` → `createViews` | [`AxumJpaViewManager::start`] |
/// | `onApplicationEvent` → `shutdownViews` | [`AxumJpaViewManager::stop`] |
/// | `updateView` / `readStreamEvents` / `handleChunk` | [`ViewProjector::tick`] |
/// | `ViewExt` | [`ManagedView`] |
pub struct AxumJpaViewManager {
    /// User-defined views wrapped with projection stream id + lock.
    ///
    /// Java: `List<JpaView> rawViews` → `List<ViewExt> views`。
    managed_views: Vec<Arc<ManagedView>>,
    /// Shared projection engine（Java 中内联在 View Manager 私有方法里）.
    projector: Arc<ViewProjector>,
    /// CRON scheduler（Java: `ScheduledTaskRegistrar` + `CronTask`）.
    scheduler: Mutex<Option<JobScheduler>>,
    /// Scheduled job ids for graceful unschedule（Java: 通过 `CronTask` 引用取消）.
    job_ids: Mutex<Vec<Uuid>>,
}

impl AxumJpaViewManager {
    /// Constructor with mandatory data.
    ///
    /// # Java 对应
    ///
    /// `SpringJpaViewManager(ScheduledAnnotationBeanPostProcessor, List<JpaView>,
    /// EventStore, ProjectionAdminEventStore, ProjectionService,
    /// PlatformTransactionManager, EntityManagerFactory)`
    ///
    /// | Java 参数 | Rust 参数 |
    /// |---|---|
    /// | `rawViews` | `views` |
    /// | `eventstore` | `event_store` |
    /// | `admin` | `admin` |
    /// | `projectionService` | `projection_service` |
    /// | （序列化强转） | `decoder` |
    /// | `postProcessor` / `transactionManager` / `entityManagerFactory` | 由适配器/应用侧另行处理，不注入本类型 |
    #[must_use]
    pub fn new(
        views: Vec<Arc<dyn JpaView>>,
        event_store: Arc<dyn EventStore>,
        admin: Arc<dyn ProjectionAdmin>,
        projection_service: Arc<dyn ProjectionService>,
        decoder: Arc<dyn EventDecoder>,
    ) -> Self {
        // Java createViews: views = rawViews.stream().map(ViewExt::new).toList()
        let managed_views = views
            .into_iter()
            .map(|view| Arc::new(ManagedView::new(view)))
            .collect();
        let projector = Arc::new(ViewProjector::new(
            event_store,
            admin,
            projection_service,
            decoder,
        ));
        Self {
            managed_views,
            projector,
            scheduler: Mutex::new(None),
            job_ids: Mutex::new(Vec::new()),
        }
    }

    /// Returns the registered views.
    ///
    /// # Java 对应
    ///
    /// 无公开 getter；等价于访问注入的 `rawViews` / 内部 `views` 的 delegate。
    #[must_use]
    pub fn views(&self) -> Vec<Arc<dyn JpaView>> {
        self.managed_views
            .iter()
            .map(|managed| Arc::clone(&managed.view))
            .collect()
    }

    /// Returns managed view metadata including projection stream ids.
    ///
    /// # Java 对应
    ///
    /// 内部 `List<ViewExt>`（含 `getProjectionStreamId()` / `getLock()`）。
    #[must_use]
    pub fn managed_views(&self) -> &[Arc<ManagedView>] {
        &self.managed_views
    }

    /// Runs one projection tick for every registered view (useful in tests).
    ///
    /// # Java 对应
    ///
    /// 无同名公开方法；语义等同于对每个 view 调用一次 `updateView(view)`。
    ///
    /// # Errors
    ///
    /// Propagates the first [`ViewProjectorError`] from any view.
    pub async fn tick_all(&self) -> Result<u64, AxumJpaViewManagerError> {
        let mut applied = 0_u64;
        for managed in &self.managed_views {
            applied = applied.saturating_add(self.projector.tick(managed).await?);
        }
        Ok(applied)
    }

    /// Starts the CRON scheduler for all registered views.
    ///
    /// # Java 对应
    ///
    /// `configureTasks(ScheduledTaskRegistrar)` → `createViews(taskRegistrar)`：
    /// ```java
    /// view.setCronTask(new CronTask(() -> updateView(view), view.getCron()));
    /// taskRegistrar.addCronTask(view.getCronTask());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a scheduler error when the job scheduler cannot start.
    pub async fn start(&self) -> Result<(), AxumJpaViewManagerError> {
        let mut scheduler_guard = self.scheduler.lock().await;
        if scheduler_guard.is_some() {
            return Ok(());
        }

        let scheduler = JobScheduler::new()
            .await
            .map_err(|error| AxumJpaViewManagerError::Scheduler(error.to_string()))?;

        let mut job_ids = Vec::new();
        // Java createViews: for (final ViewExt view : views) { ... addCronTask ... }
        for managed in &self.managed_views {
            let cron = managed.view.cron().to_owned();
            let view_name = managed.view.name().to_owned();
            let projector = Arc::clone(&self.projector);
            let managed_view = Arc::clone(managed);

            log::info!("Registering Spring-migrated view '{view_name}' with CRON '{cron}' in Axum");

            // Java: new CronTask(() -> updateView(view), view.getCron())
            let job = Job::new_async(cron.as_str(), move |_uuid, _lock| {
                let projector = Arc::clone(&projector);
                let managed_view = Arc::clone(&managed_view);
                Box::pin(async move {
                    // Java updateView: catch RuntimeException and LOG.error
                    if let Err(error) = projector.tick(&managed_view).await {
                        log::error!(
                            "Error projecting view '{}': {error}",
                            managed_view.view.name()
                        );
                    }
                })
            })
            .map_err(|error| AxumJpaViewManagerError::Scheduler(error.to_string()))?;

            let job_id = scheduler
                .add(job)
                .await
                .map_err(|error| AxumJpaViewManagerError::Scheduler(error.to_string()))?;
            job_ids.push(job_id);
        }

        scheduler
            .start()
            .await
            .map_err(|error| AxumJpaViewManagerError::Scheduler(error.to_string()))?;

        *self.job_ids.lock().await = job_ids;
        *scheduler_guard = Some(scheduler);
        Ok(())
    }

    /// Stops all scheduled tasks.
    ///
    /// # Java 对应
    ///
    /// `onApplicationEvent(ContextClosedEvent)` → `shutdownViews()`：
    /// ```java
    /// scheduledTasks.stream()
    ///     .filter(scheduled -> scheduled.getTask() == view.getCronTask())
    ///     .findFirst()
    ///     .ifPresent(ScheduledTask::cancel);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a scheduler error when graceful shutdown fails.
    pub async fn stop(&self) -> Result<(), AxumJpaViewManagerError> {
        let mut scheduler_guard = self.scheduler.lock().await;
        if let Some(mut scheduler) = scheduler_guard.take() {
            log::info!(
                "Stopping axum view manager with {} views",
                self.managed_views.len()
            );
            // Java shutdownViews: cancel each CronTask
            for job_id in self.job_ids.lock().await.drain(..) {
                if let Err(error) = scheduler.remove(&job_id).await {
                    log::warn!("Failed to unschedule job {job_id}: {error}");
                }
            }
            scheduler
                .shutdown()
                .await
                .map_err(|error| AxumJpaViewManagerError::Scheduler(error.to_string()))?;
        }
        Ok(())
    }
}

/// Errors during view manager operations.
///
/// # Java 对应
///
/// Java 在 `updateView` 内吞掉 `RuntimeException`；调度器失败则向上抛出。
/// Rust 区分调度错误与投影错误。
#[derive(Debug, thiserror::Error)]
pub enum AxumJpaViewManagerError {
    /// The background scheduler could not be started, stopped, or queried.
    #[error("Scheduler error: {0}")]
    Scheduler(String),
    /// Projection tick failed（测试/`tick_all` 路径）.
    #[error(transparent)]
    Projector(#[from] ViewProjectorError),
    /// Another view-manager operation failed.
    #[error("{0}")]
    Other(String),
}
