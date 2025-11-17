//! Axum API server for CommerceRack

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use commercerack_cart::CartStore;
use commercerack_customer::Customer;
use commercerack_product::Product;
use commercerack_order::Order;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::{Arc, Mutex};

pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub cart_store: Arc<Mutex<CartStore>>,
}

/// Build the Axum router with all routes
pub fn app(pool: PgPool) -> Router {
    let cart_store = Arc::new(Mutex::new(CartStore::new()));
    let state = AppState { pool, cart_store: cart_store.clone() };

    Router::new()
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

    #[tokio::test]
    async fn test_health_check() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let app = app(pool);

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
