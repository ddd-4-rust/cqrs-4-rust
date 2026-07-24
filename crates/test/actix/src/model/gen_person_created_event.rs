//! Generator contract for person-created events.

use crate::generated::{PersonId, PersonName};

/// Behavior required from generated person-created events.
pub trait GenPersonCreatedEvent {
    /// Returns the created person's identifier.
    fn person_id(&self) -> PersonId;

    /// Returns the created person's name.
    fn person_name(&self) -> &PersonName;
}
