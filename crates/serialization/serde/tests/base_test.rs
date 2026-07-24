//! Shared Serde wire-name checks.

use cqrs_4_rust_core::ResultType;

#[test]
fn result_type_uses_java_enum_names() {
    assert_eq!(
        serde_json::to_string(&ResultType::Warning).expect("result type should serialize"),
        r#""WARNING""#
    );
}
