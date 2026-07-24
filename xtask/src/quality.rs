//! Workspace quality gates.

use std::process::Command;

pub(crate) fn run() -> Result<(), String> {
    run_cargo(&["fmt", "--all", "--", "--check"])?;
    run_cargo(&["check", "--workspace", "--all-targets", "--all-features"])?;
    run_cargo(&["test", "--workspace", "--all-targets", "--all-features"])?;
    run_cargo(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--all-features",
        "--",
        "-D",
        "warnings",
    ])?;
    run_cargo(&["doc", "--workspace", "--all-features", "--no-deps"])
}

fn run_cargo(arguments: &[&str]) -> Result<(), String> {
    let status = Command::new("cargo")
        .args(arguments)
        .status()
        .map_err(|error| format!("failed to start cargo {}: {error}", arguments.join(" ")))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "cargo {} failed with {status}",
            arguments.join(" ")
        ))
    }
}
