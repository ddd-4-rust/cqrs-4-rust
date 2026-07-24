//! Lifecycle-safe `KurrentDB` connection settings wrapper.

use cqrs_4_rust_actix::EventStoreConfig;
use std::sync::atomic::{AtomicBool, Ordering};

/// Wraps the configured `KurrentDB` endpoint and shutdown state.
#[derive(Debug)]
pub struct KurrentDbWrapper {
    endpoint: String,
    tls: bool,
    active: AtomicBool,
}

impl KurrentDbWrapper {
    /// Creates a wrapper from adapter configuration.
    pub fn new(config: &EventStoreConfig) -> Self {
        Self {
            endpoint: format!("{}:{}", config.host, config.port),
            tls: config.tls,
            active: AtomicBool::new(true),
        }
    }

    /// Returns the configured endpoint.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Returns whether TLS is enabled.
    pub const fn tls(&self) -> bool {
        self.tls
    }

    /// Returns whether the wrapper is active.
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Acquire)
    }

    /// Marks all wrapped clients as shut down.
    pub fn shutdown(&self) {
        self.active.store(false, Ordering::Release);
    }
}
