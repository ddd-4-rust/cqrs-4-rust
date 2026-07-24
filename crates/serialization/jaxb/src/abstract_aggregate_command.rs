//! XML-serializable base implementation for aggregate commands.

use chrono::{DateTime, Utc};
use cqrs_4_rust_core::{AggregateCommand, Command};
use ddd_4_rust_core::{
    AggregateRootId, AggregateVersion, DomainEvent, EntityId, EntityIdPath, Event, EventId,
    EventType,
};
use ddd_4_rust_serde::AbstractDomainEvent;
use serde::Serialize;

/// JAXB-compatible metadata for a command targeting an aggregate.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "aggregate-command")]
pub struct AbstractAggregateCommand<RootId> {
    #[serde(flatten)]
    base: AbstractDomainEvent,
    #[serde(skip)]
    aggregate_root_id: RootId,
}

impl<RootId> AbstractAggregateCommand<RootId> {
    /// Creates aggregate command metadata with explicit event values.
    pub fn new(
        event_id: EventId,
        event_type: EventType,
        event_timestamp: DateTime<Utc>,
        correlation_id: Option<EventId>,
        causation_id: Option<EventId>,
        entity_id_path: EntityIdPath,
        aggregate_version: Option<AggregateVersion>,
        aggregate_root_id: RootId,
    ) -> Self {
        Self {
            base: AbstractDomainEvent::new(
                event_id,
                event_type,
                event_timestamp,
                correlation_id,
                causation_id,
                entity_id_path,
                aggregate_version,
            ),
            aggregate_root_id,
        }
    }

    /// Creates aggregate command metadata using a fresh ID and timestamp.
    pub fn new_now(
        event_type: EventType,
        entity_id_path: EntityIdPath,
        aggregate_version: Option<AggregateVersion>,
        aggregate_root_id: RootId,
    ) -> Self {
        Self {
            base: AbstractDomainEvent::new_now(event_type, entity_id_path, aggregate_version),
            aggregate_root_id,
        }
    }

    /// Serializes this aggregate command to XML.
    ///
    /// # Errors
    ///
    /// Returns an error when command metadata cannot be represented as XML.
    pub fn to_xml(&self) -> Result<String, quick_xml::SeError>
    where
        RootId: Serialize,
    {
        quick_xml::se::to_string_with_root("aggregate-command", self)
    }
}

impl<RootId: AggregateRootId> Event for AbstractAggregateCommand<RootId> {
    fn event_id(&self) -> &EventId {
        self.base.event_id()
    }
    fn event_type(&self) -> &EventType {
        self.base.event_type()
    }
    fn event_timestamp(&self) -> &DateTime<Utc> {
        self.base.event_timestamp()
    }
    fn correlation_id(&self) -> Option<&EventId> {
        self.base.correlation_id()
    }
    fn causation_id(&self) -> Option<&EventId> {
        self.base.causation_id()
    }
}

impl<RootId: AggregateRootId> Command for AbstractAggregateCommand<RootId> {}

impl<RootId: AggregateRootId> DomainEvent<dyn EntityId> for AbstractAggregateCommand<RootId> {
    fn entity_id_path(&self) -> &EntityIdPath {
        self.base.entity_id_path()
    }
    fn entity_id(&self) -> &dyn EntityId {
        self.base.entity_id()
    }
    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        self.base.aggregate_version()
    }
}

impl<RootId: AggregateRootId> AggregateCommand<RootId, dyn EntityId>
    for AbstractAggregateCommand<RootId>
{
    fn aggregate_root_id(&self) -> &RootId {
        &self.aggregate_root_id
    }
}
