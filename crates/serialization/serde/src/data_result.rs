//! Result carrying an optional Serde payload.

use crate::{data_result_deserializer, data_result_serializer};
use cqrs_4_rust_core::ResultType;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Result carrying optional typed data and its dynamic JSON element name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataResult<Data> {
    result_type: ResultType,
    code: Option<String>,
    message: Option<String>,
    data: Option<Data>,
    data_class: Option<String>,
    data_element: Option<String>,
}

impl<Data> DataResult<Data> {
    /// Creates a result without dynamic payload metadata.
    pub fn new(
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
            data_class: None,
            data_element: None,
        }
    }

    /// Creates a result with the metadata required by the compatible wire format.
    pub fn with_metadata(
        result_type: ResultType,
        code: Option<String>,
        message: Option<String>,
        data: Option<Data>,
        data_class: Option<String>,
        data_element: Option<String>,
    ) -> Self {
        Self {
            result_type,
            code,
            message,
            data,
            data_class,
            data_element,
        }
    }

    /// Creates a successful result.
    pub fn ok(data: Option<Data>) -> Self {
        Self::new(ResultType::Ok, None, None, data)
    }

    /// Creates a successful result with a dynamic JSON element name.
    pub fn ok_with_element(data: Data, data_element: impl Into<String>) -> Self {
        Self::with_metadata(
            ResultType::Ok,
            None,
            None,
            Some(data),
            Some(std::any::type_name::<Data>().to_owned()),
            Some(data_element.into()),
        )
    }

    /// Creates an error result without data.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(
            ResultType::Error,
            Some(code.into()),
            Some(message.into()),
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

    /// Returns the serialized payload type name.
    pub fn data_class(&self) -> Option<&str> {
        self.data_class.as_deref()
    }

    /// Returns the JSON field containing the payload.
    pub fn data_element(&self) -> Option<&str> {
        self.data_element.as_deref()
    }
}

impl<Data: Serialize> Serialize for DataResult<Data> {
    fn serialize<Output>(&self, serializer: Output) -> Result<Output::Ok, Output::Error>
    where
        Output: Serializer,
    {
        data_result_serializer::serialize(self, serializer)
    }
}

impl<'de, Data> Deserialize<'de> for DataResult<Data>
where
    Data: DeserializeOwned,
{
    fn deserialize<Input>(deserializer: Input) -> Result<Self, Input::Error>
    where
        Input: Deserializer<'de>,
    {
        data_result_deserializer::deserialize(deserializer)
    }
}

pub(crate) struct DataResultParts<Data> {
    pub(crate) result_type: ResultType,
    pub(crate) code: Option<String>,
    pub(crate) message: Option<String>,
    pub(crate) data: Option<Data>,
    pub(crate) data_class: Option<String>,
    pub(crate) data_element: Option<String>,
}

impl<Data> From<DataResultParts<Data>> for DataResult<Data> {
    fn from(parts: DataResultParts<Data>) -> Self {
        Self::with_metadata(
            parts.result_type,
            parts.code,
            parts.message,
            parts.data,
            parts.data_class,
            parts.data_element,
        )
    }
}
