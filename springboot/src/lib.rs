//! CQRS-4-Rust Spring Boot: actix-web integration for CQRS views.
//!
//! 1:1 translation of `cqrs-4-java-springboot`.
//!
//! Provides `ViewManager` (equivalent to `SpringJpaViewManager`) for
//! auto-discovering `JpaView` implementations and scheduling CRON-based
//! event processing.

mod view_manager;
mod config;

pub use view_manager::ViewManager;
pub use config::EventStoreConfig;
