//! Generated validated person name.

use crate::model::GenPersonName;
use serde::{Deserialize, Serialize};

/// Validated person name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PersonName(String);

impl PersonName {
    /// Maximum generated name length.
    pub const MAX_LENGTH: usize = 100;

    /// Validates and creates a name.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is empty or exceeds [`Self::MAX_LENGTH`].
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("person name must not be empty".to_owned());
        }
        if value.len() > Self::MAX_LENGTH {
            return Err(format!(
                "person name exceeds {} characters",
                Self::MAX_LENGTH
            ));
        }
        Ok(Self(value))
    }

    /// Returns the underlying name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl GenPersonName for PersonName {
    fn value(&self) -> &str {
        self.as_str()
    }
}
