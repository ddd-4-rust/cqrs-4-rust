//! Feature-gated facade checks.

#[test]
fn core_is_always_available() {
    let result = cqrs_4_rust::core::CqrsResult::<()>::ok();
    assert_eq!(result.result_type(), cqrs_4_rust::core::ResultType::Ok);
}

#[cfg(feature = "esc")]
#[test]
fn esc_is_available_when_enabled() {
    let _ = cqrs_4_rust::esc::SimpleJpaEventDispatcher::new();
}
