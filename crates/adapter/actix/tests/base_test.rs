//! Shared Quarkus-to-Actix mapping checks.

use cqrs_4_rust_actix::QryProjectionPosition;

#[test]
fn preserves_the_quarkus_projection_table_contract() {
    assert_eq!(
        QryProjectionPosition::TABLE_NAME,
        "QUARKUS_QRY_PROJECTION_POS"
    );
}
