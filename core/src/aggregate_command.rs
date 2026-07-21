//! Common behavior shared by all commands related to an aggregate.
//!
//! 1:1 translation of `org.fuin.cqrs4j.core.AggregateCommand`.

use ddd_4_rust_core::aggregate_root_id::AggregateRootId;
use ddd_4_rust_core::domain_event::DomainEvent;
use ddd_4_rust_core::entity_id::EntityId;
use crate::command::Command;

/// Common behavior shared by all commands related to an aggregate.
///
/// Java: `AggregateCommand<ROOT_ID extends AggregateRootId, ENTITY_ID extends EntityId>
///         extends Command, DomainEvent<ENTITY_ID>`
pub trait AggregateCommand<RootId: AggregateRootId + ?Sized, EntityIdType: EntityId + ?Sized>:
    Command + DomainEvent<EntityIdType>
{
    /// Returns the identifier of the aggregate root this command targets.
    ///
    /// Java: `@NotNull getAggregateRootId()`
    fn aggregate_root_id(&self) -> &RootId;
}
