//! Serde serializer for the Java-compatible `DataResult` wire format.

use crate::DataResult;
use serde::ser::{Error, SerializeMap};
use serde::{Serialize, Serializer};

pub(crate) fn serialize<Data, Output>(
    result: &DataResult<Data>,
    serializer: Output,
) -> Result<Output::Ok, Output::Error>
where
    Data: Serialize,
    Output: Serializer,
{
    let mut map = serializer.serialize_map(None)?;
    map.serialize_entry("type", &result.result_type())?;
    if let Some(code) = result.code() {
        map.serialize_entry("code", code)?;
    }
    if let Some(message) = result.message() {
        map.serialize_entry("message", message)?;
    }
    if let Some(data) = result.data() {
        let data_element = result.data_element().ok_or_else(|| {
            Output::Error::custom(
                "the 'data-element' value is required when serializing result data",
            )
        })?;
        let data_class = result.data_class().unwrap_or(std::any::type_name::<Data>());
        map.serialize_entry("data-class", data_class)?;
        map.serialize_entry("data-element", data_element)?;
        map.serialize_entry(data_element, data)?;
    }
    map.end()
}
