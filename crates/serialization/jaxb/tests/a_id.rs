//! JAXB aggregate-root ID fixture matching Java `AId`.

use ddd_4_rust_core::{AggregateRootId, EntityId, EntityType, StringBasedEntityType};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AId(String);

impl AId {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for AId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl EntityId for AId {
    fn entity_type(&self) -> &dyn EntityType {
        static ENTITY_TYPE: OnceLock<StringBasedEntityType> = OnceLock::new();
        ENTITY_TYPE.get_or_init(|| StringBasedEntityType::new("A").expect("valid entity type"))
    }

    fn as_string(&self) -> String {
        self.0.clone()
    }
}

impl AggregateRootId for AId {}

#[test]
fn preserves_java_typed_identifier_form() {
    assert_eq!(AId::new("1").as_typed_string(), "A 1");
}
