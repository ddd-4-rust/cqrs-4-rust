//! Facade for the cqrs-4-rust ecosystem.
//!
//! The core CQRS contracts are always available. Event-store, serialization,
//! and web-framework integrations are exposed through opt-in features.

/// Core commands, executors, results, views, and handlers.
pub use cqrs_4_rust_core as core;

/// Event-store commons integration.
#[cfg(feature = "esc")]
pub use cqrs_4_rust_esc as esc;

/// Serde implementation of the Java-compatible wire format.
#[cfg(feature = "serde")]
pub use cqrs_4_rust_serde as serde;

/// JAXB-compatible XML serialization support.
#[cfg(feature = "jaxb")]
pub use cqrs_4_rust_jaxb as jaxb;

/// JSON-B-compatible serialization and compile-time component registry.
#[cfg(feature = "jsonb")]
pub use cqrs_4_rust_jsonb as jsonb;

/// Actix Web integration.
#[cfg(feature = "actix")]
pub use cqrs_4_rust_actix as actix;

/// Axum integration.
#[cfg(feature = "axum")]
pub use cqrs_4_rust_axum as axum;
