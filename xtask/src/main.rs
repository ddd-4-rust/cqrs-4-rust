//! Repository automation for cqrs-4-rust.

mod coverage;
mod migration;
mod quality;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let task = env::args().nth(1);
    let result = match task.as_deref() {
        Some("coverage") => coverage::run(),
        Some("migration-parity") => migration::run(),
        Some("quality") => quality::run(),
        _ => {
            eprintln!("usage: cargo xtask <coverage|migration-parity|quality>");
            return ExitCode::FAILURE;
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
