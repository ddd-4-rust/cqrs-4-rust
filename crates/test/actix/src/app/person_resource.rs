//! Actix HTTP resource for person projections.

use crate::PersonStore;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

/// `GET /persons/{id}` resource.
#[derive(Debug, Clone, Copy, Default)]
pub struct PersonResource;

impl PersonResource {
    /// Registers the person endpoint.
    pub fn configure(config: &mut web::ServiceConfig) {
        config.route("/persons/{id}", web::get().to(Self::read));
    }

    async fn read(id: web::Path<Uuid>, store: web::Data<PersonStore>) -> HttpResponse {
        store.read().await.get(&id.into_inner()).map_or_else(
            || HttpResponse::NotFound().finish(),
            |person| HttpResponse::Ok().json(person),
        )
    }
}
