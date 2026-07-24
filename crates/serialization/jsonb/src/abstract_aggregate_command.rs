//! Base JSON-B-compatible aggregate command implementation.

use crate::AbstractCommand;
use chrono::{DateTime, Utc};
use cqrs_4_rust_core::{AggregateCommand, Command};
use ddd_4_rust_core::{
    AggregateRootId, AggregateVersion, DomainEvent, EntityId, EntityIdPath, Event, EventId,
    EventType,
};
use serde::{Serialize, Serializer};

/// Aggregate command retaining both event metadata and aggregate identity.
#[derive(Debug, Clone, Serialize)]
pub struct AbstractAggregateCommand<RootId> {
    #[serde(flatten)]
    base: AbstractCommand,
    #[serde(rename = "entity-id-path", serialize_with = "serialize_entity_id_path")]
    entity_id_path: EntityIdPath,
    #[serde(rename = "aggregate-version", skip_serializing_if = "Option::is_none")]
    aggregate_version: Option<AggregateVersion>,
    #[serde(skip)]
    aggregate_root_id: RootId,
}

impl<RootId> AbstractAggregateCommand<RootId> {
    /// Creates an aggregate command with explicit event metadata.
    #[allow(clippy::too_many_arguments)]
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
            base: AbstractCommand::new(
                event_id,
                event_type,
                event_timestamp,
                correlation_id,
                causation_id,
            ),
            entity_id_path,
            aggregate_version,
            aggregate_root_id,
        }
    }

    /// Creates an aggregate command with current event metadata.
    pub fn new_now(
        event_type: EventType,
        entity_id_path: EntityIdPath,
        aggregate_version: Option<AggregateVersion>,
        aggregate_root_id: RootId,
    ) -> Self {
        Self {
            base: AbstractCommand::new_now(event_type),
            entity_id_path,
            aggregate_version,
            aggregate_root_id,
        }
    }

    /// Returns the concrete aggregate root identifier.
    pub const fn aggregate_root_id(&self) -> &RootId {
        &self.aggregate_root_id
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
        &self.entity_id_path
    }

    fn entity_id(&self) -> &dyn EntityId {
        self.entity_id_path.last().as_ref()
    }

    fn aggregate_version(&self) -> Option<&AggregateVersion> {
        self.aggregate_version.as_ref()
    }
}

impl<RootId: AggregateRootId> AggregateCommand<RootId, dyn EntityId>
    for AbstractAggregateCommand<RootId>
{
    fn aggregate_root_id(&self) -> &RootId {
        &self.aggregate_root_id
    }
}

fn serialize_entity_id_path<Output>(
    path: &EntityIdPath,
    serializer: Output,
) -> Result<Output::Ok, Output::Error>
where
    Output: Serializer,
{
    serializer.serialize_str(&path.as_base_type())
}
