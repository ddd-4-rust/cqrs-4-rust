//! Architectural invariants replacing the Java `ArchUnit` base test.

use cqrs_4_rust_jsonb::{JandexJsonbRegistry, JsonbRegistry};

#[test]
fn built_in_adapter_is_discoverable_without_runtime_scanning() {
    let registry = JandexJsonbRegistry::new();
    assert!(
        registry
            .adapters()
            .iter()
            .any(|item| item.name == "DataResultJsonbAdapter")
    );
}
