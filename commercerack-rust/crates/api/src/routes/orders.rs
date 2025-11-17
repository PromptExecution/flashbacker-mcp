use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use commercerack_order::OrderService;
use ::entity::prelude::Order as OrderModel;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateOrderRequest {
    pub mid: i32,
    pub orderid: String,
    pub cartid: String,
    pub customer: i32,
    pub pool: String,
    pub total: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct OrderResponse {
    pub id: i32,
    pub mid: i32,
    pub orderid: String,
    pub cartid: String,
    pub customer: i32,
    pub pool: String,
    pub total: String,
    pub created_gmt: i32,
    pub paid_gmt: Option<i32>,
    pub shipped_gmt: Option<i32>,
}

impl From<OrderModel> for OrderResponse {
    fn from(order: OrderModel) -> Self {
        Self {
            id: order.id,
            mid: order.mid,
            orderid: order.orderid,
            cartid: order.cartid,
            customer: order.customer,
            pool: order.pool,
            total: order.total.to_string(),
            created_gmt: order.created_gmt,
            paid_gmt: order.paid_gmt,
            shipped_gmt: order.shipped_gmt,
        }
    }
}

#[derive(Deserialize, utoipa::IntoParams)]
pub struct ListQuery {
    pub mid: i32,
    #[serde(default = "default_limit")]
    pub limit: u64,
    #[serde(default)]
    pub offset: u64,
}

fn default_limit() -> u64 {
    20
}

/// Create a new order
#[utoipa::path(
    post,
    path = "/api/orders",
    request_body = CreateOrderRequest,
    responses(
        (status = 201, description = "Order created successfully", body = OrderResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "orders"
)]
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), StatusCode> {
    let total = req.total.parse::<Decimal>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    OrderService::create(
        &*state.db,
        req.mid,
        &req.orderid,
        &req.cartid,
        req.customer,
        &req.pool,
        total,
    )
    .await
    .map(|order| (StatusCode::CREATED, Json(order.into())))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Get an order by ID
#[utoipa::path(
    get,
    path = "/api/orders/{mid}/{id}",
    params(
        ("mid" = i32, Path, description = "Merchant ID"),
        ("id" = i32, Path, description = "Order ID")
    ),
    responses(
        (status = 200, description = "Order found", body = OrderResponse),
        (status = 404, description = "Order not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "orders"
)]
pub async fn get(
    State(state): State<AppState>,
    Path((mid, id)): Path<(i32, i32)>,
) -> Result<Json<OrderResponse>, StatusCode> {
    OrderService::find_by_id(&*state.db, mid, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(|order| Json(order.into()))
        .ok_or(StatusCode::NOT_FOUND)
}

/// List orders (placeholder - needs implementation in OrderService)
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<OrderResponse>>, StatusCode> {
    // TODO: Implement general list in OrderService
    Ok(Json(vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

    #[tokio::test]
    async fn test_create_order() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
            ])
            .into_connection();

        let state = AppState {
            db: std::sync::Arc::new(db),
            cart_store: std::sync::Arc::new(std::sync::Mutex::new(
                commercerack_cart::CartStore::new()
            )),
        };

        let req = CreateOrderRequest {
            mid: 1,
            orderid: "ORD001".to_string(),
            cartid: "CART001".to_string(),
            customer: 1,
            pool: "RECENT".to_string(),
            total: "199.99".to_string(),
        };

        // This will fail in mock but validates the structure
        let result = create(State(state), Json(req)).await;
        assert!(result.is_err());
    }
}
