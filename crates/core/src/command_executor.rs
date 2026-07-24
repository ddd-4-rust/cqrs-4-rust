//! Executes one or more commands.

use crate::{Command, CommandExecutionError};
use async_trait::async_trait;
use ddd_4_rust_core::EventType;
use std::collections::HashSet;

/// Executes one or more command types and returns an application-defined output.
#[async_trait]
pub trait CommandExecutor<Context, Output>: Send + Sync {
    /// Returns the unique command types handled by this executor.
    fn command_types(&self) -> HashSet<EventType>;

    /// Executes a command.
    async fn execute(
        &self,
        context: &Context,
        command: &dyn Command,
    ) -> Result<Output, CommandExecutionError>;
}
