//! Fixture corresponding to Jackson `BId`.

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct BId(i64);

#[test]
fn preserves_b_id_value() {
    let original = BId(1);
    let json = serde_json::to_string(&original).expect("BId should serialize");
    assert_eq!(
        serde_json::from_str::<BId>(&json).expect("valid BId"),
        original
    );
}
