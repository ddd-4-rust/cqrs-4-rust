//! JAXB fixture event matching the Java `ACreatedEvent` responsibility.

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "a-created-event")]
pub(crate) struct ACreatedEvent {
    pub(crate) name: String,
}

#[test]
fn uses_the_jaxb_root_element_name() {
    let xml = quick_xml::se::to_string(&ACreatedEvent {
        name: "alpha".to_owned(),
    })
    .expect("event should serialize");
    assert_eq!(xml, "<a-created-event><name>alpha</name></a-created-event>");
}
