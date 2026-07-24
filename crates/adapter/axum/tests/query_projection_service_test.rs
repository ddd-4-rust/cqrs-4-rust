//! Spring projection service behavior checks for Axum.

use cqrs_4_rust_axum::QryProjectionService;
use cqrs_4_rust_esc::{ProjectionPosition, ProjectionService};

#[tokio::test]
async fn reads_updates_and_resets_positions() {
    let service = QryProjectionService::new();
    let initial = service
        .read_projection_position("streamId")
        .await
        .expect("read should succeed")
        .expect("Java contract returns position zero for missing rows");
    assert_eq!(initial.next_position, 0);

    service
        .update_projection_position(
            "streamId",
            &ProjectionPosition {
                stream_id: "streamId".to_owned(),
                next_position: 4711,
            },
        )
        .await
        .expect("update should succeed");
    assert_eq!(
        service
            .read_projection_position("streamId")
            .await
            .expect("read should succeed")
            .expect("position should exist")
            .next_position,
        4711
    );

    service
        .reset_projection_position("streamId")
        .await
        .expect("reset should succeed");
    assert_eq!(
        service
            .read_projection_position("streamId")
            .await
            .expect("read should succeed")
            .expect("position should exist")
            .next_position,
        0
    );
}
