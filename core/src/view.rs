//! View (projection) marker trait.
//!
//! 1:1 translation of `org.fuin.cqrs4j.core.View`.

use ddd_4_rust_core::event_type::EventType;

/// A CQRS view representing a read-side projection.
///
/// Java: `View` interface
///
/// Methods: `getName()`, `getEventTypes()`
pub trait View: Send + Sync {
    /// Returns the unique name of this view.
    ///
    /// Java: `getName() -> String`
    fn name(&self) -> &str;

    /// Returns the set of event types this view subscribes to.
    ///
    /// Java: `getEventTypes() -> Set<EventType>`
    fn event_types(&self) -> Vec<EventType>;
}
