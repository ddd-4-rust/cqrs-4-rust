//! Fixture corresponding to Jackson `Invoice`.

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Invoice {
    id: String,
}

#[test]
fn preserves_invoice_wire_shape() {
    let invoice = Invoice {
        id: "I-0123456".to_owned(),
    };
    let value = serde_json::to_value(&invoice).expect("invoice should serialize");
    assert_eq!(value, serde_json::json!({"id": "I-0123456"}));
}
