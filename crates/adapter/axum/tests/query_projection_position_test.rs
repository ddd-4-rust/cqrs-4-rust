//! Spring projection-position entity checks for Axum.

use cqrs_4_rust_axum::QryProjectionPosition;

#[test]
fn creates_reads_and_updates_a_position() {
    let mut position = QryProjectionPosition::new("streamId", 4711);
    assert_eq!(position.stream_id(), "streamId");
    assert_eq!(position.next_position(), 4711);
    position.set_next_position(4712);
    assert_eq!(position.next_position(), 4712);
}
