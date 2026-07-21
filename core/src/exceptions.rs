//! CQRS-specific exceptions.
//!
//! 1:1 translation of `CommandExecutionFailedException` and related exceptions.

use thiserror::Error;

/// Errors that occur during command execution.
///
/// Java: `CommandExecutionFailedException extends Exception`
#[derive(Debug, Error)]
pub enum CommandExecutionError {
    /// The command execution failed due to a domain error.
    #[error("Command execution failed: {message}")]
    CommandExecutionFailed {
        /// Human-readable message.
        message: String,
    },

    /// No executor was found for the given command type.
    #[error("No executor found for command type: {command_type}")]
    NoExecutorFound {
        /// The command type that has no executor.
        command_type: String,
    },

    /// The URL entity ID path doesn't match the command's entity ID path.
    ///
    /// Java: `UrlParamEntityIdPathNotEqualsCmdException`
    #[error("URL entity ID path '{url_path}' doesn't match command entity ID path '{cmd_path}'")]
    UrlParamEntityIdPathNotEqualsCmd {
        /// The path from the URL.
        url_path: String,
        /// The path from the command.
        cmd_path: String,
    },

    /// Wrapped error from ddd-4-rust-core.
    #[error("Aggregate error: {0}")]
    AggregateError(#[from] ddd_4_rust_core::AggregateError),

    /// Other errors.
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
