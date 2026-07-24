//! Public API architecture checks.

use cqrs_4_rust_core::{CommandExecutor, MultiCommandExecutor, ToResultCapable};

fn assert_send_sync<T: Send + Sync>() {}
fn accepts_executor<Executor: CommandExecutor<(), ()>>() {}
fn accepts_result_capable<Value: ToResultCapable<()>>() {}

#[test]
fn public_facade_exposes_core_contracts() {
    assert_send_sync::<MultiCommandExecutor<(), ()>>();
    let _ = accepts_executor::<NeverExecutor>;
    let _ = accepts_result_capable::<NeverResult>;
}

struct NeverExecutor;

#[async_trait::async_trait]
impl CommandExecutor<(), ()> for NeverExecutor {
    fn command_types(&self) -> std::collections::HashSet<ddd_4_rust_core::EventType> {
        std::collections::HashSet::new()
    }

    async fn execute(
        &self,
        _context: &(),
        _command: &dyn cqrs_4_rust_core::Command,
    ) -> Result<(), cqrs_4_rust_core::CommandExecutionError> {
        Ok(())
    }
}

struct NeverResult;

impl ToResultCapable<()> for NeverResult {
    fn to_result(&self) -> cqrs_4_rust_core::CqrsResult<()> {
        cqrs_4_rust_core::CqrsResult::ok()
    }
}
