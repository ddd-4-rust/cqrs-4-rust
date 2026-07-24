//! JSON-B-compatible result without a payload.

use cqrs_4_rust_core::ResultType;
use serde::{Deserialize, Serialize};

/// Result containing only type, code, and message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimpleResult {
    #[serde(rename = "type")]
    result_type: ResultType,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl SimpleResult {
    /// Creates a successful result.
    pub const fn ok() -> Self {
        Self {
            result_type: ResultType::Ok,
            code: None,
            message: None,
        }
    }

    /// Creates a warning result.
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            result_type: ResultType::Warning,
            code: Some(code.into()),
            message: Some(message.into()),
        }
    }

    /// Creates an error result.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            result_type: ResultType::Error,
            code: Some(code.into()),
            message: Some(message.into()),
        }
    }

    /// Returns the result type.
    pub const fn result_type(&self) -> ResultType {
        self.result_type
    }

    /// Returns the optional result code.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns the optional result message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
