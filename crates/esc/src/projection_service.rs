//! Projection position tracking service.
//!
//! # Java 对应
//!
//! `org.fuin.cqrs4j.esc.ProjectionService`
//!
//! Java source doc: *"Provides functionality related to projections."*

#![allow(clippy::doc_markdown)]

use async_trait::async_trait;

/// Tracks the last-read position for a CQRS projection.
///
/// # Java 对应
///
/// `org.fuin.cqrs4j.esc.ProjectionService`
///
/// ```java
/// public interface ProjectionService {
///     void resetProjectionPosition(@NotNull StreamId streamId);
///     @NotNull Long readProjectionPosition(@NotNull StreamId streamId);
///     void updateProjectionPosition(@NotNull StreamId streamId, @NotNull Long nextEventNumber);
/// }
/// ```
#[async_trait]
pub trait ProjectionService: Send + Sync {
    /// Sets the stored position of the projection to the start position.
    ///
    /// # Java 对应
    ///
    /// `ProjectionService.resetProjectionPosition(StreamId streamId)`
    ///
    /// @param stream_id Unique ID of the stream.
    async fn reset_projection_position(&self, stream_id: &str) -> Result<(), ProjectionError>;

    /// Reads the position that was read last time.
    ///
    /// # Java 对应
    ///
    /// `ProjectionService.readProjectionPosition(StreamId streamId)`
    ///
    /// @param stream_id Unique ID of the stream.
    /// @return Number of the next event to read（Rust 用 [`ProjectionPosition`] 包装；
    /// 尚无记录时返回 `None` 或由实现约定为 `0`）。
    async fn read_projection_position(
        &self,
        stream_id: &str,
    ) -> Result<Option<ProjectionPosition>, ProjectionError>;

    /// Updates the position to read next time.
    ///
    /// # Java 对应
    ///
    /// `ProjectionService.updateProjectionPosition(StreamId streamId, Long nextEventNumber)`
    ///
    /// @param stream_id Unique ID of the stream.
    /// @param position Number of the next event to read（见 [`ProjectionPosition::next_position`]）.
    async fn update_projection_position(
        &self,
        stream_id: &str,
        position: &ProjectionPosition,
    ) -> Result<(), ProjectionError>;
}

/// A projection position tracking the last processed event number.
///
/// # Java 对应
///
/// Java 接口直接使用 `Long nextEventNumber`；Rust 将 `streamId` 与位置一并携带，
/// 便于适配器内存实现与后续持久化。
#[derive(Debug, Clone)]
pub struct ProjectionPosition {
    /// The stream identifier (`StreamId` / `ProjectionStreamId`).
    pub stream_id: String,
    /// Number of the next event to read（Java `nextEventNumber`）.
    pub next_position: i64,
}

/// Errors during projection position operations.
///
/// # Java 对应
///
/// Java 以运行时异常表达；Rust 用结构化错误。
#[derive(Debug, thiserror::Error)]
pub enum ProjectionError {
    /// The requested stream does not exist.
    #[error("Stream not found: {0}")]
    StreamNotFound(String),
    /// Projection position persistence failed.
    #[error("Database error: {0}")]
    Database(String),
    /// Any other projection position failure.
    #[error("{0}")]
    Other(String),
}
