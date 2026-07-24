//! XML-serializable base implementation for commands.

use chrono::{DateTime, Utc};
use cqrs_4_rust_core::Command;
use ddd_4_rust_core::{Event, EventId, EventType};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// JAXB-compatible command metadata.
#[derive(Debug, Clone)]
pub struct AbstractCommand {
    event_id: EventId,
    event_type: EventType,
    event_timestamp: DateTime<Utc>,
    correlation_id: Option<EventId>,
    causation_id: Option<EventId>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "command")]
struct CommandWire {
    #[serde(rename = "event-id")]
    event_id: String,
    #[serde(rename = "event-type")]
    event_type: String,
    #[serde(rename = "event-timestamp")]
    event_timestamp: String,
    #[serde(rename = "correlation-id", skip_serializing_if = "Option::is_none")]
    correlation_id: Option<String>,
    #[serde(rename = "causation-id", skip_serializing_if = "Option::is_none")]
    causation_id: Option<String>,
}

impl AbstractCommand {
    /// Creates command metadata with explicit event values.
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

    /// Creates command metadata using a fresh ID and current timestamp.
    pub fn new_now(event_type: EventType) -> Self {
        Self::new(EventId::new(), event_type, Utc::now(), None, None)
    }

    /// Serializes this command to XML.
    ///
    /// # Errors
    ///
    /// Returns an error when command metadata cannot be represented as XML.
    pub fn to_xml(&self) -> Result<String, quick_xml::SeError> {
        quick_xml::se::to_string_with_root("command", self)
    }

    /// Deserializes command metadata from XML.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed XML or invalid event metadata.
    pub fn from_xml(xml: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(xml)
    }
}

impl Serialize for AbstractCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        CommandWire {
            event_id: self.event_id.as_string(),
            event_type: self.event_type.as_str().to_owned(),
            event_timestamp: self.event_timestamp.to_rfc3339(),
            correlation_id: self.correlation_id.as_ref().map(EventId::as_string),
            causation_id: self.causation_id.as_ref().map(EventId::as_string),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AbstractCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = CommandWire::deserialize(deserializer)?;
        let event_id = EventId::value_of(&wire.event_id)
            .ok_or_else(|| D::Error::custom("invalid event-id UUID"))?;
        let event_type =
            EventType::new(wire.event_type).map_err(|error| D::Error::custom(error.to_string()))?;
        let event_timestamp = DateTime::parse_from_rfc3339(&wire.event_timestamp)
            .map_err(D::Error::custom)?
            .with_timezone(&Utc);
        let correlation_id = wire
            .correlation_id
            .map(|value| {
                EventId::value_of(&value)
                    .ok_or_else(|| D::Error::custom("invalid correlation-id UUID"))
            })
            .transpose()?;
        let causation_id = wire
            .causation_id
            .map(|value| {
                EventId::value_of(&value)
                    .ok_or_else(|| D::Error::custom("invalid causation-id UUID"))
            })
            .transpose()?;
        Ok(Self::new(
            event_id,
            event_type,
            event_timestamp,
            correlation_id,
            causation_id,
        ))
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
