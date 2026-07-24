//! Fixture corresponding to Java `MyIdFactory`.

#[derive(Debug, Clone, Copy, Default)]
struct MyIdFactory;

impl MyIdFactory {
    fn create(entity_type: &str, value: &str) -> Option<String> {
        matches!(entity_type, "A" | "B" | "C").then(|| format!("{entity_type} {value}"))
    }
}

#[test]
fn creates_only_registered_fixture_ids() {
    assert_eq!(MyIdFactory::create("A", "1").as_deref(), Some("A 1"));
    assert_eq!(MyIdFactory::create("unknown", "1"), None);
}
