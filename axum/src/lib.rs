//! CQRS-4-Rust axum: axum integration for CQRS views.
//!
//! Equivalent to `cqrs-4-rust-actix` but for the axum web framework.

mod view_manager;
mod config;

pub use view_manager::ViewManager;
pub use config::EventStoreConfig;
