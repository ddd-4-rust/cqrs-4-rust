//! Actix application entry-point configuration.

use crate::app::PersonResource;
use actix_web::web;

/// Configures the migrated Quarkus HTTP surface in Actix.
#[derive(Debug, Clone, Copy, Default)]
pub struct ActixApp;

impl ActixApp {
    /// Registers all application routes.
    pub fn configure(config: &mut web::ServiceConfig) {
        PersonResource::configure(config);
    }
}
