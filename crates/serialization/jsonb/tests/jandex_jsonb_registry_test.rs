//! Compile-time JSON-B registry tests.

use cqrs_4_rust_jsonb::{JandexJsonbRegistry, JsonbComponent, JsonbComponentKind, JsonbRegistry};

inventory::submit! {
    JsonbComponent::serializer("InvoiceSerializer")
}

inventory::submit! {
    JsonbComponent::deserializer("InvoiceDeserializer")
}

#[test]
fn groups_inventory_components_by_responsibility() {
    let registry = JandexJsonbRegistry::new();
    assert!(
        registry
            .adapters()
            .iter()
            .any(|item| item.name == "DataResultJsonbAdapter")
    );
    assert_eq!(
        registry.serializers(),
        &[JsonbComponent {
            name: "InvoiceSerializer",
            kind: JsonbComponentKind::Serializer,
        }]
    );
    assert_eq!(
        registry.deserializers(),
        &[JsonbComponent {
            name: "InvoiceDeserializer",
            kind: JsonbComponentKind::Deserializer,
        }]
    );
}
