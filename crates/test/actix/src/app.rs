//! Actix application wiring migrated from Quarkus CDI producers.

mod actix_app;
mod actix_factory;
mod kurrent_db_wrapper;
mod person_resource;

pub use actix_app::ActixApp;
pub use actix_factory::ActixFactory;
pub use kurrent_db_wrapper::KurrentDbWrapper;
pub use person_resource::PersonResource;
