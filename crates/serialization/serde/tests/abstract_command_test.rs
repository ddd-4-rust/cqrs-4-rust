//! `AbstractCommand` Serde round-trip checks.

use cqrs_4_rust_serde::AbstractCommand;
use ddd_4_rust_core::{Event, EventType};

#[test]
fn round_trips_event_metadata() {
    let original =
        AbstractCommand::new_now(EventType::new("CreatePerson").expect("valid command event type"));
    let json = serde_json::to_string(&original).expect("command should serialize");
    let copy: AbstractCommand = serde_json::from_str(&json).expect("command should deserialize");

    assert_eq!(copy.event_id(), original.event_id());
    assert_eq!(copy.event_type(), original.event_type());
    assert_eq!(copy.correlation_id(), None);
    assert_eq!(copy.causation_id(), None);
}
