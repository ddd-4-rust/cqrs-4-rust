//! JAXB data-result XML round-trip tests.

use cqrs_4_rust_jaxb::DataResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "invoice")]
struct Invoice {
    id: String,
}

#[test]
fn round_trips_an_xml_any_element_payload() {
    let original = DataResult::ok_with_element(
        Invoice {
            id: "I-0123456".to_owned(),
        },
        "invoice",
    );
    let xml = original.to_xml().expect("data result should serialize");
    assert!(xml.contains("<type>OK</type>"));
    assert!(xml.contains("<invoice><id>I-0123456</id></invoice>"));

    let copy =
        DataResult::<Invoice>::from_xml(&xml, "invoice").expect("data result should deserialize");
    assert_eq!(copy, original);
}
