//! Decodes stored [`CommonEvent`] payloads into domain [`Event`] trait objects.
//!
//! # Java 对应
//!
//! Java View Manager 中通过强转完成：
//! ```java
//! // QuarkusJpaViewManager.asEvents / SpringJpaViewManager.asEvents
//! private List<Event> asEvents(List<CommonEvent> events) {
//!     return events.stream().map(event -> (Event) event.getData()).toList();
//! }
//! ```
//!
//! Rust 的 [`ddd_4_rust_esc::CommonEvent`] 将 `data` 存为 `Vec<u8>`，因此需要
//! [`EventDecoder`] 按事件类型反序列化为 `Box<dyn Event>`，语义对齐 Java 的
//! `getData()` 强转。

#![allow(clippy::doc_markdown)]

use async_trait::async_trait;
use ddd_4_rust_core::EventType;
use ddd_4_rust_core::event::Event;
use ddd_4_rust_esc::CommonEvent;
use std::collections::HashMap;
use std::sync::Arc;

/// Errors while decoding a stored event payload.
///
/// # Java 对应
///
/// Java 强转失败会抛 `ClassCastException`；Rust 以显式错误分支表达。
#[derive(Debug, thiserror::Error)]
pub enum EventDecodeError {
    /// No decoder is registered for the event type.
    #[error("no decoder registered for event type: {0}")]
    UnknownType(String),
    /// Payload bytes could not be converted into a domain event.
    #[error("decode failed for event type {event_type}: {message}")]
    DecodeFailed {
        /// Stored event type name (`CommonEvent.getType()` / `event_type` 字段).
        event_type: String,
        /// Underlying decode failure.
        message: String,
    },
}

/// Converts a [`CommonEvent`] into a boxed domain event.
///
/// # Java 对应
///
/// `QuarkusJpaViewManager.asEvents(List<CommonEvent>)` /
/// `SpringJpaViewManager.asEvents(List<CommonEvent>)` 中的
/// `(Event) event.getData()`。
#[async_trait]
pub trait EventDecoder: Send + Sync {
    /// Decodes one stored event.
    ///
    /// # Java 对应
    ///
    /// 单条 `(Event) commonEvent.getData()`。
    async fn decode(&self, common: &CommonEvent) -> Result<Box<dyn Event>, EventDecodeError>;
}

/// Typed decode function registered by event type name.
///
/// 等价于 Java 侧按事件类型选择 deserializer / 工厂后再强转为 `Event`。
pub type TypedDecodeFn = Arc<dyn Fn(&CommonEvent) -> Result<Box<dyn Event>, String> + Send + Sync>;

/// Registry-backed decoder keyed by [`EventType`] string.
///
/// # Java 对应
///
/// 无同名类；在 Java 中序列化框架（Jackson/JSONB）与 Event Store 类型注册表共同承担此职责。
/// Rust 用显式注册表替代，便于测试与进程内 `EventStore` 接入。
#[derive(Default)]
pub struct RegistryEventDecoder {
    /// Decoders keyed by event type name (`EventType.asString()`).
    decoders: HashMap<String, TypedDecodeFn>,
}

impl RegistryEventDecoder {
    /// Creates an empty decoder registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a typed decoder for one event type.
    ///
    /// # Java 对应
    ///
    /// 对齐于向 Event Store / Serde 模块注册某一 `EventType` 的反序列化器。
    pub fn register<F>(&mut self, event_type: &EventType, decoder: F)
    where
        F: Fn(&CommonEvent) -> Result<Box<dyn Event>, String> + Send + Sync + 'static,
    {
        self.decoders
            .insert(event_type.as_str().to_owned(), Arc::new(decoder));
    }

    /// Registers a typed decoder using a type name string.
    ///
    /// # Java 对应
    ///
    /// 同 [`Self::register`]，入参为 `EventType.asString()` / `TypeName` 字符串形式。
    pub fn register_str<F>(&mut self, event_type: &str, decoder: F)
    where
        F: Fn(&CommonEvent) -> Result<Box<dyn Event>, String> + Send + Sync + 'static,
    {
        self.decoders
            .insert(event_type.to_owned(), Arc::new(decoder));
    }
}

#[async_trait]
impl EventDecoder for RegistryEventDecoder {
    /// See [`EventDecoder::decode`].
    async fn decode(&self, common: &CommonEvent) -> Result<Box<dyn Event>, EventDecodeError> {
        let decoder = self
            .decoders
            .get(&common.event_type)
            .ok_or_else(|| EventDecodeError::UnknownType(common.event_type.clone()))?;
        // Java: (Event) commonEvent.getData()
        decoder(common).map_err(|message| EventDecodeError::DecodeFailed {
            event_type: common.event_type.clone(),
            message,
        })
    }
}
