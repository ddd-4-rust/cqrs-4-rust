//! Fixture corresponding to Jackson `AId`.

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct AId(i64);

#[test]
fn preserves_a_id_value() {
    let original = AId(123);
    let json = serde_json::to_string(&original).expect("AId should serialize");
    assert_eq!(
        serde_json::from_str::<AId>(&json).expect("valid AId"),
        original
    );
}
