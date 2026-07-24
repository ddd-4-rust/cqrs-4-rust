//! HashMap-backed implementation of [`JpaEventDispatcher`].

use crate::JpaEventDispatcher;
use async_trait::async_trait;
use cqrs_4_rust_core::{JpaEventHandler, JpaEventHandlerError};
use ddd_4_rust_core::EventType;
use ddd_4_rust_core::event::{Event, downcast_event};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

#[async_trait]
trait ErasedEventHandler: Send + Sync {
    async fn handle(&self, event: &dyn Event) -> Result<(), JpaEventHandlerError>;
}

struct TypedEventHandler<EventTypeValue, Handler>
where
    EventTypeValue: Event,
    Handler: JpaEventHandler<EventTypeValue>,
{
    handler: Arc<Handler>,
    event_marker: PhantomData<fn() -> EventTypeValue>,
}

#[async_trait]
impl<EventTypeValue, Handler> ErasedEventHandler for TypedEventHandler<EventTypeValue, Handler>
where
    EventTypeValue: Event + 'static,
    Handler: JpaEventHandler<EventTypeValue> + 'static,
{
    async fn handle(&self, event: &dyn Event) -> Result<(), JpaEventHandlerError> {
        let typed_event = downcast_event::<EventTypeValue>(event).ok_or_else(|| {
            JpaEventHandlerError::Other(format!(
                "event {} could not be downcast to its registered Rust type",
                event.event_type()
            ))
        })?;
        self.handler.handle(typed_event).await
    }
}

/// HashMap-backed dispatcher equivalent to Java's `SimpleJpaEventDispatcher`.
#[derive(Default)]
pub struct SimpleJpaEventDispatcher {
    handlers: HashMap<EventType, Vec<Arc<dyn ErasedEventHandler>>>,
}

impl SimpleJpaEventDispatcher {
    /// Creates an empty dispatcher to be populated with [`Self::register`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a typed handler. Multiple handlers may share one event type.
    pub fn register<EventTypeValue, Handler>(&mut self, handler: Arc<Handler>)
    where
        EventTypeValue: Event + 'static,
        Handler: JpaEventHandler<EventTypeValue> + 'static,
    {
        let event_type = handler.event_type();
        let erased: Arc<dyn ErasedEventHandler> = Arc::new(TypedEventHandler {
            handler,
            event_marker: PhantomData,
        });
        self.handlers.entry(event_type).or_default().push(erased);
    }
}

#[async_trait]
impl JpaEventDispatcher for SimpleJpaEventDispatcher {
    fn all_types(&self) -> Vec<EventType> {
        self.handlers.keys().cloned().collect()
    }

    async fn dispatch_common_events(
        &self,
        events: &[Box<dyn Event>],
    ) -> Result<(), JpaEventHandlerError> {
        self.dispatch_events(events).await
    }

    async fn dispatch_events(&self, events: &[Box<dyn Event>]) -> Result<(), JpaEventHandlerError> {
        for event in events {
            self.dispatch_event(event.as_ref()).await?;
        }
        Ok(())
    }

    async fn dispatch_event(&self, event: &dyn Event) -> Result<(), JpaEventHandlerError> {
        if let Some(handlers) = self.handlers.get(event.event_type()) {
            for handler in handlers {
                handler.handle(event).await?;
            }
        }
        Ok(())
    }
}
