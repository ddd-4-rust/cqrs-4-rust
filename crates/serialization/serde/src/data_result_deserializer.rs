//! Serde deserializer for the Java-compatible `DataResult` wire format.

use crate::data_result::{DataResult, DataResultParts};
use cqrs_4_rust_core::ResultType;
use serde::de::{DeserializeOwned, Error};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

pub(crate) fn deserialize<'de, Data, Input>(
    deserializer: Input,
) -> Result<DataResult<Data>, Input::Error>
where
    Data: DeserializeOwned,
    Input: Deserializer<'de>,
{
    let mut object = Map::<String, Value>::deserialize(deserializer)?;
    let result_type = take_required::<ResultType, Input::Error>(&mut object, "type")?;
    let code = take_optional::<String, Input::Error>(&mut object, "code")?;
    let message = take_optional::<String, Input::Error>(&mut object, "message")?;
    let data_class = take_optional::<String, Input::Error>(&mut object, "data-class")?;

    let (data, data_element) = if data_class.is_some() {
        let element = take_required::<String, Input::Error>(&mut object, "data-element")?;
        let value = object.remove(&element).ok_or_else(|| {
            Input::Error::custom(format!(
                "the dynamic data element '{element}' was not found"
            ))
        })?;
        let data = serde_json::from_value(value).map_err(Input::Error::custom)?;
        (Some(data), Some(element))
    } else {
        (None, None)
    };

    Ok(DataResultParts {
        result_type,
        code,
        message,
        data,
        data_class,
        data_element,
    }
    .into())
}

fn take_required<ValueType, DeserializeError>(
    object: &mut Map<String, Value>,
    field: &str,
) -> Result<ValueType, DeserializeError>
where
    ValueType: DeserializeOwned,
    DeserializeError: Error,
{
    let value = object
        .remove(field)
        .ok_or_else(|| DeserializeError::custom(format!("missing required field '{field}'")))?;
    serde_json::from_value(value).map_err(DeserializeError::custom)
}

fn take_optional<ValueType, DeserializeError>(
    object: &mut Map<String, Value>,
    field: &str,
) -> Result<Option<ValueType>, DeserializeError>
where
    ValueType: DeserializeOwned,
    DeserializeError: Error,
{
    object
        .remove(field)
        .map(serde_json::from_value)
        .transpose()
        .map_err(DeserializeError::custom)
}
