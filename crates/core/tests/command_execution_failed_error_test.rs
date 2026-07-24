//! Command execution error checks.

use cqrs_4_rust_core::CommandExecutionError;

#[test]
fn command_execution_failure_preserves_message() {
    let error = CommandExecutionError::CommandExecutionFailed {
        message: "boom".to_owned(),
    };
    assert_eq!(error.to_string(), "Command execution failed: boom");
}
