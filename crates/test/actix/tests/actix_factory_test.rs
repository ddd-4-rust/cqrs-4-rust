//! Actix factory wiring tests.

use cqrs_4_rust_actix::EventStoreConfig;
use cqrs_4_rust_test_actix::app::ActixFactory;

#[test]
fn wires_kurrent_and_projection_state() {
    let config = EventStoreConfig::default();
    let factory = ActixFactory::new(&config);
    assert_eq!(factory.kurrent().endpoint(), "localhost:2113");
    assert!(factory.kurrent().is_active());
    assert!(
        factory
            .store()
            .try_read()
            .expect("unlocked store")
            .is_empty()
    );
}
