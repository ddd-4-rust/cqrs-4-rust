//! Spring Boot event-store configuration checks for Axum.

use cqrs_4_rust_axum::EventStoreConfig;

#[test]
fn applies_the_java_default_connection_values() {
    let config = EventStoreConfig::default();
    assert!(!config.tls);
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 2113);
}
