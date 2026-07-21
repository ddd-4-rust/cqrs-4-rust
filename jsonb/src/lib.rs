//! CQRS-4-Rust JSON-B: serde-based serialization for CQRS commands and results.
//!
//! 1:1 translation of `cqrs-4-java-jsonb`.

// Re-export from jackson (identical in Rust — both use serde)
pub use cqrs_4_rust_jackson::{
    AbstractCommand, AbstractAggregateCommand, AbstractResult, SimpleResult, DataResult,
};
