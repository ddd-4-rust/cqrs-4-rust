//! Result of a request with type, code, message and optional data.
//!
//! 1:1 translation of `org.fuin.cqrs4j.core.Result<DATA>`.

use crate::result_type::ResultType;
use serde::{Deserialize, Serialize};

/// Result of a request. The type signals if the execution was successful or not.
///
/// Java: `Result<DATA>` interface
///
/// Methods: `getType()`, `getCode()`, `getMessage()`, `getData()`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CqrsResult<D = ()> {
    /// The result type.
    #[serde(rename = "type")]
    pub result_type: ResultType,
    /// The result code (optional, for WARNING and ERROR).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The result message (optional, for WARNING and ERROR).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional data payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<D>,
}

impl<D> CqrsResult<D> {
    /// Creates a successful result without data.
    ///
    /// Java: `SimpleResult.ok()`
    pub fn ok() -> Self {
        Self {
            result_type: ResultType::Ok,
            code: None,
            message: None,
            data: None,
        }
    }

    /// Creates a successful result with data.
    ///
    /// Java: `DataResult.ok(D data)`
    pub fn ok_with_data(data: D) -> Self {
        Self {
            result_type: ResultType::Ok,
            code: None,
            message: None,
            data: Some(data),
        }
    }

    /// Creates a warning result.
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            result_type: ResultType::Warning,
            code: Some(code.into()),
            message: Some(message.into()),
            data: None,
        }
    }

    /// Creates a warning result with data.
    pub fn warning_with_data(code: impl Into<String>, message: impl Into<String>, data: D) -> Self {
        Self {
            result_type: ResultType::Warning,
            code: Some(code.into()),
            message: Some(message.into()),
            data: Some(data),
        }
    }

    /// Creates an error result.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            result_type: ResultType::Error,
            code: Some(code.into()),
            message: Some(message.into()),
            data: None,
        }
    }

    /// Creates an error result with data.
    pub fn error_with_data(code: impl Into<String>, message: impl Into<String>, data: D) -> Self {
        Self {
            result_type: ResultType::Error,
            code: Some(code.into()),
            message: Some(message.into()),
            data: Some(data),
        }
    }

    /// Returns the result type.
    pub fn result_type(&self) -> ResultType {
        self.result_type
    }

    /// Returns the result code.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns the result message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns the result data.
    pub fn data(&self) -> Option<&D> {
        self.data.as_ref()
    }

    /// Returns true if the result is OK.
    pub fn is_ok(&self) -> bool {
        self.result_type == ResultType::Ok
    }

    /// Returns true if the result is a warning.
    pub fn is_warning(&self) -> bool {
        self.result_type == ResultType::Warning
    }

    /// Returns true if the result is an error.
    pub fn is_error(&self) -> bool {
        self.result_type == ResultType::Error
    }
}

impl<D> Default for CqrsResult<D> {
    fn default() -> Self {
        Self::ok()
    }
}
