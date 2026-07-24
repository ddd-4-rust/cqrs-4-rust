//! Generator contract for person identifiers.

use uuid::Uuid;

/// Behavior required from generated person identifiers.
pub trait GenPersonId {
    /// Returns the underlying UUID.
    fn value(&self) -> Uuid;
}
