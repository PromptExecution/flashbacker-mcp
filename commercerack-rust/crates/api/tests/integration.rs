//! Integration tests for CommerceRack API
//! Simple tests using MockDatabase

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use commercerack_api::app;
use sea_orm::{DatabaseBackend, MockDatabase};
use tower::ServiceExt;

#[tokio::test]
async fn test_health_endpoint() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let app = app(db);

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_swagger_ui_available() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let app = app(db);

    let request = Request::builder()
        .uri("/swagger-ui/")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_rapidoc_available() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let app = app(db);

    let request = Request::builder()
        .uri("/rapidoc")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
