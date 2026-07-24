//! Base implementation for results.
//!
//! 1:1 translation of `org.fuin.cqrs4j.jackson.AbstractResult`.

use cqrs_4_rust_core::CqrsResult;

/// Re-export [`CqrsResult`] with serde support.
pub type AbstractResult<D = ()> = CqrsResult<D>;
