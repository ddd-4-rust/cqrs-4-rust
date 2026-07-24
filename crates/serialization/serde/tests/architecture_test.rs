//! Public Serde facade checks.

use cqrs_4_rust_serde::{Cqrs4SerdeModule, DataResult};

#[test]
fn public_facade_exposes_wire_format_types() {
    assert_eq!(Cqrs4SerdeModule::NAME, "Cqrs4SerdeModule");
    assert!(DataResult::<()>::ok(None).data().is_none());
}
