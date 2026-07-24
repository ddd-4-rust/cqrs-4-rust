//! JSON comparison helper corresponding to Jackson `TestUtils`.

fn json_equal(left: &str, right: &str) -> bool {
    let left: serde_json::Value = serde_json::from_str(left).expect("left JSON should be valid");
    let right: serde_json::Value = serde_json::from_str(right).expect("right JSON should be valid");
    left == right
}

#[test]
fn compares_json_independent_of_object_field_order() {
    assert!(json_equal(
        r#"{"type":"OK","code":"A"}"#,
        r#"{"code":"A","type":"OK"}"#
    ));
}
