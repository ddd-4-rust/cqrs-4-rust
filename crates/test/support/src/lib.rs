//! Shared test support for cqrs-4-rust integration suites.

mod test_helper;

pub use test_helper::{EventStoreContainerSpec, MariaDbContainerSpec, TestHelper};
