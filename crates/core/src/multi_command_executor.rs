//! Concrete multi-command executor.

use crate::{AbstractMultiCommandExecutor, Command, CommandExecutionError, CommandExecutor};
use ddd_4_rust_core::EventType;
use std::collections::HashSet;
use std::sync::Arc;

/// Public facade over [`AbstractMultiCommandExecutor`].
pub struct MultiCommandExecutor<Context, Output> {
    delegate: AbstractMultiCommandExecutor<Context, Output>,
}

impl<Context, Output> MultiCommandExecutor<Context, Output> {
    /// Creates a validated multi-command executor.
    ///
    /// # Errors
    ///
    /// Returns an invalid configuration error for an empty executor list or a
    /// duplicate command type.
    pub fn try_new(
        command_executors: Vec<Arc<dyn CommandExecutor<Context, Output>>>,
    ) -> Result<Self, CommandExecutionError> {
        Ok(Self {
            delegate: AbstractMultiCommandExecutor::try_new(command_executors)?,
        })
    }

    /// Returns all handled command types.
    pub fn command_types(&self) -> HashSet<EventType> {
        self.delegate.command_types()
    }

    /// Executes a command using the registered executor.
    ///
    /// # Errors
    ///
    /// Returns an unknown-command error or propagates the selected executor's error.
    pub async fn execute(
        &self,
        context: &Context,
        command: &dyn Command,
    ) -> Result<Output, CommandExecutionError> {
        self.delegate.execute(context, command).await
    }
}
