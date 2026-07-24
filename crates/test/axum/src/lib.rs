//! Axum integration model migrated from the Java Spring Boot end-to-end module.

pub mod app;
pub mod generated;
pub mod model;
pub mod view;

use model::PersonEntity;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Shared person read-model store.
pub type PersonStore = Arc<RwLock<BTreeMap<Uuid, PersonEntity>>>;

/// Creates an empty person read-model store.
pub fn person_store() -> PersonStore {
    Arc::new(RwLock::new(BTreeMap::new()))
}
