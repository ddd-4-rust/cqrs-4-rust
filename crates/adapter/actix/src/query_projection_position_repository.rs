//! Quarkus projection repository migrated to an Actix-managed service.
//!
//! # Java 对应
//!
//! - 契约：`org.fuin.cqrs4j.esc.ProjectionService`
//! - Quarkus 适配侧常见实现名：`QryProjectionPositionRepository`（example / quarkus 查询模块）
//!
//! 本类型为进程内实现，用于本地与测试；生产可替换为 JPA/Panache 持久化实现。

#![allow(clippy::doc_markdown)]

use crate::QryProjectionPosition;
use async_trait::async_trait;
use cqrs_4_rust_esc::{ProjectionError, ProjectionPosition, ProjectionService};
use std::collections::BTreeMap;
use tokio::sync::RwLock;

/// Persists Quarkus-compatible projection positions for the Actix adapter.
///
/// # Java 对应
///
/// 实现 `org.fuin.cqrs4j.esc.ProjectionService` 的 Quarkus 查询侧仓储
/// （命名对齐 example 中的 `QryProjectionPositionRepository`）。
#[derive(Debug, Default)]
pub struct QryProjectionPositionRepository {
    /// In-memory map of stream id → last position（Java 通常落库）.
    positions: RwLock<BTreeMap<String, QryProjectionPosition>>,
}

impl QryProjectionPositionRepository {
    /// Creates an empty repository.
    ///
    /// # Java 对应
    ///
    /// CDI 注入后的空表 / 空实体状态。
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ProjectionService for QryProjectionPositionRepository {
    /// Sets the stored position of the projection to the start position.
    ///
    /// # Java 对应
    ///
    /// `ProjectionService.resetProjectionPosition(StreamId)`
    async fn reset_projection_position(&self, stream_id: &str) -> Result<(), ProjectionError> {
        if let Some(position) = self.positions.write().await.get_mut(stream_id) {
            position.set_next_position(0);
        }
        Ok(())
    }

    /// Reads the position that was read last time.
    ///
    /// # Java 对应
    ///
    /// `ProjectionService.readProjectionPosition(StreamId)` → `Long`
    ///
    /// 尚无记录时返回 `next_position = 0`（与 View Manager 从流头开始读一致）。
    async fn read_projection_position(
        &self,
        stream_id: &str,
    ) -> Result<Option<ProjectionPosition>, ProjectionError> {
        let positions = self.positions.read().await;
        Ok(Some(positions.get(stream_id).map_or_else(
            || ProjectionPosition {
                stream_id: stream_id.to_owned(),
                next_position: 0,
            },
            ProjectionPosition::from,
        )))
    }

    /// Updates the position to read next time.
    ///
    /// # Java 对应
    ///
    /// `ProjectionService.updateProjectionPosition(StreamId, Long nextEventNumber)`
    ///
    /// 调用点：`QuarkusJpaViewManager.handleChunk` /
    /// `ViewProjector::tick` 处理完 chunk 之后。
    async fn update_projection_position(
        &self,
        stream_id: &str,
        position: &ProjectionPosition,
    ) -> Result<(), ProjectionError> {
        self.positions.write().await.insert(
            stream_id.to_owned(),
            QryProjectionPosition::new(stream_id, position.next_position),
        );
        Ok(())
    }
}
