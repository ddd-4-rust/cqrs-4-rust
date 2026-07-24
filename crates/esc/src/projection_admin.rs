//! Projection administration SPI.
//!
//! # Java 对应
//!
//! - 接口：`org.fuin.esc.api.ProjectionAdminEventStore`
//! - 调用点：
//!   - `QuarkusJpaViewManager.readStreamEvents(...)`
//!   - `SpringJpaViewManager.readStreamEvents(...)`
//!
//! ```java
//! // Create an event store projection if it does not exist.
//! if (!admin.projectionExists(view.getProjectionStreamId())) {
//!     admin.createProjection(view.getProjectionStreamId(), true, typeNames);
//! }
//! ```
//!
//! Real `KurrentDB` gRPC clients can implement [`ProjectionAdmin`] later; this crate
//! ships [`MemoryProjectionAdmin`] for local development and tests.

#![allow(clippy::doc_markdown)]

use crate::ProjectionStreamId;
use async_trait::async_trait;
use ddd_4_rust_core::EventType;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

/// Errors while creating or querying projections.
///
/// # Java 对应
///
/// Java 侧以运行时异常表达（如 `StreamAlreadyExistsException`）；Rust 用结构化错误返回。
#[derive(Debug, thiserror::Error)]
pub enum ProjectionAdminError {
    /// The projection already exists with a conflicting definition.
    ///
    /// Java: `org.fuin.esc.api.StreamAlreadyExistsException`（Spring 路径会捕获并记日志）.
    #[error("projection already exists: {0}")]
    AlreadyExists(String),
    /// Persistence or registry failure.
    #[error("{0}")]
    Other(String),
}

/// Creates and inspects CQRS projection streams.
///
/// # Java 对应
///
/// `org.fuin.esc.api.ProjectionAdminEventStore`
#[async_trait]
pub trait ProjectionAdmin: Send + Sync {
    /// Returns whether a projection stream is already registered.
    ///
    /// # Java 对应
    ///
    /// `ProjectionAdminEventStore.projectionExists(ProjectionStreamId)`
    ///
    /// 调用点：`QuarkusJpaViewManager.readStreamEvents` /
    /// `SpringJpaViewManager.readStreamEvents` 中的
    /// `if (!admin.projectionExists(...)) { ... }`。
    async fn projection_exists(
        &self,
        stream_id: &ProjectionStreamId,
    ) -> Result<bool, ProjectionAdminError>;

    /// Registers a projection that includes the given event types.
    ///
    /// # Java 对应
    ///
    /// `ProjectionAdminEventStore.createProjection(ProjectionStreamId, boolean ordered, List<TypeName>)`
    ///
    /// Java 调用：
    /// ```java
    /// admin.createProjection(view.getProjectionStreamId(), true, typeNames);
    /// ```
    ///
    /// `ordered` mirrors the Java API flag; the in-memory admin records it but
    /// filtering is always deterministic by event number.
    async fn create_projection(
        &self,
        stream_id: &ProjectionStreamId,
        ordered: bool,
        type_names: &[EventType],
    ) -> Result<(), ProjectionAdminError>;

    /// Returns the event types registered for a projection, if any.
    ///
    /// # Java 对应
    ///
    /// Java `ProjectionAdminEventStore` 无等价查询方法；此方法为 Rust 内存实现与测试辅助扩展。
    async fn event_types_for(
        &self,
        stream_id: &ProjectionStreamId,
    ) -> Result<Option<HashSet<EventType>>, ProjectionAdminError>;
}

/// In-memory projection registry used with a process-local event store.
///
/// # Java 对应
///
/// 进程内替代 `GrpcProjectionAdminEventStore`（见 Java example 的
/// `ProjectionAdminEventStoreFactory`）。不连接外部 Event Store，仅登记投影元数据，
/// 实际读事件由 [`crate::ViewProjector`] 对 `$all` 按类型过滤完成。
#[derive(Debug, Default)]
pub struct MemoryProjectionAdmin {
    /// Registered projections keyed by stream id string.
    projections: RwLock<HashMap<String, ProjectionDefinition>>,
}

/// Internal projection metadata recorded by [`MemoryProjectionAdmin`].
#[derive(Debug, Clone)]
struct ProjectionDefinition {
    /// Mirrors Java `createProjection(..., ordered, ...)`; retained for API parity.
    #[allow(dead_code)]
    ordered: bool,
    /// Event types included in this projection (`List<TypeName>` in Java).
    type_names: HashSet<EventType>,
}

impl MemoryProjectionAdmin {
    /// Creates an empty projection registry.
    ///
    /// # Java 对应
    ///
    /// 无直接 Java 类型；等价于启动时尚未调用 `createProjection` 的空 admin。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Acquires a write lock on the projection registry.
    fn lock(
        &self,
    ) -> Result<
        std::sync::RwLockWriteGuard<'_, HashMap<String, ProjectionDefinition>>,
        ProjectionAdminError,
    > {
        self.projections
            .write()
            .map_err(|_| ProjectionAdminError::Other("projection admin lock poisoned".to_owned()))
    }

    /// Acquires a read lock on the projection registry.
    fn read_lock(
        &self,
    ) -> Result<
        std::sync::RwLockReadGuard<'_, HashMap<String, ProjectionDefinition>>,
        ProjectionAdminError,
    > {
        self.projections
            .read()
            .map_err(|_| ProjectionAdminError::Other("projection admin lock poisoned".to_owned()))
    }
}

#[async_trait]
impl ProjectionAdmin for MemoryProjectionAdmin {
    /// See [`ProjectionAdmin::projection_exists`].
    async fn projection_exists(
        &self,
        stream_id: &ProjectionStreamId,
    ) -> Result<bool, ProjectionAdminError> {
        Ok(self.read_lock()?.contains_key(stream_id.as_str()))
    }

    /// See [`ProjectionAdmin::create_projection`].
    ///
    /// 幂等：若投影已存在则直接返回 `Ok`（对齐 Spring 对
    /// `StreamAlreadyExistsException` 的容忍处理）。
    async fn create_projection(
        &self,
        stream_id: &ProjectionStreamId,
        ordered: bool,
        type_names: &[EventType],
    ) -> Result<(), ProjectionAdminError> {
        let mut projections = self.lock()?;
        // Java Spring: catch StreamAlreadyExistsException after race with projectionExists
        if projections.contains_key(stream_id.as_str()) {
            return Ok(());
        }
        let mut names = HashSet::new();
        for event_type in type_names {
            names.insert(event_type.clone());
        }
        projections.insert(
            stream_id.as_str().to_owned(),
            ProjectionDefinition {
                ordered,
                type_names: names,
            },
        );
        Ok(())
    }

    /// See [`ProjectionAdmin::event_types_for`].
    async fn event_types_for(
        &self,
        stream_id: &ProjectionStreamId,
    ) -> Result<Option<HashSet<EventType>>, ProjectionAdminError> {
        Ok(self
            .read_lock()?
            .get(stream_id.as_str())
            .map(|definition| definition.type_names.clone()))
    }
}
