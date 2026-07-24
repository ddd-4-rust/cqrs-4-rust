//! Errors raised while executing or registering commands.

use crate::UrlParamEntityIdPathNotEqualsCmdError;
use thiserror::Error;

/// Errors that occur during command execution.
#[derive(Debug, Error)]
pub enum CommandExecutionError {
    /// A domain-specific command execution failure.
    #[error("Command execution failed: {message}")]
    CommandExecutionFailed {
        /// Human-readable error message.
        message: String,
    },

    /// No executor is registered for a command type.
    #[error("No executor found for command type: {command_type}")]
    NoExecutorFound {
        /// The unhandled command type.
        command_type: String,
    },

    /// Executor registration violates an invariant.
    #[error("Invalid command executor configuration: {message}")]
    InvalidExecutorConfiguration {
        /// Human-readable invariant violation.
        message: String,
    },

    /// The URL and command entity paths differ.
    #[error(transparent)]
    EntityIdPathMismatch(#[from] UrlParamEntityIdPathNotEqualsCmdError),

    /// Error propagated by the DDD aggregate layer.
    #[error("Aggregate error: {0}")]
    AggregateError(#[from] ddd_4_rust_core::AggregateError),

    /// Any other command execution error.
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
