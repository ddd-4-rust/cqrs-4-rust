//! JSON-B result construction tests.

use cqrs_4_rust_core::ResultType;
use cqrs_4_rust_jsonb::DataResult;

#[test]
fn creates_success_and_error_results() {
    let success = DataResult::ok_with_element("payload".to_owned(), "data");
    assert_eq!(success.result_type(), ResultType::Ok);
    assert_eq!(success.data(), Some(&"payload".to_owned()));
    assert_eq!(success.data_element(), Some("data"));
    assert!(success.data_class().is_some());

    let error = DataResult::<()>::error("E-1", "failed");
    assert_eq!(error.result_type(), ResultType::Error);
    assert_eq!(error.code(), Some("E-1"));
    assert_eq!(error.message(), Some("failed"));
    assert_eq!(error.data(), None);
}
