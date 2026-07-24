//! Conversion into a CQRS result.

use crate::CqrsResult;

/// Marks a value that can be converted into a CQRS result.
pub trait ToResultCapable<Data> {
    /// Converts this value into a result.
    fn to_result(&self) -> CqrsResult<Data>;
}
