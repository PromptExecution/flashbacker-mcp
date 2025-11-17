//! Axum API server for CommerceRack with SeaORM, JWT, and OpenAPI

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use commercerack_cart::CartStore;
use sea_orm::DatabaseConnection;
use std::sync::{Arc, Mutex};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_rapidoc::RapiDoc;

pub mod auth;
pub mod routes;

/// API Documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        routes::customers::create,
        routes::customers::get,
        routes::products::create,
        routes::products::get,
        routes::orders::create,
        routes::orders::get,
    ),
    components(
        schemas(
            auth::Claims,
            routes::customers::CreateCustomerRequest,
            routes::customers::CustomerResponse,
            routes::products::CreateProductRequest,
            routes::products::ProductResponse,
            routes::orders::CreateOrderRequest,
            routes::orders::OrderResponse,
        )
    ),
    tags(
        (name = "customers", description = "Customer management endpoints"),
        (name = "products", description = "Product catalog endpoints"),
        (name = "orders", description = "Order management endpoints"),
        (name = "cart", description = "Shopping cart endpoints"),
    ),
    security(
        ("bearer" = [])
    )
)]
pub struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub cart_store: Arc<Mutex<CartStore>>,
}

/// Build the Axum router with all routes and OpenAPI documentation
pub fn app(db: DatabaseConnection) -> Router {
    let cart_store = Arc::new(Mutex::new(CartStore::new()));
    let state = AppState {
        db: Arc::new(db),
        cart_store: cart_store.clone(),
    };

    Router::new()
        // OpenAPI documentation
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        // Customer routes
        .route("/api/customers", post(routes::customers::create))
        .route("/api/customers/:mid/:id", get(routes::customers::get))
        .route("/api/customers", get(routes::customers::list))
        // Product routes
        .route("/api/products", post(routes::products::create))
        .route("/api/products/:mid/:id", get(routes::products::get))
        .route("/api/products", get(routes::products::list))
        // Order routes
        .route("/api/orders", post(routes::orders::create))
        .route("/api/orders/:mid/:id", get(routes::orders::get))
        .route("/api/orders", get(routes::orders::list))
        // Cart routes
        .route("/api/carts", post(routes::cart::create_cart))
        .route("/api/carts/:cart_id", get(routes::cart::get_cart))
        .route("/api/carts/:cart_id/items", post(routes::cart::add_item))
        .route("/api/carts/:cart_id/items/:sku", put(routes::cart::update_quantity))
        .route("/api/carts/:cart_id/items/:sku", delete(routes::cart::remove_item))
        .route("/api/carts/:cart_id/clear", post(routes::cart::clear_cart))
        .route("/api/carts/:cart_id", delete(routes::cart::delete_cart))
        // Health check
        .route("/health", get(health_check))
        .with_state(state)
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
async fn health_check() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn test_health_check() {
        // Use mock database for testing
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .into_connection();

        let app = app(db);

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_swagger_ui_available() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .into_connection();

        let app = app(db);

        let response = app
            .oneshot(Request::builder().uri("/swagger-ui/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
