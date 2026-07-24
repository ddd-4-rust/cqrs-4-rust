//! JSON-B compatibility implemented with Rust's Serde ecosystem.
//!
//! Java's runtime Jandex discovery is represented by [`inventory`]-based
//! compile-time registration. The wire contract remains independent from the
//! Jackson-compatible crate.

mod abstract_aggregate_command;
mod abstract_command;
mod abstract_result;
mod data_result;
mod data_result_jsonb_adapter;
mod jandex_jsonb_registry;
mod jsonb_registry;
mod simple_result;

pub use abstract_aggregate_command::AbstractAggregateCommand;
pub use abstract_command::AbstractCommand;
pub use abstract_result::AbstractResult;
pub use data_result::DataResult;
pub use data_result_jsonb_adapter::{DataResultJsonbAdapter, DataResultJsonbAdapterError};
pub use jandex_jsonb_registry::JandexJsonbRegistry;
pub use jsonb_registry::{JsonbComponent, JsonbComponentKind, JsonbRegistry};
pub use simple_result::SimpleResult;

inventory::submit! {
    JsonbComponent::adapter("DataResultJsonbAdapter")
}
