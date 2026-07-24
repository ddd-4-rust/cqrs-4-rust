//! Shared implementation for executors that route multiple command types.

use crate::{Command, CommandExecutionError, CommandExecutor};
use ddd_4_rust_core::EventType;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Routes commands to exactly one registered executor per event type.
pub struct AbstractMultiCommandExecutor<Context, Output> {
    command_executors: HashMap<EventType, Arc<dyn CommandExecutor<Context, Output>>>,
}

impl<Context, Output> AbstractMultiCommandExecutor<Context, Output> {
    /// Creates a router with at least one executor and no duplicate command type.
    ///
    /// # Errors
    ///
    /// Returns [`CommandExecutionError::InvalidExecutorConfiguration`] when the
    /// list is empty or more than one executor handles the same command type.
    pub fn try_new(
        command_executors: Vec<Arc<dyn CommandExecutor<Context, Output>>>,
    ) -> Result<Self, CommandExecutionError> {
        if command_executors.is_empty() {
            return Err(CommandExecutionError::InvalidExecutorConfiguration {
                message: "command_executors cannot be empty".to_owned(),
            });
        }

        let mut executors_by_type = HashMap::new();
        for executor in command_executors {
            for command_type in executor.command_types() {
                if executors_by_type
                    .insert(command_type.clone(), Arc::clone(&executor))
                    .is_some()
                {
                    return Err(CommandExecutionError::InvalidExecutorConfiguration {
                        message: format!(
                            "multiple executors are registered for command type {command_type}"
                        ),
                    });
                }
            }
        }

        Ok(Self {
            command_executors: executors_by_type,
        })
    }

    /// Returns all command types handled by the router.
    pub fn command_types(&self) -> HashSet<EventType> {
        self.command_executors.keys().cloned().collect()
    }

    /// Routes a command to its registered executor.
    ///
    /// # Errors
    ///
    /// Returns [`CommandExecutionError::NoExecutorFound`] for an unknown command,
    /// or propagates the selected executor's error.
    pub async fn execute(
        &self,
        context: &Context,
        command: &dyn Command,
    ) -> Result<Output, CommandExecutionError> {
        let command_type = command.event_type();
        let executor = self.command_executors.get(command_type).ok_or_else(|| {
            CommandExecutionError::NoExecutorFound {
                command_type: command_type.to_string(),
            }
        })?;
        executor.execute(context, command).await
    }
}
