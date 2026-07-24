//! Quarkus view-manager projection loop checks for Actix.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cqrs_4_rust_actix::{ActixJpaViewManager, QryProjectionPositionRepository};
use cqrs_4_rust_core::{JpaView, JpaViewError, View};
use cqrs_4_rust_esc::{MemoryProjectionAdmin, ProjectionService, RegistryEventDecoder};
use ddd_4_rust_core::{Event, EventId, EventType};
use ddd_4_rust_esc::{CommonEvent, EventStore, StreamId};
use ddd_4_rust_test::MemoryEventStore;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

struct CountingEvent {
    event_id: EventId,
    event_type: EventType,
    timestamp: DateTime<Utc>,
}

impl CountingEvent {
    fn new() -> Self {
        Self {
            event_id: EventId::new(),
            event_type: EventType::new("PersonCreated").expect("valid event type"),
            timestamp: Utc::now(),
        }
    }
}

impl Event for CountingEvent {
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

struct CountingView {
    handled: Arc<AtomicU64>,
    event_type: EventType,
}

impl View for CountingView {
    #[allow(clippy::unnecessary_literal_bound)]
    fn name(&self) -> &str {
        "persons-view"
    }

    fn event_types(&self) -> Vec<EventType> {
        vec![self.event_type.clone()]
    }
}

#[async_trait]
impl JpaView for CountingView {
    #[allow(clippy::unnecessary_literal_bound)]
    fn cron(&self) -> &str {
        "0/1 * * * * *"
    }

    async fn handle_events(&self, events: &[Box<dyn Event>]) -> Result<(), JpaViewError> {
        self.handled
            .fetch_add(u64::try_from(events.len()).unwrap_or(0), Ordering::SeqCst);
        Ok(())
    }
}

fn common_event(event_number: i64) -> CommonEvent {
    CommonEvent {
        event_id: Uuid::new_v4(),
        event_type: "PersonCreated".to_owned(),
        data: b"payload".to_vec(),
        metadata: None,
        created: Utc::now(),
        event_number,
    }
}

fn build_manager(
    handled: Arc<AtomicU64>,
    store: Arc<MemoryEventStore>,
    projection_service: Arc<QryProjectionPositionRepository>,
) -> ActixJpaViewManager {
    let mut decoder = RegistryEventDecoder::new();
    decoder.register_str("PersonCreated", |_common| {
        Ok(Box::new(CountingEvent::new()) as Box<dyn Event>)
    });

    let view: Arc<dyn JpaView> = Arc::new(CountingView {
        handled,
        event_type: EventType::new("PersonCreated").expect("valid event type"),
    });

    ActixJpaViewManager::new(
        vec![view],
        store,
        Arc::new(MemoryProjectionAdmin::new()),
        projection_service,
        Arc::new(decoder),
    )
}

#[tokio::test]
async fn starts_and_stops_with_no_registered_views() {
    let store: Arc<dyn EventStore> = Arc::new(MemoryEventStore::new());
    let manager = ActixJpaViewManager::new(
        Vec::new(),
        store,
        Arc::new(MemoryProjectionAdmin::new()),
        Arc::new(QryProjectionPositionRepository::new()),
        Arc::new(RegistryEventDecoder::new()),
    );
    assert!(manager.views().is_empty());
    manager.start().await.expect("startup should succeed");
    manager.stop().await.expect("shutdown should succeed");
}

#[tokio::test]
async fn tick_applies_events_advances_position_and_is_idempotent() {
    let handled = Arc::new(AtomicU64::new(0));
    let store = Arc::new(MemoryEventStore::new());
    let projection_service = Arc::new(QryProjectionPositionRepository::new());

    store
        .append_to_stream(
            &StreamId::new("Person-1"),
            -1,
            vec![common_event(0), common_event(1)],
        )
        .await
        .expect("append should succeed");

    let manager = build_manager(
        Arc::clone(&handled),
        Arc::clone(&store),
        Arc::clone(&projection_service),
    );

    let applied = manager.tick_all().await.expect("first tick");
    assert_eq!(applied, 2);
    assert_eq!(handled.load(Ordering::SeqCst), 2);

    let stream_id = manager.managed_views()[0].stream_id.as_str().to_owned();
    let position = projection_service
        .read_projection_position(&stream_id)
        .await
        .expect("read position")
        .expect("position exists");
    assert_eq!(position.next_position, 2);

    let applied_again = manager.tick_all().await.expect("second tick");
    assert_eq!(applied_again, 0);
    assert_eq!(handled.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn start_and_stop_round_trip_with_views() {
    let handled = Arc::new(AtomicU64::new(0));
    let store = Arc::new(MemoryEventStore::new());
    let manager = build_manager(
        handled,
        store,
        Arc::new(QryProjectionPositionRepository::new()),
    );
    manager.start().await.expect("start");
    manager.stop().await.expect("stop");
}
