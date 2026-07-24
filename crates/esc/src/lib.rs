//! CQRS-4-Rust ESC: Event Store Commons for CQRS projections.
//!
//! # Java 对应
//!
//! 1:1 translation of `cqrs-4-java` 模块 `esc`（`org.fuin.cqrs4j.esc`），
//! 并补充 View Manager 所需的投影管理 / 解码 / 共享 tick 引擎：
//!
//! | Rust | Java |
//! |---|---|
//! | [`ProjectionService`] | `org.fuin.cqrs4j.esc.ProjectionService` |
//! | [`SimpleJpaEventDispatcher`] | `org.fuin.cqrs4j.esc.SimpleJpaEventDispatcher` |
//! | [`ProjectionStreamId`] | `org.fuin.esc.api.ProjectionStreamId` |
//! | [`ProjectionAdmin`] | `org.fuin.esc.api.ProjectionAdminEventStore` |
//! | [`EventDecoder`] | `*JpaViewManager.asEvents(...)` 中的 `(Event) getData()` |
//! | [`ViewProjector`] | `*JpaViewManager.updateView/readStreamEvents/handleChunk` |

#![allow(clippy::doc_markdown)]

mod event_decoder;
mod jpa_event_dispatcher;
mod projection_admin;
mod projection_service;
mod projection_stream_id;
mod simple_jpa_event_dispatcher;
mod view_projector;

pub use event_decoder::{EventDecodeError, EventDecoder, RegistryEventDecoder, TypedDecodeFn};
pub use jpa_event_dispatcher::JpaEventDispatcher;
pub use projection_admin::{MemoryProjectionAdmin, ProjectionAdmin, ProjectionAdminError};
pub use projection_service::{ProjectionError, ProjectionPosition, ProjectionService};
pub use projection_stream_id::ProjectionStreamId;
pub use simple_jpa_event_dispatcher::SimpleJpaEventDispatcher;
pub use view_projector::{ManagedView, ViewProjector, ViewProjectorError};
