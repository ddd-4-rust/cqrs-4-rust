//! JSON-B-compatible result carrying an optional payload.

use cqrs_4_rust_core::ResultType;

/// Result with optional typed data and dynamic JSON element metadata.
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
            data_class: None,
            data_element: None,
        }
    }

    /// Creates a result with all JSON-B payload metadata.
    pub const fn with_metadata(
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

    /// Creates a successful result without a payload element.
    pub const fn ok(data: Option<Data>) -> Self {
        Self::new(ResultType::Ok, None, None, data)
    }

    /// Creates a successful result with JSON-B dynamic element metadata.
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

    /// Creates an error result without a payload.
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

    /// Returns the optional code.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns the optional message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns the optional payload.
    pub const fn data(&self) -> Option<&Data> {
        self.data.as_ref()
    }

    /// Returns the serialized Rust type name for the payload.
    pub fn data_class(&self) -> Option<&str> {
        self.data_class.as_deref()
    }

    /// Returns the dynamic JSON field containing the payload.
    pub fn data_element(&self) -> Option<&str> {
        self.data_element.as_deref()
    }
}
