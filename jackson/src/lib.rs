//! CQRS-4-Rust Jackson: serde-based serialization for CQRS commands and results.
//!
//! 1:1 translation of `cqrs-4-java-jackson`.

mod abstract_command;
mod abstract_aggregate_command;
mod abstract_result;
mod simple_result;
mod data_result;

pub use abstract_command::AbstractCommand;
pub use abstract_aggregate_command::AbstractAggregateCommand;
pub use abstract_result::AbstractResult;
pub use simple_result::SimpleResult;
pub use data_result::DataResult;
