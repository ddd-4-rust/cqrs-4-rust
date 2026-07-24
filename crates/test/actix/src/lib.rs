//! Actix integration model migrated from the Java Quarkus end-to-end module.

pub mod app;
pub mod generated;
pub mod model;
pub mod view;

use model::PersonEntity;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Shared in-process read-model store used by the integration application.
pub type PersonStore = Arc<RwLock<BTreeMap<Uuid, PersonEntity>>>;

/// Creates an empty read-model store.
pub fn person_store() -> PersonStore {
    Arc::new(RwLock::new(BTreeMap::new()))
}
