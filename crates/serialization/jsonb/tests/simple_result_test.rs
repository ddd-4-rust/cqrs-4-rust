//! JSON-B simple result tests.

use cqrs_4_rust_core::ResultType;
use cqrs_4_rust_jsonb::SimpleResult;

#[test]
fn preserves_java_result_type_and_optional_fields() {
    let ok = SimpleResult::ok();
    assert_eq!(
        serde_json::to_value(&ok).expect("valid result"),
        serde_json::json!({"type":"OK"})
    );

    let error = SimpleResult::error("DDD4J-AGGREGATE_NOT_FOUND", "Invoice not found");
    let json = serde_json::to_string(&error).expect("error should serialize");
    let copy: SimpleResult = serde_json::from_str(&json).expect("error should deserialize");
    assert_eq!(copy.result_type(), ResultType::Error);
    assert_eq!(copy.code(), Some("DDD4J-AGGREGATE_NOT_FOUND"));
    assert_eq!(copy.message(), Some("Invoice not found"));
}
