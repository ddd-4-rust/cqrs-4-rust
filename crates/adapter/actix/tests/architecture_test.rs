//! Actix adapter public API architecture checks.

use cqrs_4_rust_actix::{
    ActixJpaViewManager, EventStoreConfig, QryProjectionPosition, QryProjectionPositionRepository,
};

#[test]
fn exposes_all_quarkus_production_responsibilities() {
    let names = [
        std::any::type_name::<ActixJpaViewManager>(),
        std::any::type_name::<EventStoreConfig>(),
        std::any::type_name::<QryProjectionPosition>(),
        std::any::type_name::<QryProjectionPositionRepository>(),
    ];
    assert!(
        names
            .iter()
            .all(|name| name.starts_with("cqrs_4_rust_actix"))
    );
}
