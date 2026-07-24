//! Entity path mismatch checks.

use cqrs_4_rust_core::UrlParamEntityIdPathNotEqualsCmdError;

#[test]
fn exposes_both_entity_paths() {
    let error = UrlParamEntityIdPathNotEqualsCmdError::new("Person 1", "Person 2");
    assert_eq!(error.url_path(), "Person 1");
    assert_eq!(error.command_path(), "Person 2");
    assert!(error.to_string().contains("Person 1"));
    assert!(error.to_string().contains("Person 2"));
}
