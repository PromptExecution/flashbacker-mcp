use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use commercerack_customer::Customer;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateCustomerRequest {
    pub mid: i32,
    pub email: String,
    pub password: Option<String>,
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
    Json(req): Json<CreateCustomerRequest>,
) -> Result<Json<Customer>, StatusCode> {
    Customer::create(&state.pool, req.mid, &req.email, req.password.as_deref())
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get(
    State(state): State<AppState>,
    Path((mid, id)): Path<(i32, i32)>,
) -> Result<Json<Customer>, StatusCode> {
    Customer::get(&state.pool, mid, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Customer>>, StatusCode> {
    Customer::list(&state.pool, query.mid, query.limit, query.offset)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
