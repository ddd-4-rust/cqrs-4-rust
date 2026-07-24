//! Registry contract replacing the Java JSON-B component lists.

/// Category of a JSON-B extension component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonbComponentKind {
    /// Converts one representation into another.
    Adapter,
    /// Provides custom serialization.
    Serializer,
    /// Provides custom deserialization.
    Deserializer,
}

/// Compile-time descriptor for a JSON-B extension component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonbComponent {
    /// Stable component name.
    pub name: &'static str,
    /// Component category.
    pub kind: JsonbComponentKind,
}

impl JsonbComponent {
    /// Creates an adapter descriptor.
    pub const fn adapter(name: &'static str) -> Self {
        Self {
            name,
            kind: JsonbComponentKind::Adapter,
        }
    }

    /// Creates a serializer descriptor.
    pub const fn serializer(name: &'static str) -> Self {
        Self {
            name,
            kind: JsonbComponentKind::Serializer,
        }
    }

    /// Creates a deserializer descriptor.
    pub const fn deserializer(name: &'static str) -> Self {
        Self {
            name,
            kind: JsonbComponentKind::Deserializer,
        }
    }
}

inventory::collect!(JsonbComponent);

/// Provides all registered JSON-B components grouped by responsibility.
pub trait JsonbRegistry {
    /// Returns registered adapters.
    fn adapters(&self) -> &[JsonbComponent];

    /// Returns registered serializers.
    fn serializers(&self) -> &[JsonbComponent];

    /// Returns registered deserializers.
    fn deserializers(&self) -> &[JsonbComponent];
}
