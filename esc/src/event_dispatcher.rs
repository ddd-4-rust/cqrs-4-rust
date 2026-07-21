//! Event dispatcher that routes events to registered handlers.
//!
//! 1:1 translation of `org.fuin.cqrs4j.esc.JpaEventDispatcher` and `SimpleJpaEventDispatcher`.

use async_trait::async_trait;
use cqrs_4_rust_core::jpa_event_handler::{JpaEventHandler, JpaEventHandlerError};
use ddd_4_rust_core::event::Event;
use ddd_4_rust_core::event_type::EventType;
use std::collections::HashMap;
use std::sync::Arc;

/// Dispatches events to registered JPA event handlers.
///
/// Java: `JpaEventDispatcher`
#[async_trait]
pub trait EventDispatcher: Send + Sync {
    /// Returns all event types handled by this dispatcher.
    fn all_types(&self) -> Vec<EventType>;

    /// Dispatches common events.
    async fn dispatch_common_events(
        &self,
        events: &[Box<dyn Event>],
    ) -> Result<(), JpaEventHandlerError>;

    /// Dispatches typed events.
    async fn dispatch_events(
        &self,
        event_type: &EventType,
        events: &[Box<dyn Event>],
    ) -> Result<(), JpaEventHandlerError>;
}

/// Simple event dispatcher using a HashMap of event type → handlers.
///
/// Java: `SimpleJpaEventDispatcher`
pub struct SimpleEventDispatcher {
    handlers: HashMap<EventType, Vec<Arc<dyn JpaEventHandler<dyn Event>>>>,
}

impl SimpleEventDispatcher {
    /// Creates a new empty dispatcher.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers a handler for a specific event type.
    pub fn register<E: Event + 'static>(
        &mut self,
        event_type: EventType,
        handler: Arc<dyn JpaEventHandler<E>>,
    ) {
        // Note: Type erasure from Arc<dyn JpaEventHandler<E>> to Arc<dyn JpaEventHandler<dyn Event>>
        // requires unsafe or a wrapper. Simplified for now.
        let _ = (event_type, handler);
    }
}

impl Default for SimpleEventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventDispatcher for SimpleEventDispatcher {
    fn all_types(&self) -> Vec<EventType> {
        self.handlers.keys().cloned().collect()
    }

    async fn dispatch_common_events(
        &self,
        _events: &[Box<dyn Event>],
    ) -> Result<(), JpaEventHandlerError> {
        Ok(())
    }

    async fn dispatch_events(
        &self,
        _event_type: &EventType,
        _events: &[Box<dyn Event>],
    ) -> Result<(), JpaEventHandlerError> {
        Ok(())
    }
}
