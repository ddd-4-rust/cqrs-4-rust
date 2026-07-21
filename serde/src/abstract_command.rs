//! Base implementation for commands with Jackson/serde serialization.
//!
//! 1:1 translation of `org.fuin.cqrs4j.jackson.AbstractCommand`.

use chrono::{DateTime, Utc};
use cqrs_4_rust_core::Command;
use ddd_4_rust_core::{Event, EventId, EventType};
use ddd_4_rust_serde::AbstractEvent;
use serde::{Deserialize, Serialize};

/// Base struct for commands with serde serialization.
///
/// Extends the DDD `AbstractEvent` to provide command-specific behavior.
///
/// Java: `AbstractCommand extends AbstractEvent implements Command`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractCommand {
    #[serde(flatten)]
    base: AbstractEvent,
}

impl AbstractCommand {
    pub fn new(
        event_id: EventId,
        event_type: EventType,
        event_timestamp: DateTime<Utc>,
        correlation_id: Option<EventId>,
        causation_id: Option<EventId>,
    ) -> Self {
        Self {
            base: AbstractEvent::new(event_id, event_type, event_timestamp, correlation_id, causation_id),
        }
    }

    pub fn new_now(event_type: EventType) -> Self {
        Self {
            base: AbstractEvent::new_now(event_type),
        }
    }
}

impl Event for AbstractCommand {
    fn event_id(&self) -> &EventId { self.base.event_id() }
    fn event_type(&self) -> &EventType { self.base.event_type() }
    fn event_timestamp(&self) -> &DateTime<Utc> { self.base.event_timestamp() }
    fn correlation_id(&self) -> Option<&EventId> { self.base.correlation_id() }
    fn causation_id(&self) -> Option<&EventId> { self.base.causation_id() }
}

impl Command for AbstractCommand {}
