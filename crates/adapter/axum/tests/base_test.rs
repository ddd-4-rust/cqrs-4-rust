//! Shared Spring-Boot-to-Axum mapping checks.

use cqrs_4_rust_axum::QryProjectionPosition;

#[test]
fn preserves_the_spring_projection_table_contract() {
    assert_eq!(
        QryProjectionPosition::TABLE_NAME,
        "SPRING_QRY_PROJECTION_POS"
    );
}
