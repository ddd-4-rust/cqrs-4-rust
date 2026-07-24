//! Axum application wiring migrated from Spring configuration.

mod axum_app;
mod axum_config;
mod person_resource;
mod test_model_serde_module;

pub use axum_app::AxumApp;
pub use axum_config::AxumConfig;
pub use person_resource::PersonResource;
pub use test_model_serde_module::TestModelSerdeModule;
