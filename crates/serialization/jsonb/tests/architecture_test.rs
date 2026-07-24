//! Public surface checks for the eight Java JSON-B production responsibilities.

use cqrs_4_rust_jsonb::{
    AbstractAggregateCommand, AbstractCommand, AbstractResult, DataResult, DataResultJsonbAdapter,
    JandexJsonbRegistry, JsonbRegistry, SimpleResult,
};

#[test]
fn exposes_all_jsonb_production_responsibilities() {
    let _: Option<AbstractAggregateCommand<ddd_4_rust_core::AggregateRootUuid>> = None;
    let _: Option<AbstractCommand> = None;
    let _: Option<AbstractResult<()>> = None;
    let _: Option<DataResult<()>> = None;
    let _ = DataResultJsonbAdapter;
    let registry = JandexJsonbRegistry::new();
    let _: &dyn JsonbRegistry = &registry;
    let _: SimpleResult = SimpleResult::ok();
}
