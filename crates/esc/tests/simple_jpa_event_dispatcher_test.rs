//! `SimpleJpaEventDispatcher` behavior checks.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cqrs_4_rust_core::{JpaEventHandler, JpaEventHandlerError};
use cqrs_4_rust_esc::{JpaEventDispatcher, SimpleJpaEventDispatcher};
use ddd_4_rust_core::{Event, EventId, EventType};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

struct TestEvent {
    event_id: EventId,
    event_type: EventType,
    timestamp: DateTime<Utc>,
}

impl TestEvent {
    fn new(event_type: &str) -> Self {
        Self {
            event_id: EventId::new(),
            event_type: EventType::new(event_type).expect("valid test event type"),
            timestamp: Utc::now(),
        }
    }
}

impl Event for TestEvent {
    fn event_id(&self) -> &EventId {
        &self.event_id
    }

    fn event_type(&self) -> &EventType {
        &self.event_type
    }

    fn event_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    fn correlation_id(&self) -> Option<&EventId> {
        None
    }

    fn causation_id(&self) -> Option<&EventId> {
        None
    }
}

struct CountingHandler {
    event_type: EventType,
    count: Arc<AtomicU64>,
}

#[async_trait]
impl JpaEventHandler<TestEvent> for CountingHandler {
    fn event_type(&self) -> EventType {
        self.event_type.clone()
    }

    async fn handle(&self, _event: &TestEvent) -> Result<(), JpaEventHandlerError> {
        self.count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

#[tokio::test]
async fn dispatches_to_every_handler_for_the_event_type() {
    let count = Arc::new(AtomicU64::new(0));
    let mut dispatcher = SimpleJpaEventDispatcher::new();
    for _ in 0..2 {
        dispatcher.register::<TestEvent, _>(Arc::new(CountingHandler {
            event_type: EventType::new("Created").expect("valid test event type"),
            count: Arc::clone(&count),
        }));
    }

    assert_eq!(
        dispatcher.all_types(),
        vec![EventType::new("Created").expect("valid test event type")]
    );
    dispatcher
        .dispatch_event(&TestEvent::new("Created"))
        .await
        .expect("registered event should dispatch");
    assert_eq!(count.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn ignores_unknown_event_types() {
    let dispatcher = SimpleJpaEventDispatcher::new();
    dispatcher
        .dispatch_event(&TestEvent::new("Unknown"))
        .await
        .expect("unknown event types should be ignored");
}
