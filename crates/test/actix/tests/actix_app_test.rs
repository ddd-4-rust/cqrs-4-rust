//! In-process Actix end-to-end projection and HTTP test.

use actix_web::{App, http::StatusCode, test, web};
use cqrs_4_rust_actix::EventStoreConfig;
use cqrs_4_rust_test_actix::app::{ActixApp, ActixFactory};
use cqrs_4_rust_test_actix::generated::{PersonCreatedEvent, PersonId, PersonName};
use cqrs_4_rust_test_actix::model::AbstractPersonsView;

#[actix_web::test]
async fn projects_an_event_and_reads_it_over_http() {
    let factory = ActixFactory::new(&EventStoreConfig::default());
    let event = PersonCreatedEvent::new(
        PersonId::new(),
        PersonName::new("Peter Parker").expect("valid name"),
    );
    assert!(factory.view().handle(&event).await);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(factory.store()))
            .configure(ActixApp::configure),
    )
    .await;
    let request = test::TestRequest::get()
        .uri(&format!("/persons/{}", event.id()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::OK);
}
