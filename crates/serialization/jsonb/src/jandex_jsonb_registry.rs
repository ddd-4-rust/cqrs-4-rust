//! Compile-time replacement for Java's Jandex JSON-B scanning.

use crate::{JsonbComponent, JsonbComponentKind, JsonbRegistry};

/// Registry populated from all linked [`inventory`] submissions.
#[derive(Debug, Clone, Default)]
pub struct JandexJsonbRegistry {
    adapters: Vec<JsonbComponent>,
    serializers: Vec<JsonbComponent>,
    deserializers: Vec<JsonbComponent>,
}

impl JandexJsonbRegistry {
    /// Collects and deterministically orders all linked JSON-B components.
    pub fn new() -> Self {
        let mut registry = Self::default();
        for component in inventory::iter::<JsonbComponent> {
            match component.kind {
                JsonbComponentKind::Adapter => registry.adapters.push(*component),
                JsonbComponentKind::Serializer => registry.serializers.push(*component),
                JsonbComponentKind::Deserializer => registry.deserializers.push(*component),
            }
        }
        registry.adapters.sort_unstable_by_key(|item| item.name);
        registry.serializers.sort_unstable_by_key(|item| item.name);
        registry
            .deserializers
            .sort_unstable_by_key(|item| item.name);
        registry
    }
}

impl JsonbRegistry for JandexJsonbRegistry {
    fn adapters(&self) -> &[JsonbComponent] {
        &self.adapters
    }

    fn serializers(&self) -> &[JsonbComponent] {
        &self.serializers
    }

    fn deserializers(&self) -> &[JsonbComponent] {
        &self.deserializers
    }
}
