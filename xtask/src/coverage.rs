//! Coverage task.

use std::process::Command;

pub(crate) fn run() -> Result<(), String> {
    let status = Command::new("cargo")
        .args(["llvm-cov", "--workspace", "--all-features"])
        .status()
        .map_err(|error| format!("failed to start cargo llvm-cov: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("cargo llvm-cov failed with {status}"))
    }
}
