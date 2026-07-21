//! Data result with optional payload.
//!
//! 1:1 translation of `org.fuin.cqrs4j.jackson.DataResult`.

use cqrs_4_rust_core::CqrsResult;

/// Data result with optional payload data.
///
/// Java: `DataResult<D> extends AbstractResult<D>`
pub type DataResult<D> = CqrsResult<D>;
