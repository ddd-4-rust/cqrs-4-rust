//! Persisted person read-model entity.

use crate::generated::{PersonId, PersonName};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use uuid::Uuid;

/// Person projection stored by the view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonEntity {
    /// Stable person identifier.
    pub id: Uuid,
    /// Display name.
    pub name: String,
}

impl PersonEntity {
    /// Creates an entity from validated domain values.
    pub fn new(id: PersonId, name: &PersonName) -> Self {
        Self {
            id: id.as_uuid(),
            name: name.as_str().to_owned(),
        }
    }
}

impl PartialEq for PersonEntity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PersonEntity {}

impl PartialOrd for PersonEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PersonEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name).then(self.id.cmp(&other.id))
    }
}
