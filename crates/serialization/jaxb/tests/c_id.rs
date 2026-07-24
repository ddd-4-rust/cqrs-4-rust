//! JAXB entity ID fixture matching Java `CId`.

use ddd_4_rust_core::{EntityId, EntityType, StringBasedEntityType};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CId(String);

impl CId {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for CId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl EntityId for CId {
    fn entity_type(&self) -> &dyn EntityType {
        static ENTITY_TYPE: OnceLock<StringBasedEntityType> = OnceLock::new();
        ENTITY_TYPE.get_or_init(|| StringBasedEntityType::new("C").expect("valid entity type"))
    }

    fn as_string(&self) -> String {
        self.0.clone()
    }
}

#[test]
fn preserves_java_typed_identifier_form() {
    assert_eq!(CId::new("3").as_typed_string(), "C 3");
}
