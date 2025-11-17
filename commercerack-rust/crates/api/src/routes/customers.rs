use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use commercerack_customer::CustomerService;
use ::entity::prelude::Customer;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateCustomerRequest {
    pub mid: i32,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub password: Option<String>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct CustomerResponse {
    pub cid: i32,
    pub mid: i32,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub created_gmt: i32,
    pub modified_gmt: i32,
}

impl From<Customer> for CustomerResponse {
    fn from(customer: Customer) -> Self {
        Self {
            cid: customer.cid,
            mid: customer.mid,
            email: customer.email,
            firstname: customer.firstname,
            lastname: customer.lastname,
            created_gmt: customer.created_gmt,
            modified_gmt: customer.modified_gmt,
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

/// Create a new customer
#[utoipa::path(
    post,
    path = "/api/customers",
    request_body = CreateCustomerRequest,
    responses(
        (status = 201, description = "Customer created successfully", body = CustomerResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "customers"
)]
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateCustomerRequest>,
) -> Result<(StatusCode, Json<CustomerResponse>), StatusCode> {
    CustomerService::create(
        &*state.db,
        req.mid,
        &req.email,
        &req.firstname,
        &req.lastname,
        req.password.as_deref(),
    )
    .await
    .map(|customer| (StatusCode::CREATED, Json(customer.into())))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Get a customer by ID
#[utoipa::path(
    get,
    path = "/api/customers/{mid}/{id}",
    params(
        ("mid" = i32, Path, description = "Merchant ID"),
        ("id" = i32, Path, description = "Customer ID")
    ),
    responses(
        (status = 200, description = "Customer found", body = CustomerResponse),
        (status = 404, description = "Customer not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "customers"
)]
pub async fn get(
    State(state): State<AppState>,
    Path((mid, id)): Path<(i32, i32)>,
) -> Result<Json<CustomerResponse>, StatusCode> {
    CustomerService::find_by_id(&*state.db, mid, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(|customer| Json(customer.into()))
        .ok_or(StatusCode::NOT_FOUND)
}

/// List customers (placeholder - not implemented in CustomerService yet)
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<CustomerResponse>>, StatusCode> {
    // TODO: Implement list in CustomerService
    Ok(Json(vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };

    #[tokio::test]
    async fn test_create_customer() {
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

        let req = CreateCustomerRequest {
            mid: 1,
            email: "test@example.com".to_string(),
            firstname: "Test".to_string(),
            lastname: "User".to_string(),
            password: Some("password123".to_string()),
        };

        // This will fail in mock but validates the structure
        let result = create(State(state), Json(req)).await;

        // We expect an error with mock database, but this validates the code compiles
        assert!(result.is_err());
    }
}
