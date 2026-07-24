//! Event fixture helper migrated from `QuarkusTestHelper`.

use cqrs_4_rust_test_actix::generated::{PersonCreatedEvent, PersonId, PersonName};

fn person_created_event(name: &str) -> PersonCreatedEvent {
    PersonCreatedEvent::new(
        PersonId::new(),
        PersonName::new(name).expect("valid fixture name"),
    )
}

#[test]
fn creates_a_person_event_fixture() {
    let event = person_created_event("Peter Parker");
    assert_eq!(event.name().as_str(), "Peter Parker");
    assert_eq!(PersonCreatedEvent::TYPE, "PersonCreatedEvent");
}
