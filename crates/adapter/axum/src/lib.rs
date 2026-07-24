//! CQRS-4-Rust Axum integration migrated from the Java Spring Boot module.
//!
//! 1:1 responsibility translation of `cqrs-4-java-springboot`.

mod axum_jpa_view_manager;
mod event_store_config;
mod query_projection_position;
mod query_projection_service;

pub use axum_jpa_view_manager::{AxumJpaViewManager, AxumJpaViewManagerError};
pub use event_store_config::EventStoreConfig;
pub use query_projection_position::QryProjectionPosition;
pub use query_projection_service::QryProjectionService;
