//! Adapter for the Java JSON-B `DataResult` wire contract.

use crate::DataResult;
use cqrs_4_rust_core::ResultType;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use thiserror::Error;

/// Failures while adapting a [`DataResult`] to or from JSON.
#[derive(Debug, Error)]
pub enum DataResultJsonbAdapterError {
    /// A mandatory property is absent.
    #[error("missing required JSON-B property '{0}'")]
    MissingProperty(String),
    /// The root value is not an object.
    #[error("a JSON-B data result must be a JSON object")]
    ExpectedObject,
    /// Serde failed to encode or decode the typed payload.
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

/// Converts [`DataResult`] values using the Java JSON-B dynamic field layout.
#[derive(Debug, Clone, Copy, Default)]
pub struct DataResultJsonbAdapter;

impl DataResultJsonbAdapter {
    /// Converts a typed result into a JSON object value.
    ///
    /// # Errors
    ///
    /// Returns an error when payload serialization fails or payload metadata is
    /// incomplete.
    pub fn adapt_to_json<Data: Serialize>(
        result: &DataResult<Data>,
    ) -> Result<Value, DataResultJsonbAdapterError> {
        let mut object = Map::new();
        object.insert(
            "type".to_owned(),
            serde_json::to_value(result.result_type())?,
        );
        if let Some(code) = result.code() {
            object.insert("code".to_owned(), Value::String(code.to_owned()));
        }
        if let Some(message) = result.message() {
            object.insert("message".to_owned(), Value::String(message.to_owned()));
        }
        if let Some(data) = result.data() {
            let data_element = result.data_element().ok_or_else(|| {
                DataResultJsonbAdapterError::MissingProperty("data-element".to_owned())
            })?;
            let data_class = result.data_class().unwrap_or(std::any::type_name::<Data>());
            object.insert(
                "data-class".to_owned(),
                Value::String(data_class.to_owned()),
            );
            object.insert(
                "data-element".to_owned(),
                Value::String(data_element.to_owned()),
            );
            object.insert(data_element.to_owned(), serde_json::to_value(data)?);
        }
        Ok(Value::Object(object))
    }

    /// Restores a typed result from a JSON object value.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed metadata, a missing dynamic payload field,
    /// or a payload that cannot be deserialized into `Data`.
    pub fn adapt_from_json<Data: DeserializeOwned>(
        value: Value,
    ) -> Result<DataResult<Data>, DataResultJsonbAdapterError> {
        let Value::Object(mut object) = value else {
            return Err(DataResultJsonbAdapterError::ExpectedObject);
        };
        let result_type = take_required::<ResultType>(&mut object, "type")?;
        let code = take_optional::<String>(&mut object, "code")?;
        let message = take_optional::<String>(&mut object, "message")?;
        let data_class = take_optional::<String>(&mut object, "data-class")?;
        if let Some(data_class) = data_class {
            let data_element = take_required::<String>(&mut object, "data-element")?;
            let payload = object.remove(&data_element).ok_or_else(|| {
                DataResultJsonbAdapterError::MissingProperty(data_element.clone())
            })?;
            let data = serde_json::from_value(payload)?;
            Ok(DataResult::with_metadata(
                result_type,
                code,
                message,
                Some(data),
                Some(data_class),
                Some(data_element),
            ))
        } else {
            Ok(DataResult::new(result_type, code, message, None))
        }
    }
}

fn take_required<Data: DeserializeOwned>(
    object: &mut Map<String, Value>,
    name: &str,
) -> Result<Data, DataResultJsonbAdapterError> {
    let value = object
        .remove(name)
        .ok_or_else(|| DataResultJsonbAdapterError::MissingProperty(name.to_owned()))?;
    Ok(serde_json::from_value(value)?)
}

fn take_optional<Data: DeserializeOwned>(
    object: &mut Map<String, Value>,
    name: &str,
) -> Result<Option<Data>, DataResultJsonbAdapterError> {
    object
        .remove(name)
        .map(serde_json::from_value)
        .transpose()
        .map_err(Into::into)
}
