//! Common JSON-B-compatible result metadata.

use cqrs_4_rust_core::ResultType;
use serde::{Deserialize, Serialize};

/// Result metadata shared by simple and data-carrying results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbstractResult<Data = ()> {
    #[serde(rename = "type")]
    result_type: ResultType,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Data>,
}

impl<Data> AbstractResult<Data> {
    /// Creates result metadata with optional data.
    pub const fn new(
        result_type: ResultType,
        code: Option<String>,
        message: Option<String>,
        data: Option<Data>,
    ) -> Self {
        Self {
            result_type,
            code,
            message,
            data,
        }
    }

    /// Returns the result type.
    pub const fn result_type(&self) -> ResultType {
        self.result_type
    }

    /// Returns the optional code.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns the optional message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns the optional data.
    pub const fn data(&self) -> Option<&Data> {
        self.data.as_ref()
    }
}
