//! Base implementation for Jackson-compatible commands using Serde.

use chrono::{DateTime, Utc};
use cqrs_4_rust_core::Command;
use ddd_4_rust_core::{Event, EventId, EventType};
use serde::{Deserialize, Serialize};

/// Command event metadata serialized with the Java Jackson field names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbstractCommand {
    #[serde(rename = "event-id")]
    event_id: EventId,
    #[serde(rename = "event-type")]
    event_type: EventType,
    #[serde(rename = "event-timestamp")]
    event_timestamp: DateTime<Utc>,
    #[serde(rename = "correlation-id", skip_serializing_if = "Option::is_none")]
    correlation_id: Option<EventId>,
    #[serde(rename = "causation-id", skip_serializing_if = "Option::is_none")]
    causation_id: Option<EventId>,
}

impl AbstractCommand {
    /// Creates a command with explicit event metadata.
    pub const fn new(
        event_id: EventId,
        event_type: EventType,
        event_timestamp: DateTime<Utc>,
        correlation_id: Option<EventId>,
        causation_id: Option<EventId>,
    ) -> Self {
        Self {
            event_id,
            event_type,
            event_timestamp,
            correlation_id,
            causation_id,
        }
    }

    /// Creates a command using the current timestamp and a new event ID.
    pub fn new_now(event_type: EventType) -> Self {
        Self::new(EventId::new(), event_type, Utc::now(), None, None)
    }
}

impl Event for AbstractCommand {
    fn event_id(&self) -> &EventId {
        &self.event_id
    }

    fn event_type(&self) -> &EventType {
        &self.event_type
    }

    fn event_timestamp(&self) -> &DateTime<Utc> {
        &self.event_timestamp
    }

    fn correlation_id(&self) -> Option<&EventId> {
        self.correlation_id.as_ref()
    }

    fn causation_id(&self) -> Option<&EventId> {
        self.causation_id.as_ref()
    }
}

impl Command for AbstractCommand {}
