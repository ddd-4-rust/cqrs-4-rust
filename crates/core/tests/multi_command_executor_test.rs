//! Multi-command routing checks.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cqrs_4_rust_core::{Command, CommandExecutionError, CommandExecutor, MultiCommandExecutor};
use ddd_4_rust_core::{Event, EventId, EventType};
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

struct TestCommand {
    event_id: EventId,
    event_type: EventType,
    timestamp: DateTime<Utc>,
}

impl TestCommand {
    fn new(event_type: &str) -> Self {
        Self {
            event_id: EventId::new(),
            event_type: EventType::new(event_type).unwrap(),
            timestamp: Utc::now(),
        }
    }
}

impl Event for TestCommand {
    fn event_id(&self) -> &EventId {
        &self.event_id
    }

    fn event_type(&self) -> &EventType {
        &self.event_type
    }

    fn event_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    fn correlation_id(&self) -> Option<&EventId> {
        None
    }

    fn causation_id(&self) -> Option<&EventId> {
        None
    }
}

impl Command for TestCommand {}

struct CountingExecutor {
    event_type: EventType,
    count: AtomicU64,
}

#[async_trait]
impl CommandExecutor<(), u64> for CountingExecutor {
    fn command_types(&self) -> HashSet<EventType> {
        HashSet::from([self.event_type.clone()])
    }

    async fn execute(
        &self,
        _context: &(),
        _command: &dyn Command,
    ) -> Result<u64, CommandExecutionError> {
        Ok(self.count.fetch_add(1, Ordering::SeqCst) + 1)
    }
}

fn executor(event_type: &str) -> Arc<dyn CommandExecutor<(), u64>> {
    Arc::new(CountingExecutor {
        event_type: EventType::new(event_type).unwrap(),
        count: AtomicU64::new(0),
    })
}

#[tokio::test]
async fn routes_commands_and_returns_generic_output() {
    let router = MultiCommandExecutor::try_new(vec![executor("MyCommand")]).unwrap();
    assert_eq!(
        router
            .execute(&(), &TestCommand::new("MyCommand"))
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        router
            .execute(&(), &TestCommand::new("MyCommand"))
            .await
            .unwrap(),
        2
    );
}

#[test]
fn rejects_empty_and_duplicate_executor_sets() {
    assert!(matches!(
        MultiCommandExecutor::<(), u64>::try_new(vec![]),
        Err(CommandExecutionError::InvalidExecutorConfiguration { .. })
    ));

    let shared = executor("MyCommand");
    assert!(matches!(
        MultiCommandExecutor::try_new(vec![Arc::clone(&shared), shared]),
        Err(CommandExecutionError::InvalidExecutorConfiguration { .. })
    ));
}

#[tokio::test]
async fn rejects_unknown_command_type() {
    let router = MultiCommandExecutor::try_new(vec![executor("Known")]).unwrap();
    assert!(matches!(
        router.execute(&(), &TestCommand::new("Unknown")).await,
        Err(CommandExecutionError::NoExecutorFound { .. })
    ));
}
