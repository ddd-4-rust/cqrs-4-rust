//! Projection stream identifier derived from a view name and event-type checksum.
//!
//! # Java 对应
//!
//! - 类型：[`org.fuin.esc.api.ProjectionStreamId`](https://github.com/fuinorg/esc)
//! - 构造用法（CQRS View Manager）：
//!   `new ProjectionStreamId(view.getName() + "-" + CqrsUtils.calculateAdler32Checksum(eventTypes))`
//!   见 `QuarkusJpaViewManager.ViewExt` / `SpringJpaViewManager.ViewExt` 构造函数。

#![allow(clippy::doc_markdown)]

use cqrs_4_rust_core::CqrsUtils;
use ddd_4_rust_core::EventType;
use ddd_4_rust_esc::StreamId;

/// Stable stream id for a CQRS projection: `{view_name}-{adler32}`.
///
/// # Java 对应
///
/// `org.fuin.esc.api.ProjectionStreamId`
///
/// Java View Manager 中的构造方式：
/// ```java
/// // QuarkusJpaViewManager.ViewExt / SpringJpaViewManager.ViewExt
/// final String name = delegate.getName() + "-" + CqrsUtils.calculateAdler32Checksum(eventTypes);
/// projectionStreamId = new ProjectionStreamId(name);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectionStreamId {
    /// Underlying projection stream name (`asString()` / `toString()` 语义).
    value: String,
}

impl ProjectionStreamId {
    /// Builds a projection stream id from the view name and subscribed event types.
    ///
    /// # Java 对应
    ///
    /// `ViewExt` 构造函数内：
    /// `new ProjectionStreamId(delegate.getName() + "-" + CqrsUtils.calculateAdler32Checksum(eventTypes))`
    ///
    /// # Panics
    ///
    /// Panics when `event_types` is empty (matches [`CqrsUtils::calculate_adler32_checksum`]
    /// / Java `CqrsUtils.calculateAdler32Checksum` 对空集合抛异常的契约).
    #[must_use]
    pub fn from_view(view_name: &str, event_types: &[EventType]) -> Self {
        // Java: CqrsUtils.calculateAdler32Checksum(eventTypes)
        let checksum = CqrsUtils::calculate_adler32_checksum(event_types);
        Self {
            value: format!("{view_name}-{checksum}"),
        }
    }

    /// Creates a projection stream id from an already-computed string.
    ///
    /// # Java 对应
    ///
    /// `new ProjectionStreamId(String name)`
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the underlying stream name.
    ///
    /// # Java 对应
    ///
    /// `ProjectionStreamId.asString()` / `toString()`
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Converts to the ESC [`StreamId`] used by [`ddd_4_rust_esc::EventStore`].
    ///
    /// # Java 对应
    ///
    /// `ProjectionStreamId` 在 Java 中本身实现/可作为 `StreamId` 传给
    /// `EventStore` / `ProjectionService`。
    #[must_use]
    pub fn as_stream_id(&self) -> StreamId {
        StreamId::new(self.value.clone())
    }
}

impl std::fmt::Display for ProjectionStreamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AsRef<str> for ProjectionStreamId {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
