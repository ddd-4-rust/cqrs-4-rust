//! In-process Axum end-to-end projection and HTTP test.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use cqrs_4_rust_test_axum::app::{AxumApp, AxumConfig};
use cqrs_4_rust_test_axum::generated::{PersonCreatedEvent, PersonId, PersonName};
use tower::ServiceExt;

#[tokio::test]
async fn projects_an_event_and_reads_it_over_http() {
    let config = AxumConfig::default();
    let event = PersonCreatedEvent::new(
        PersonId::new(),
        PersonName::new("Peter Parker").expect("valid name"),
    );
    assert!(config.view.handle(&event).await);

    let response = AxumApp::router(config)
        .oneshot(
            Request::builder()
                .uri(format!("/persons/{}", event.id()))
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), StatusCode::OK);
}
