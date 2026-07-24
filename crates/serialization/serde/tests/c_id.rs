//! Fixture corresponding to Jackson `CId`.

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct CId(i64);

#[test]
fn preserves_c_id_value() {
    let original = CId(2);
    let json = serde_json::to_string(&original).expect("CId should serialize");
    assert_eq!(
        serde_json::from_str::<CId>(&json).expect("valid CId"),
        original
    );
}
