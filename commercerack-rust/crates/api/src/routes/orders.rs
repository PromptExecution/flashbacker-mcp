use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use commercerack_order::Order;
use rust_decimal::Decimal;
use serde::Deserialize;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub mid: i32,
    pub orderid: String,
    pub customer: i32,
    pub order_total: Decimal,
}

#[derive(Deserialize)]
pub struct ListQuery {
    pub mid: i32,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    20
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<Json<Order>, StatusCode> {
    Order::create(&state.pool, req.mid, &req.orderid, req.customer, req.order_total)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get(
    State(state): State<AppState>,
    Path((mid, id)): Path<(i32, i32)>,
) -> Result<Json<Order>, StatusCode> {
    Order::get(&state.pool, mid, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    Order::list(&state.pool, query.mid, query.limit, query.offset)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
