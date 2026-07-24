//! Axum adapter public API architecture checks.

use cqrs_4_rust_axum::{
    AxumJpaViewManager, EventStoreConfig, QryProjectionPosition, QryProjectionService,
};

#[test]
fn exposes_all_spring_boot_production_responsibilities() {
    let names = [
        std::any::type_name::<AxumJpaViewManager>(),
        std::any::type_name::<EventStoreConfig>(),
        std::any::type_name::<QryProjectionPosition>(),
        std::any::type_name::<QryProjectionService>(),
    ];
    assert!(
        names
            .iter()
            .all(|name| name.starts_with("cqrs_4_rust_axum"))
    );
}
