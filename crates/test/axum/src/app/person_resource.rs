//! Axum HTTP resource for person projections.

use crate::app::AxumConfig;
use crate::model::PersonEntity;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

/// `GET /persons/{id}` resource.
#[derive(Debug, Clone, Copy, Default)]
pub struct PersonResource;

impl PersonResource {
    /// Reads a projected person.
    ///
    /// # Errors
    ///
    /// Returns [`StatusCode::NOT_FOUND`] when the identifier has not been projected.
    pub async fn read(
        Path(id): Path<Uuid>,
        State(config): State<AxumConfig>,
    ) -> Result<Json<PersonEntity>, StatusCode> {
        config
            .store
            .read()
            .await
            .get(&id)
            .cloned()
            .map(Json)
            .ok_or(StatusCode::NOT_FOUND)
    }
}
