//! Fixture corresponding to Jackson `MyIdFactory`.

#[derive(Debug, Clone, Copy, Default)]
struct MyIdFactory;

impl MyIdFactory {
    fn create(entity_type: &str, value: &str) -> Option<String> {
        matches!(entity_type, "A" | "B" | "C").then(|| format!("{entity_type} {value}"))
    }
}

#[test]
fn creates_only_registered_fixture_ids() {
    assert_eq!(MyIdFactory::create("C", "3").as_deref(), Some("C 3"));
    assert_eq!(MyIdFactory::create("unknown", "3"), None);
}
