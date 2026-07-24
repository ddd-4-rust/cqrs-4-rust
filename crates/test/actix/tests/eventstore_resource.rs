//! `EventStoreDB` container specification migrated from Quarkus test resources.

use cqrs_4_rust_test_support::TestHelper;

#[test]
fn preserves_eventstore_container_contract() {
    let resource = TestHelper::event_store("24.10");
    assert_eq!(resource.image, "eventstore/eventstore:24.10");
    assert_eq!(resource.exposed_port, 2113);
    assert_eq!(resource.environment["EVENTSTORE_MEM_DB"], "TRUE");
}
