//! JSON-B aggregate command identity tests.

use cqrs_4_rust_jsonb::AbstractAggregateCommand;
use ddd_4_rust_core::{
    AggregateRootUuid, AggregateVersion, DomainEvent, EntityId, EntityIdPath, EventType,
};
use std::sync::Arc;

#[test]
fn serializes_aggregate_identity_with_java_field_names() {
    let root_id = AggregateRootUuid::new("A").expect("valid entity type");
    let path = EntityIdPath::new(vec![Arc::new(root_id.clone()) as Arc<dyn EntityId>])
        .expect("non-empty path");
    let command = AbstractAggregateCommand::new_now(
        EventType::new("MyCommand").expect("valid event type"),
        path,
        Some(AggregateVersion::new(1)),
        root_id.clone(),
    );

    let value = serde_json::to_value(&command).expect("aggregate command should serialize");
    assert_eq!(command.aggregate_root_id(), &root_id);
    assert_eq!(command.aggregate_version(), Some(&AggregateVersion::new(1)));
    assert_eq!(value["aggregate-version"], 1);
    assert!(value["entity-id-path"].as_str().is_some());
}
