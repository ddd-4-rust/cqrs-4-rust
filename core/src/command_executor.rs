//! Executes one or more commands.
//!
//! 1:1 translation of `org.fuin.cqrs4j.core.CommandExecutor`.

use async_trait::async_trait;
use ddd_4_rust_core::event_type::EventType;
use crate::command::Command;
use crate::exceptions::CommandExecutionError;

/// Executes one or more commands.
///
/// Java: `CommandExecutor<CONTEXT, RESULT, CMD extends Command>`
///
/// # Type Parameters
/// - `Ctx`: Type of context for the command execution.
/// - `Cmd`: Type of command to execute.
#[async_trait]
pub trait CommandExecutor<Ctx, Cmd: Command + ?Sized>: Send + Sync {
    /// Returns a list of commands this executor can handle.
    ///
    /// Java: `@NotNull getCommandTypes() -> Set<EventType>`
    fn command_types(&self) -> Vec<EventType>;

    /// Executes the given command.
    ///
    /// Java: `execute(CONTEXT ctx, CMD cmd) -> RESULT`
    async fn execute(
        &self,
        ctx: &Ctx,
        cmd: &Cmd,
    ) -> Result<(), CommandExecutionError>;
}
