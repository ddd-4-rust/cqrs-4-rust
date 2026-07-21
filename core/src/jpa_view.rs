//! JPA-backed CQRS view (projection) with CRON scheduling.
//!
//! 1:1 translation of `org.fuin.cqrs4j.core.JpaView`.

use async_trait::async_trait;
use ddd_4_rust_core::event::Event;
use crate::view::View;

/// A JPA-backed CQRS view with CRON scheduling for event processing.
///
/// Java: `JpaView extends View`
#[async_trait]
pub trait JpaView: View {
    /// Returns the CRON expression for the scheduling interval.
    fn cron(&self) -> &str;

    /// Returns the chunk size for reading events (default 100).
    fn chunk_size(&self) -> u32 {
        100
    }

    /// Handles a batch of events for this view.
    async fn handle_events(
        &self,
        events: &[Box<dyn Event>],
    ) -> Result<(), JpaViewError>;
}

/// Errors during JPA view event handling.
#[derive(Debug, thiserror::Error)]
pub enum JpaViewError {
    #[error("Entity not found: {0}")]
    EntityNotFound(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("{0}")]
    Other(String),
}
