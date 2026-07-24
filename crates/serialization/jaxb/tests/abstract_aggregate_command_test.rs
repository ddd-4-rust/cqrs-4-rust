//! JAXB aggregate-command compatibility tests.

mod a_id;

use a_id::AId;
use cqrs_4_rust_core::AggregateCommand;
use cqrs_4_rust_jaxb::AbstractAggregateCommand;
use ddd_4_rust_core::{AggregateVersion, EntityIdPath, EventType};
use std::sync::Arc;

#[test]
fn keeps_the_aggregate_root_and_serializes_the_entity_path() {
    let root_id = AId::new("4711");
    let path = EntityIdPath::new(vec![Arc::new(root_id.clone())]).expect("non-empty path");
    let command = AbstractAggregateCommand::new_now(
        EventType::new("ChangeA").expect("valid command type"),
        path,
        Some(AggregateVersion::new(3)),
        root_id.clone(),
    );

    let xml = command
        .to_xml()
        .expect("aggregate command should serialize");
    assert_eq!(command.aggregate_root_id(), &root_id);
    assert!(xml.contains("A 4711"));
    assert!(!xml.contains("aggregate_root_id"));
}
