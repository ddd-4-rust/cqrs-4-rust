//! Common behavior shared by all commands.
//!
//! 1:1 translation of `org.fuin.cqrs4j.core.Command`.

use ddd_4_rust_core::event::Event;

/// Common behavior shared by all commands.
///
/// In this model, commands ARE events (they can be stored in the event store).
///
/// Java: `Command extends Event`
pub trait Command: Event {}
