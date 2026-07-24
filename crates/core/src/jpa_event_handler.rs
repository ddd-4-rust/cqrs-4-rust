//! Handles individual event types for a CQRS view.
//!
//! 1:1 translation of `org.fuin.cqrs4j.core.JpaEventHandler`.

use async_trait::async_trait;
use ddd_4_rust_core::EventType;
use ddd_4_rust_core::event::Event;

/// Handles a specific event type and updates database entities accordingly.
///
/// Java: `JpaEventHandler<TYPE extends Event>`
///
/// # Type Parameters
/// - `E`: The event type this handler processes.
#[async_trait]
pub trait JpaEventHandler<E: Event + ?Sized>: Send + Sync {
    /// Returns the event type handled by this instance.
    fn event_type(&self) -> EventType;

    /// Handles the given event.
    ///
    /// Java: `handle(EntityManager em, TYPE event)`
    async fn handle(&self, event: &E) -> Result<(), JpaEventHandlerError>;
}

/// Errors during event handling.
#[derive(Debug, thiserror::Error)]
pub enum JpaEventHandlerError {
    /// A persistence operation failed.
    #[error("Database error: {0}")]
    Database(String),
    /// Any other handler failure.
    #[error("{0}")]
    Other(String),
}
