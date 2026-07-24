//! CQRS utility checks.

use cqrs_4_rust_core::CqrsUtils;
use ddd_4_rust_core::EventType;

#[test]
fn calculates_java_compatible_adler32_checksum() {
    assert_eq!(
        CqrsUtils::calculate_adler32_checksum(&[EventType::new("A").unwrap()]),
        4_325_442
    );
    assert_eq!(
        CqrsUtils::calculate_adler32_checksum(&[
            EventType::new("A").unwrap(),
            EventType::new("B").unwrap(),
        ]),
        12_976_260
    );
}

#[test]
#[should_panic(expected = "event_types cannot be empty")]
fn rejects_empty_event_types() {
    CqrsUtils::calculate_adler32_checksum(&[]);
}
