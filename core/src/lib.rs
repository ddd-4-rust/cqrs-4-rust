//! CQRS-4-Rust Core: Command Query Responsibility Segregation traits.
//!
//! 1:1 translation of `cqrs-4-java-core`.

pub mod command;
pub mod aggregate_command;
pub mod command_executor;
pub mod multi_command_executor;
pub mod result;
pub mod result_type;
pub mod view;
pub mod jpa_view;
pub mod jpa_event_handler;
pub mod cqrs_utils;
pub mod exceptions;

pub use command::Command;
pub use aggregate_command::AggregateCommand;
pub use command_executor::CommandExecutor;
pub use multi_command_executor::MultiCommandExecutor;
pub use result::CqrsResult;
pub use result_type::ResultType;
pub use view::View;
pub use jpa_view::JpaView;
pub use jpa_event_handler::{JpaEventHandler, JpaEventHandlerError};
pub use cqrs_utils::CqrsUtils;
pub use exceptions::CommandExecutionError;
