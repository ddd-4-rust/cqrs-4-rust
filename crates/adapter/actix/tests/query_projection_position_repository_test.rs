//! Quarkus projection repository behavior checks for Actix.

use cqrs_4_rust_actix::QryProjectionPositionRepository;
use cqrs_4_rust_esc::{ProjectionPosition, ProjectionService};

#[tokio::test]
async fn reads_updates_and_resets_positions() {
    let repository = QryProjectionPositionRepository::new();
    let initial = repository
        .read_projection_position("streamId")
        .await
        .expect("read should succeed")
        .expect("Java contract returns position zero for missing rows");
    assert_eq!(initial.next_position, 0);

    repository
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
        repository
            .read_projection_position("streamId")
            .await
            .expect("read should succeed")
            .expect("position should exist")
            .next_position,
        4711
    );

    repository
        .reset_projection_position("streamId")
        .await
        .expect("reset should succeed");
    assert_eq!(
        repository
            .read_projection_position("streamId")
            .await
            .expect("read should succeed")
            .expect("position should exist")
            .next_position,
        0
    );
}
