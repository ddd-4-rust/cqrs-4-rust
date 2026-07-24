//! Contract for dispatching persisted events to JPA-style event handlers.

use async_trait::async_trait;
use cqrs_4_rust_core::JpaEventHandlerError;
use ddd_4_rust_core::{EventType, event::Event};

/// Dispatches events to registered persistence event handlers.
#[async_trait]
pub trait JpaEventDispatcher: Send + Sync {
    /// Returns all handled event types.
    fn all_types(&self) -> Vec<EventType>;

    /// Dispatches each common event according to its runtime event type.
    async fn dispatch_common_events(
        &self,
        events: &[Box<dyn Event>],
    ) -> Result<(), JpaEventHandlerError>;

    /// Dispatches each event according to its runtime event type.
    async fn dispatch_events(&self, events: &[Box<dyn Event>]) -> Result<(), JpaEventHandlerError>;

    /// Dispatches one event to every handler registered for its type.
    async fn dispatch_event(&self, event: &dyn Event) -> Result<(), JpaEventHandlerError>;
}
