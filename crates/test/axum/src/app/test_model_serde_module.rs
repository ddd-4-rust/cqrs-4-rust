//! Serde replacement for the Spring test model Jackson module.

use serde::Serialize;
use serde::de::DeserializeOwned;

/// Serializes and deserializes generated test-model values.
#[derive(Debug, Clone, Copy, Default)]
pub struct TestModelSerdeModule;

impl TestModelSerdeModule {
    /// Serializes a test-model value.
    ///
    /// # Errors
    ///
    /// Returns a Serde error when the value cannot be encoded.
    pub fn to_json<Value: Serialize>(value: &Value) -> serde_json::Result<String> {
        serde_json::to_string(value)
    }

    /// Deserializes a test-model value.
    ///
    /// # Errors
    ///
    /// Returns a Serde error when the JSON does not match `Value`.
    pub fn from_json<Value: DeserializeOwned>(json: &str) -> serde_json::Result<Value> {
        serde_json::from_str(json)
    }
}
