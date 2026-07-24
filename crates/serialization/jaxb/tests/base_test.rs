//! Shared JAXB wire-value checks.

use cqrs_4_rust_core::ResultType;

#[test]
fn result_type_uses_java_xml_values() {
    assert_eq!(
        quick_xml::se::to_string(&ResultType::Warning).expect("result type should serialize"),
        "<WARNING/>"
    );
}
