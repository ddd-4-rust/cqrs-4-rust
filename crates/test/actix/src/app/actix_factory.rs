//! Explicit Actix dependency factory replacing Quarkus CDI producers.

use crate::app::KurrentDbWrapper;
use crate::view::PersonsView;
use crate::{PersonStore, person_store};
use cqrs_4_rust_actix::EventStoreConfig;
use std::sync::Arc;

/// Owns application-scoped Actix test services.
#[derive(Debug, Clone)]
pub struct ActixFactory {
    store: PersonStore,
    view: PersonsView,
    kurrent: Arc<KurrentDbWrapper>,
}

impl ActixFactory {
    /// Builds all services from event-store configuration.
    pub fn new(config: &EventStoreConfig) -> Self {
        let store = person_store();
        Self {
            view: PersonsView::new(store.clone()),
            store,
            kurrent: Arc::new(KurrentDbWrapper::new(config)),
        }
    }

    /// Returns shared person state.
    pub fn store(&self) -> PersonStore {
        self.store.clone()
    }

    /// Returns the person projection.
    pub const fn view(&self) -> &PersonsView {
        &self.view
    }

    /// Returns the `KurrentDB` lifecycle wrapper.
    pub fn kurrent(&self) -> Arc<KurrentDbWrapper> {
        self.kurrent.clone()
    }
}
