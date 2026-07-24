//! Axum application entry point migrated from Spring Boot.

use crate::app::{AxumConfig, PersonResource};
use axum::Router;
use axum::routing::get;

/// Builds the complete Axum HTTP application.
#[derive(Debug, Clone, Copy, Default)]
pub struct AxumApp;

impl AxumApp {
    /// Creates the application router.
    pub fn router(config: AxumConfig) -> Router {
        Router::new()
            .route("/persons/{id}", get(PersonResource::read))
            .with_state(config)
    }
}
