//! Explicit Axum state replacing Spring bean configuration.

use crate::PersonStore;
use crate::person_store;
use crate::view::PersonsView;
use cqrs_4_rust_axum::EventStoreConfig;

/// Application-scoped Axum dependencies.
#[derive(Debug, Clone)]
pub struct AxumConfig {
    /// Event-store connection properties.
    pub event_store: EventStoreConfig,
    /// Shared person read-model state.
    pub store: PersonStore,
    /// Person projection.
    pub view: PersonsView,
}

impl AxumConfig {
    /// Builds application state from event-store configuration.
    pub fn new(event_store: EventStoreConfig) -> Self {
        let store = person_store();
        Self {
            event_store,
            view: PersonsView::new(store.clone()),
            store,
        }
    }
}

impl Default for AxumConfig {
    fn default() -> Self {
        Self::new(EventStoreConfig::default())
    }
}
