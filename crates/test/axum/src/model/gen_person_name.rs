//! Generator contract for person names.

/// Behavior required from generated person names.
pub trait GenPersonName {
    /// Returns the underlying name.
    fn value(&self) -> &str;
}
