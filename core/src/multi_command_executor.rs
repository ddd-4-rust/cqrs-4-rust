//! Dispatches commands to registered executors by command type.
//!
//! 1:1 translation of `org.fuin.cqrs4j.core.AbstractMultiCommandExecutor` and `MultiCommandExecutor`.

use ddd_4_rust_core::event_type::EventType;
use crate::command::Command;
use crate::command_executor::CommandExecutor;
use crate::exceptions::CommandExecutionError;
use std::collections::HashMap;

/// Dispatches commands to registered executors by command type.
///
/// Java: `AbstractMultiCommandExecutor<CONTEXT, RESULT>` / `MultiCommandExecutor`
///
/// Maintains a map of `EventType` → `CommandExecutor` and routes commands accordingly.
pub struct MultiCommandExecutor<Ctx> {
    executors: HashMap<EventType, Box<dyn CommandExecutor<Ctx, dyn Command>>>,
}

impl<Ctx> MultiCommandExecutor<Ctx> {
    /// Creates a new empty multi-command executor.
    pub fn new() -> Self {
        Self {
            executors: HashMap::new(),
        }
    }

    /// Returns the command types this executor handles.
    pub fn command_types(&self) -> Vec<EventType> {
        self.executors.keys().cloned().collect()
    }

    /// Executes a command by finding the right executor.
    pub async fn execute(
        &self,
        ctx: &Ctx,
        cmd: &dyn Command,
    ) -> Result<(), CommandExecutionError> {
        let cmd_type = cmd.event_type().clone();
        match self.executors.get(&cmd_type) {
            Some(executor) => executor.execute(ctx, cmd).await,
            None => Err(CommandExecutionError::NoExecutorFound {
                command_type: cmd_type.to_string(),
            }),
        }
    }
}

impl<Ctx> Default for MultiCommandExecutor<Ctx> {
    fn default() -> Self {
        Self::new()
    }
}
