//! Common JAXB-compatible result metadata.

use cqrs_4_rust_core::ResultType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Result type, code, and message shared by XML result variants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbstractResult<Data = ()> {
    #[serde(rename = "type", with = "result_type_xml")]
    result_type: ResultType,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip)]
    data_type: PhantomData<fn() -> Data>,
}

pub(crate) mod result_type_xml {
    use cqrs_4_rust_core::ResultType;
    use serde::de::Error as _;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(
        clippy::trivially_copy_pass_by_ref,
        reason = "Serde with-modules require this serializer signature"
    )]
    pub(crate) fn serialize<S>(value: &ResultType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match value {
            ResultType::Ok => "OK",
            ResultType::Warning => "WARNING",
            ResultType::Error => "ERROR",
        };
        serializer.serialize_str(value)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<ResultType, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "OK" => Ok(ResultType::Ok),
            "WARNING" => Ok(ResultType::Warning),
            "ERROR" => Ok(ResultType::Error),
            value => Err(D::Error::custom(format!("unknown result type '{value}'"))),
        }
    }
}

impl<Data> AbstractResult<Data> {
    /// Creates result metadata.
    pub fn new(result_type: ResultType, code: Option<String>, message: Option<String>) -> Self {
        Self {
            result_type,
            code,
            message,
            data_type: PhantomData,
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
