//! Shared result contract checks.

use cqrs_4_rust_core::{CqrsResult, ResultType};

#[test]
fn result_wire_names_match_java() {
    let result = CqrsResult::<()>::error("E-1", "failed");
    let json = serde_json::to_value(result).expect("result should serialize");

    assert_eq!(json["type"], "ERROR");
    assert_eq!(json["code"], "E-1");
    assert_eq!(json["message"], "failed");
    assert_eq!(ResultType::Ok, serde_json::from_str("\"OK\"").unwrap());
}
