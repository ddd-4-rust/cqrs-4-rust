//! Projection position tracking service.
//!
//! 1:1 translation of `org.fuin.cqrs4j.esc.ProjectionService`.

use async_trait::async_trait;

/// Tracks the last-read position for a CQRS projection.
///
/// Java: `ProjectionService`
#[async_trait]
pub trait ProjectionService: Send + Sync {
    /// Resets the projection position for a given stream.
    async fn reset_projection_position(&self, stream_id: &str) -> Result<(), ProjectionError>;

    /// Reads the current projection position.
    async fn read_projection_position(&self, stream_id: &str) -> Result<Option<ProjectionPosition>, ProjectionError>;

    /// Updates the projection position after processing events.
    async fn update_projection_position(
        &self,
        stream_id: &str,
        position: &ProjectionPosition,
    ) -> Result<(), ProjectionError>;
}

/// A projection position tracking the last processed event number.
#[derive(Debug, Clone)]
pub struct ProjectionPosition {
    /// The stream identifier.
    pub stream_id: String,
    /// The next event number to read.
    pub next_position: i64,
}

/// Errors during projection position operations.
#[derive(Debug, thiserror::Error)]
pub enum ProjectionError {
    #[error("Stream not found: {0}")]
    StreamNotFound(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("{0}")]
    Other(String),
}
