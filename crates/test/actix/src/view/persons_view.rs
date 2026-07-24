//! Actix person projection migrated from the Quarkus JPA view.

use crate::PersonStore;
use crate::generated::PersonCreatedEvent;
use crate::model::{AbstractPersonsView, PersonEntity};

/// Idempotent person read-model projection.
#[derive(Debug, Clone)]
pub struct PersonsView {
    store: PersonStore,
}

impl PersonsView {
    /// Creates a projection over the shared store.
    pub const fn new(store: PersonStore) -> Self {
        Self { store }
    }
}

impl AbstractPersonsView for PersonsView {
    fn name(&self) -> &'static str {
        "persons-view"
    }

    fn cron(&self) -> &'static str {
        "* * * * * *"
    }

    async fn handle(&self, event: &PersonCreatedEvent) -> bool {
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
