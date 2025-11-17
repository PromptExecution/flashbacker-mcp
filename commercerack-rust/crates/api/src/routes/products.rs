use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use commercerack_product::Product;
use serde::Deserialize;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub mid: i32,
    pub product_code: String,
    pub product_name: String,
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
    Json(req): Json<CreateProductRequest>,
) -> Result<Json<Product>, StatusCode> {
    Product::create(&state.pool, req.mid, &req.product_code, &req.product_name)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get(
    State(state): State<AppState>,
    Path((mid, id)): Path<(i32, i32)>,
) -> Result<Json<Product>, StatusCode> {
    Product::get(&state.pool, mid, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    Product::list(&state.pool, query.mid, query.limit, query.offset)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
