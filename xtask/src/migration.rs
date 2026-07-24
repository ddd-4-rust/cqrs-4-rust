//! Java-to-Rust migration parity task.

use std::process::Command;

pub(crate) fn run() -> Result<(), String> {
    let status = Command::new("bash")
        .arg("tools/check_migration_parity.sh")
        .status()
        .map_err(|error| format!("failed to start migration parity check: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("migration parity check failed with {status}"))
    }
}
