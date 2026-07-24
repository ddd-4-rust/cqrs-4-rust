//! Invalid Serde `DataResult` input checks.

use cqrs_4_rust_serde::{Cqrs4SerdeModule, DataResult};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Payload {
    #[allow(dead_code)]
    value: String,
}

#[test]
fn requires_data_element_when_data_class_exists() {
    let json = r#"{"type":"OK","data-class":"example.Payload"}"#;
    let error =
        Cqrs4SerdeModule::from_json::<Payload>(json).expect_err("data-element must be required");
    assert!(error.to_string().contains("data-element"));
}

#[test]
fn requires_the_declared_dynamic_field() {
    let json = r#"{"type":"OK","data-class":"example.Payload","data-element":"payload"}"#;
    let error = Cqrs4SerdeModule::from_json::<Payload>(json)
        .expect_err("dynamic data field must be required");
    assert!(error.to_string().contains("payload"));

    let _: Option<DataResult<Payload>> = None;
}
