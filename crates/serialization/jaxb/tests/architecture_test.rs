//! JAXB crate public API architecture checks.

use cqrs_4_rust_jaxb::{
    AbstractAggregateCommand, AbstractCommand, AbstractResult, DataResult, SimpleResult,
};

#[test]
fn public_facade_exposes_each_java_production_responsibility() {
    let names = [
        std::any::type_name::<AbstractAggregateCommand<()>>(),
        std::any::type_name::<AbstractCommand>(),
        std::any::type_name::<AbstractResult>(),
        std::any::type_name::<DataResult<()>>(),
        std::any::type_name::<SimpleResult>(),
    ];
    assert!(
        names
            .iter()
            .all(|name| name.starts_with("cqrs_4_rust_jaxb"))
    );
}
