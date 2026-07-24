//! Generated person-created event.

use crate::generated::{PersonId, PersonName};
use crate::model::GenPersonCreatedEvent;
use serde::{Deserialize, Serialize};

/// Event emitted when a person is created.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonCreatedEvent {
    id: PersonId,
    name: PersonName,
}

impl PersonCreatedEvent {
    /// Stable event type used by the event store.
    pub const TYPE: &'static str = "PersonCreatedEvent";

    /// Creates an event.
    pub const fn new(id: PersonId, name: PersonName) -> Self {
        Self { id, name }
    }

    /// Returns the person identifier.
    pub const fn id(&self) -> PersonId {
        self.id
    }

    /// Returns the person name.
    pub const fn name(&self) -> &PersonName {
        &self.name
    }
}

impl GenPersonCreatedEvent for PersonCreatedEvent {
    fn person_id(&self) -> PersonId {
        self.id
    }

    fn person_name(&self) -> &PersonName {
        &self.name
    }
}
