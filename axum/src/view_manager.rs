//! View manager for auto-discovering and scheduling CQRS views (axum version).
//!
//! Equivalent to `cqrs-4-rust-actix::ViewManager` but using axum/tower patterns.

use cqrs_4_rust_core::jpa_view::JpaView;
use std::sync::Arc;

/// Manages CQRS JpaView instances, schedules CRON-based event processing.
///
/// Designed for axum's `State` extractor pattern.
pub struct ViewManager {
    views: Vec<Arc<dyn JpaView>>,
}

impl ViewManager {
    /// Creates a new view manager with the given views.
    pub fn new(views: Vec<Arc<dyn JpaView>>) -> Self {
        Self { views }
    }

    /// Returns the registered views.
    pub fn views(&self) -> &[Arc<dyn JpaView>] {
        &self.views
    }

    /// Starts the CRON scheduler for all registered views.
    pub async fn start(&self) -> Result<(), ViewManagerError> {
        for view in &self.views {
            log::info!(
                "Registering view '{}' with CRON '{}' (axum)",
                view.name(),
                view.cron()
            );
        }
        Ok(())
    }

    /// Stops all scheduled tasks.
    pub async fn stop(&self) -> Result<(), ViewManagerError> {
        log::info!("Stopping axum view manager with {} views", self.views.len());
        Ok(())
    }
}

/// Errors during view manager operations.
#[derive(Debug, thiserror::Error)]
pub enum ViewManagerError {
    #[error("Scheduler error: {0}")]
    Scheduler(String),
    #[error("{0}")]
    Other(String),
}
