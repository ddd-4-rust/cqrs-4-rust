//! Event fixture helper migrated from `SpringBootTestHelper`.

use cqrs_4_rust_test_axum::app::TestModelSerdeModule;
use cqrs_4_rust_test_axum::generated::{PersonCreatedEvent, PersonId, PersonName};

#[test]
fn creates_and_serializes_a_person_event() {
    let event = PersonCreatedEvent::new(
        PersonId::new(),
        PersonName::new("Peter Parker").expect("valid name"),
    );
    let json = TestModelSerdeModule::to_json(&event).expect("event should serialize");
    let copy: PersonCreatedEvent =
        TestModelSerdeModule::from_json(&json).expect("event should deserialize");
    assert_eq!(copy, event);
}
