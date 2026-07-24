//! JAXB-compatible result without a payload.

use crate::AbstractResult;
use cqrs_4_rust_core::ResultType;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// XML result containing only type, code, and message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleResult {
    base: AbstractResult,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "result")]
struct SimpleResultWire {
    #[serde(rename = "type", with = "crate::abstract_result::result_type_xml")]
    result_type: ResultType,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl SimpleResult {
    /// Creates a successful result.
    pub fn ok() -> Self {
        Self {
            base: AbstractResult::new(ResultType::Ok, None, None),
        }
    }
    /// Creates a warning result.
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            base: AbstractResult::new(ResultType::Warning, Some(code.into()), Some(message.into())),
        }
    }
    /// Creates an error result.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            base: AbstractResult::new(ResultType::Error, Some(code.into()), Some(message.into())),
        }
    }
    /// Returns the result type.
    pub const fn result_type(&self) -> ResultType {
        self.base.result_type()
    }
    /// Returns the optional code.
    pub fn code(&self) -> Option<&str> {
        self.base.code()
    }
    /// Returns the optional message.
    pub fn message(&self) -> Option<&str> {
        self.base.message()
    }
    /// Serializes the result to XML.
    ///
    /// # Errors
    ///
    /// Returns an error when result metadata cannot be represented as XML.
    pub fn to_xml(&self) -> Result<String, quick_xml::SeError> {
        quick_xml::se::to_string_with_root("result", self)
    }
    /// Deserializes a result from XML.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed XML or an unknown result type.
    pub fn from_xml(xml: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(xml)
    }
}

impl Serialize for SimpleResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SimpleResultWire {
            result_type: self.result_type(),
            code: self.code().map(str::to_owned),
            message: self.message().map(str::to_owned),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SimpleResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = SimpleResultWire::deserialize(deserializer)?;
        Ok(Self {
            base: AbstractResult::new(wire.result_type, wire.code, wire.message),
        })
    }
}
