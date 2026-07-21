//! Base implementation for aggregate commands.
//!
//! 1:1 translation of `org.fuin.cqrs4j.jackson.AbstractAggregateCommand`.

use chrono::{DateTime, Utc};
use cqrs_4_rust_core::{AggregateCommand, Command};
use ddd_4_rust_core::{
    AggregateRootId, AggregateVersion, DomainEvent, EntityId, EntityIdPath,
    Event, EventId, EventType,
};
use ddd_4_rust_serde::AbstractDomainEvent;
use serde::{Deserialize, Serialize};

/// Base struct for aggregate commands with serde serialization.
///
/// Java: `AbstractAggregateCommand<ROOT_ID, ENTITY_ID>
///         extends AbstractCommand implements AggregateCommand<ROOT_ID, ENTITY_ID>`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractAggregateCommand {
    #[serde(flatten)]
    base: AbstractDomainEvent,
    /// The aggregate root ID (first ID in the entity ID path).
    aggregate_root_id: String,
}

impl AbstractAggregateCommand {
    pub fn new(
        event_id: EventId,
        event_type: EventType,
        event_timestamp: DateTime<Utc>,
        correlation_id: Option<EventId>,
        causation_id: Option<EventId>,
        entity_id_path: EntityIdPath,
        aggregate_version: Option<AggregateVersion>,
        aggregate_root_id: String,
    ) -> Self {
        Self {
            base: AbstractDomainEvent::new(
                event_id, event_type, event_timestamp,
                correlation_id, causation_id,
                entity_id_path, aggregate_version,
            ),
            aggregate_root_id,
        }
    }

    pub fn new_now(
        event_type: EventType,
        entity_id_path: EntityIdPath,
        aggregate_version: Option<AggregateVersion>,
        aggregate_root_id: String,
    ) -> Self {
        Self {
            base: AbstractDomainEvent::new_now(event_type, entity_id_path, aggregate_version),
            aggregate_root_id,
        }
    }

    pub fn aggregate_root_id(&self) -> &str {
        &self.aggregate_root_id
    }
}

impl Event for AbstractAggregateCommand {
    fn event_id(&self) -> &EventId { self.base.event_id() }
    fn event_type(&self) -> &EventType { self.base.event_type() }
    fn event_timestamp(&self) -> &DateTime<Utc> { self.base.event_timestamp() }
    fn correlation_id(&self) -> Option<&EventId> { self.base.correlation_id() }
    fn causation_id(&self) -> Option<&EventId> { self.base.causation_id() }
}

impl Command for AbstractAggregateCommand {}

impl DomainEvent<dyn EntityId> for AbstractAggregateCommand {
    fn entity_id_path(&self) -> &EntityIdPath { self.base.entity_id_path() }
    fn entity_id(&self) -> &dyn EntityId { self.base.entity_id() }
    fn aggregate_version(&self) -> Option<&AggregateVersion> { self.base.aggregate_version() }
}

impl AggregateCommand<dyn AggregateRootId, dyn EntityId> for AbstractAggregateCommand {
    fn aggregate_root_id(&self) -> &dyn AggregateRootId {
        // The first entity in the path is the aggregate root ID
        unimplemented!("Use concrete types for AggregateRootId instead of dyn")
    }
}
