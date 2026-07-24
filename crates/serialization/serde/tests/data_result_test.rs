//! `DataResult` construction and round-trip checks.

use cqrs_4_rust_core::ResultType;
use cqrs_4_rust_serde::{Cqrs4SerdeModule, DataResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Invoice {
    id: String,
}

#[test]
fn round_trips_dynamic_payload_field() {
    let original = DataResult::with_metadata(
        ResultType::Ok,
        None,
        None,
        Some(Invoice {
            id: "I-0123456".to_owned(),
        }),
        Some("org.fuin.cqrs4j.jackson.DataResultTest$Invoice".to_owned()),
        Some("invoice".to_owned()),
    );

    let json = Cqrs4SerdeModule::to_json(&original).expect("data result should serialize");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
    assert_eq!(value["type"], "OK");
    assert_eq!(value["data-element"], "invoice");
    assert_eq!(value["invoice"]["id"], "I-0123456");

    let copy: DataResult<Invoice> =
        Cqrs4SerdeModule::from_json(&json).expect("data result should deserialize");
    assert_eq!(copy, original);
}

#[test]
fn serializing_data_without_element_name_fails() {
    let result = DataResult::ok(Some(Invoice {
        id: "I-0123456".to_owned(),
    }));
    let error = Cqrs4SerdeModule::to_json(&result).expect_err("element name is mandatory");
    assert!(error.to_string().contains("data-element"));
}
