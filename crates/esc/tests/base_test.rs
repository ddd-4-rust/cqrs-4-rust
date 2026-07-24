//! Projection position contract checks.

use cqrs_4_rust_esc::ProjectionPosition;

#[test]
fn projection_position_keeps_stream_and_next_position() {
    let position = ProjectionPosition {
        stream_id: "persons".to_owned(),
        next_position: 42,
    };
    assert_eq!(position.stream_id, "persons");
    assert_eq!(position.next_position, 42);
}
