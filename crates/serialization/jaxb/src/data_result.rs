//! JAXB-compatible result carrying an arbitrary XML element.

use cqrs_4_rust_core::ResultType;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// XML result containing optional typed payload data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "result")]
pub struct DataResult<Data> {
    #[serde(rename = "type", with = "crate::abstract_result::result_type_xml")]
    result_type: ResultType,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    data: Option<Data>,
    #[serde(skip)]
    data_element: Option<String>,
}

impl<Data> DataResult<Data> {
    /// Creates a result with optional payload metadata.
    pub fn new(
        result_type: ResultType,
        code: Option<String>,
        message: Option<String>,
        data: Option<Data>,
        data_element: Option<String>,
    ) -> Self {
        Self {
            result_type,
            code,
            message,
            data,
            data_element,
        }
    }

    /// Creates a successful result.
    pub fn ok(data: Option<Data>) -> Self {
        Self::new(ResultType::Ok, None, None, data, None)
    }

    /// Creates a successful result with its JAXB element name recorded.
    pub fn ok_with_element(data: Data, data_element: impl Into<String>) -> Self {
        Self::new(
            ResultType::Ok,
            None,
            None,
            Some(data),
            Some(data_element.into()),
        )
    }

    /// Creates an error result without payload data.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(
            ResultType::Error,
            Some(code.into()),
            Some(message.into()),
            None,
            None,
        )
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
    /// Returns the optional payload.
    pub const fn data(&self) -> Option<&Data> {
        self.data.as_ref()
    }
    /// Returns the recorded JAXB payload element name.
    pub fn data_element(&self) -> Option<&str> {
        self.data_element.as_deref()
    }

    /// Serializes this result to XML.
    ///
    /// # Errors
    ///
    /// Returns an error when result data cannot be represented as XML.
    pub fn to_xml(&self) -> Result<String, quick_xml::SeError>
    where
        Data: Serialize,
    {
        quick_xml::se::to_string_with_root("result", self)
    }

    /// Deserializes a result and records the expected payload element name.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed XML or an incompatible payload shape.
    pub fn from_xml(xml: &str, data_element: impl Into<String>) -> Result<Self, quick_xml::DeError>
    where
        Data: DeserializeOwned,
    {
        let mut result: Self = quick_xml::de::from_str(xml)?;
        result.data_element = result.data.as_ref().map(|_| data_element.into());
        Ok(result)
    }
}
