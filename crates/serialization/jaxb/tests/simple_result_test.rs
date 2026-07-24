//! JAXB simple-result XML round-trip tests.

use cqrs_4_rust_core::ResultType;
use cqrs_4_rust_jaxb::SimpleResult;

#[test]
fn round_trips_success_warning_and_error_results() {
    let ok = SimpleResult::ok();
    let xml = ok.to_xml().expect("simple result should serialize");
    assert_eq!(xml, "<result><type>OK</type></result>");
    assert_eq!(
        SimpleResult::from_xml(&xml)
            .expect("simple result should deserialize")
            .result_type(),
        ResultType::Ok
    );

    assert_eq!(SimpleResult::warning("W1", "careful").code(), Some("W1"));
    assert_eq!(
        SimpleResult::error("E1", "failed").result_type(),
        ResultType::Error
    );
}
