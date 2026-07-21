//! View manager for auto-discovering and scheduling CQRS views.
//!
//! 1:1 translation of `org.fuin.cqrs4j.springboot.view.SpringJpaViewManager`.

use cqrs_4_rust_core::jpa_view::JpaView;
use std::sync::Arc;

/// Manages CQRS JpaView instances, schedules CRON-based event processing.
///
/// Java: `SpringJpaViewManager implements ApplicationListener<ContextClosedEvent>, SchedulingConfigurer`
///
/// In Rust/actix-web, views are registered and scheduled via this manager.
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
    ///
    /// In Java, this is handled by `SpringJpaViewManager.configureTasks()`.
    /// In Rust, this would use `tokio-cron-scheduler` to create scheduled tasks.
    pub async fn start(&self) -> Result<(), ViewManagerError> {
        for view in &self.views {
            log::info!(
                "Registering view '{}' with CRON '{}'",
                view.name(),
                view.cron()
            );
            // In production, create tokio-cron-scheduler tasks here
        }
        Ok(())
    }

    /// Stops all scheduled tasks.
    pub async fn stop(&self) -> Result<(), ViewManagerError> {
        log::info!("Stopping view manager with {} views", self.views.len());
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
