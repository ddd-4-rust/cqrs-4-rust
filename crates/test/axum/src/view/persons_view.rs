//! Axum person projection migrated from the Spring JPA view.

use crate::PersonStore;
use crate::generated::PersonCreatedEvent;
use crate::model::PersonEntity;

/// Idempotent person projection.
#[derive(Debug, Clone)]
pub struct PersonsView {
    store: PersonStore,
}

impl PersonsView {
    /// Creates a projection over the shared store.
    pub const fn new(store: PersonStore) -> Self {
        Self { store }
    }

    /// Stable view name.
    pub const fn name(&self) -> &'static str {
        "persons-view"
    }

    /// Java-compatible every-second CRON expression.
    pub const fn cron(&self) -> &'static str {
        "* * * * * *"
    }

    /// Applies an event once, returning whether a row was inserted.
    pub async fn handle(&self, event: &PersonCreatedEvent) -> bool {
        let mut store = self.store.write().await;
        if let std::collections::btree_map::Entry::Vacant(entry) = store.entry(event.id().as_uuid())
        {
            entry.insert(PersonEntity::new(event.id(), event.name()));
            true
        } else {
            false
        }
    }
}
