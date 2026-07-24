//! JPA-backed CQRS view (projection) with CRON scheduling.
//!
//! # Java 对应
//!
//! `org.fuin.cqrs4j.core.JpaView`
//!
//! Java source doc: *"Defines a unit that projects events read from the event store into
//! another representation. The view is updated regularly by using a scheduler and
//! the result will be stored using JPA."*

#![allow(clippy::doc_markdown)]

use crate::view::View;
use async_trait::async_trait;
use ddd_4_rust_core::event::Event;

/// Defines a unit that projects events read from the event store into another representation.
///
/// The view is updated regularly by using a scheduler; persistence is application-defined
/// (Java 默认经 JPA `EntityManager`；Rust 由实现自行管理存储，不再强制传入 EM）。
///
/// # Java 对应
///
/// `org.fuin.cqrs4j.core.JpaView extends View`
#[async_trait]
pub trait JpaView: View {
    /// Returns the CRON expression defining how often the view should be updated.
    ///
    /// # Java 对应
    ///
    /// `JpaView.getCron()`
    ///
    /// @return Spring Quartz CRON expression（Rust 侧由 `tokio-cron-scheduler` 解析，需含秒字段）.
    fn cron(&self) -> &str;

    /// Number of events to read and handle in one transaction.
    ///
    /// # Java 对应
    ///
    /// `JpaView.getChunkSize()` — defaults to `100`.
    ///
    /// @return Number of events (defaults to 100).
    fn chunk_size(&self) -> u32 {
        100
    }

    /// Events to handle by the view.
    ///
    /// # Java 对应
    ///
    /// `JpaView.handleEvents(EntityManager em, List<Event> events)`
    ///
    /// Rust 省略 `EntityManager` 参数：实现方可在自身持有仓储/连接，或在闭包内开启事务。
    ///
    /// @param events Events used to update the view.
    async fn handle_events(&self, events: &[Box<dyn Event>]) -> Result<(), JpaViewError>;
}

/// Errors during JPA view event handling.
///
/// # Java 对应
///
/// Java `handleEvents` 抛出的运行时/持久化异常；Rust 结构化为显式错误类型。
#[derive(Debug, thiserror::Error)]
pub enum JpaViewError {
    /// A projected entity was not found.
    #[error("Entity not found: {0}")]
    EntityNotFound(String),
    /// A persistence operation failed.
    #[error("Database error: {0}")]
    Database(String),
    /// Any other projection failure.
    #[error("{0}")]
    Other(String),
}
