//! JAXB-compatible XML serialization for CQRS commands and results.

mod abstract_aggregate_command;
mod abstract_command;
mod abstract_result;
mod data_result;
mod simple_result;

pub use abstract_aggregate_command::AbstractAggregateCommand;
pub use abstract_command::AbstractCommand;
pub use abstract_result::AbstractResult;
pub use data_result::DataResult;
pub use simple_result::SimpleResult;
