//! Exact JSON-B `DataResult` wire-format tests.

use cqrs_4_rust_core::ResultType;
use cqrs_4_rust_jsonb::{DataResult, DataResultJsonbAdapter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Invoice {
    id: String,
}

#[test]
fn round_trips_the_dynamic_payload_element() {
    let original = DataResult::with_metadata(
        ResultType::Ok,
        None,
        None,
        Some(Invoice {
            id: "I-0123456".to_owned(),
        }),
        Some("org.fuin.cqrs4j.jsonb.DataResultTest$Invoice".to_owned()),
        Some("invoice".to_owned()),
    );

    let value =
        DataResultJsonbAdapter::adapt_to_json(&original).expect("result should adapt to JSON");
    assert_eq!(value["type"], "OK");
    assert_eq!(value["data-element"], "invoice");
    assert_eq!(value["invoice"]["id"], "I-0123456");

    let copy: DataResult<Invoice> =
        DataResultJsonbAdapter::adapt_from_json(value).expect("result should adapt from JSON");
    assert_eq!(copy, original);
}

#[test]
fn requires_data_element_for_a_present_payload() {
    let result = DataResult::ok(Some(Invoice {
        id: "I-0123456".to_owned(),
    }));
    let error = DataResultJsonbAdapter::adapt_to_json(&result)
        .expect_err("payload metadata must be mandatory");
    assert!(error.to_string().contains("data-element"));
}
