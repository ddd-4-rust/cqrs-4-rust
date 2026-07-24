//! Generated strongly typed person identifier.

use crate::model::GenPersonId;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

/// UUID identifier for a person.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PersonId(Uuid);

impl PersonId {
    /// Creates a random identifier.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates an identifier from a UUID.
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    /// Returns the underlying UUID.
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for PersonId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for PersonId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl FromStr for PersonId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value).map(Self)
    }
}

impl GenPersonId for PersonId {
    fn value(&self) -> Uuid {
        self.0
    }
}
