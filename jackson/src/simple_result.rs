//! Simple result without data.
//!
//! 1:1 translation of `org.fuin.cqrs4j.jackson.SimpleResult`.

use cqrs_4_rust_core::CqrsResult;

/// Simple result without data payload.
///
/// Java: `SimpleResult extends AbstractResult<Void>`
pub type SimpleResult = CqrsResult<()>;
