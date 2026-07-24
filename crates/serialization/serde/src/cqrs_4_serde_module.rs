//! Entry point for the Serde CQRS wire-format integration.

use crate::DataResult;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Configures the cqrs-4-rust Serde integration.
#[derive(Debug, Clone, Copy, Default)]
pub struct Cqrs4SerdeModule;

impl Cqrs4SerdeModule {
    /// Stable Serde module name.
    pub const NAME: &'static str = "Cqrs4SerdeModule";

    /// Module version inherited from the workspace release line.
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// Serializes a typed data result using the Java-compatible dynamic wire format.
    ///
    /// # Errors
    ///
    /// Returns an error when result metadata or payload serialization is invalid.
    pub fn to_json<Data: Serialize>(result: &DataResult<Data>) -> serde_json::Result<String> {
        serde_json::to_string(result)
    }

    /// Deserializes a typed data result using the Java-compatible dynamic wire format.
    ///
    /// # Errors
    ///
    /// Returns an error when required metadata is missing or payload decoding fails.
    pub fn from_json<Data: DeserializeOwned>(json: &str) -> serde_json::Result<DataResult<Data>> {
        serde_json::from_str(json)
    }
}
