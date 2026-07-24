//! Serde serialization for CQRS commands and results.
//!
//! Implements the wire contracts migrated from `cqrs-4-java-jackson` using
//! Rust's Serde ecosystem.

mod abstract_aggregate_command;
mod abstract_command;
mod abstract_result;
mod cqrs_4_serde_module;
mod data_result;
mod data_result_deserializer;
mod data_result_serializer;
mod simple_result;

pub use abstract_aggregate_command::AbstractAggregateCommand;
pub use abstract_command::AbstractCommand;
pub use abstract_result::AbstractResult;
pub use cqrs_4_serde_module::Cqrs4SerdeModule;
pub use data_result::DataResult;
pub use simple_result::SimpleResult;
