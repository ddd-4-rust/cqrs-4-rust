//! CQRS-4-Rust Actix integration migrated from the Java Quarkus module.
//!
//! 1:1 responsibility translation of `cqrs-4-java-quarkus`.
//!
//! Provides `ActixJpaViewManager` (equivalent to `QuarkusJpaViewManager`) for
//! auto-discovering `JpaView` implementations and scheduling CRON-based
//! event processing.

mod actix_jpa_view_manager;
mod event_store_config;
mod query_projection_position;
mod query_projection_position_repository;

pub use actix_jpa_view_manager::{ActixJpaViewManager, ActixJpaViewManagerError};
pub use event_store_config::EventStoreConfig;
pub use query_projection_position::QryProjectionPosition;
pub use query_projection_position_repository::QryProjectionPositionRepository;
