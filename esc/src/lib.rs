//! CQRS-4-Rust ESC: Event Store Commons for CQRS projections.
//!
//! 1:1 translation of `cqrs-4-java-esc`.
//!
//! Provides `SimpleJpaEventDispatcher` and `ProjectionService`.

mod event_dispatcher;
mod projection_service;

pub use event_dispatcher::{EventDispatcher, SimpleEventDispatcher};
pub use projection_service::{ProjectionPosition, ProjectionService};
