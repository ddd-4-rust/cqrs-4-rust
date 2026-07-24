//! JAXB command metadata compatibility tests.

use cqrs_4_rust_jaxb::AbstractCommand;
use ddd_4_rust_core::{Event, EventType};

#[test]
fn round_trips_command_metadata_as_xml() {
    let original = AbstractCommand::new_now(EventType::new("CreateA").expect("valid command type"));
    let xml = original.to_xml().expect("command should serialize");
    let copy = AbstractCommand::from_xml(&xml).expect("command should deserialize");

    assert_eq!(copy.event_id(), original.event_id());
    assert_eq!(copy.event_type(), original.event_type());
    assert!(xml.starts_with("<command>"));
}
