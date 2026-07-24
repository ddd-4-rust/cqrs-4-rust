//! Fixture corresponding to Jackson `ACreatedEvent`.

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ACreatedEvent {
    id: i64,
}

#[test]
fn round_trips_a_created_event_fixture() {
    let original = ACreatedEvent { id: 123 };
    let json = serde_json::to_string(&original).expect("event should serialize");
    let copy: ACreatedEvent = serde_json::from_str(&json).expect("event should deserialize");
    assert_eq!(copy, original);
}
