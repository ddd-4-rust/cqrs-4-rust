//! `SimpleResult` behavior checks.

use cqrs_4_rust_core::ResultType;
use cqrs_4_rust_serde::SimpleResult;

#[test]
fn serializes_success_warning_and_error_results() {
    let ok = SimpleResult::ok();
    assert_eq!(ok.result_type(), ResultType::Ok);
    assert_eq!(serde_json::to_value(ok).expect("valid JSON")["type"], "OK");

    let warning = SimpleResult::warning("W1", "careful");
    assert_eq!(warning.code(), Some("W1"));

    let error = SimpleResult::error("E1", "failed");
    assert_eq!(error.result_type(), ResultType::Error);
}
