//! Spring Boot projection-position entity migrated to Axum.

use cqrs_4_rust_esc::ProjectionPosition;

/// Stores the next event position to read for one projection stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QryProjectionPosition {
    stream_id: String,
    next_position: i64,
}

impl QryProjectionPosition {
    /// Java table name retained for persistence adapters.
    pub const TABLE_NAME: &'static str = "SPRING_QRY_PROJECTION_POS";

    /// Creates a persisted projection position.
    pub fn new(stream_id: impl Into<String>, next_position: i64) -> Self {
        Self {
            stream_id: stream_id.into(),
            next_position,
        }
    }

    /// Returns the projection stream identifier.
    pub fn stream_id(&self) -> &str {
        &self.stream_id
    }

    /// Returns the next event position to read.
    pub const fn next_position(&self) -> i64 {
        self.next_position
    }

    /// Updates the next event position to read.
    pub const fn set_next_position(&mut self, next_position: i64) {
        self.next_position = next_position;
    }
}

impl From<&QryProjectionPosition> for ProjectionPosition {
    fn from(position: &QryProjectionPosition) -> Self {
        Self {
            stream_id: position.stream_id.clone(),
            next_position: position.next_position,
        }
    }
}
