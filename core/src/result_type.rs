//! Result type enumeration.
//!
//! 1:1 translation of `org.fuin.cqrs4j.core.ResultType`.

use serde::{Deserialize, Serialize};

/// Result type signaling if execution was successful or not.
///
/// Java: `enum ResultType { OK, WARNING, ERROR }`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResultType {
    /// Execution was successful.
    Ok,
    /// Execution completed with warnings.
    Warning,
    /// Execution failed.
    Error,
}
