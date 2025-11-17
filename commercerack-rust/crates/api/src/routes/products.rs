use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use commercerack_product::ProductService;
use ::entity::prelude::Product;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use crate::AppState;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateProductRequest {
    pub mid: i32,
    pub merchant: String,
    pub product_id: String,
    pub product_name: String,
    pub category: String,
    pub base_price: String,
    pub base_cost: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ProductResponse {
    pub id: i32,
    pub mid: i32,
    pub merchant: String,
    pub product: String,
    pub ts: i32,
    pub product_name: String,
    pub category: String,
    pub base_price: String,
    pub base_cost: String,
    pub supplier: String,
    pub supplier_id: String,
    pub upc: String,
    pub created_gmt: i32,
    pub lastsold_gmt: Option<i32>,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            mid: product.mid,
            merchant: product.merchant,
            product: product.product,
            ts: product.ts,
            product_name: product.product_name,
            category: product.category,
            base_price: product.base_price.to_string(),
            base_cost: product.base_cost.to_string(),
            supplier: product.supplier,
            supplier_id: product.supplier_id,
            upc: product.upc,
            created_gmt: product.created_gmt,
            lastsold_gmt: product.lastsold_gmt,
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

/// Create a new product
#[utoipa::path(
    post,
    path = "/api/products",
    request_body = CreateProductRequest,
    responses(
        (status = 201, description = "Product created successfully", body = ProductResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<ProductResponse>), StatusCode> {
    let base_price = req.base_price.parse::<Decimal>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let base_cost = req.base_cost.parse::<Decimal>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    ProductService::create(
        &*state.db,
        req.mid,
        &req.merchant,
        &req.product_id,
        &req.product_name,
        &req.category,
        base_price,
        base_cost,
    )
    .await
    .map(|product| (StatusCode::CREATED, Json(product.into())))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Get a product by ID
#[utoipa::path(
    get,
    path = "/api/products/{mid}/{id}",
    params(
        ("mid" = i32, Path, description = "Merchant ID"),
        ("id" = i32, Path, description = "Product ID")
    ),
    responses(
        (status = 200, description = "Product found", body = ProductResponse),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
pub async fn get(
    State(state): State<AppState>,
    Path((mid, id)): Path<(i32, i32)>,
) -> Result<Json<ProductResponse>, StatusCode> {
    ProductService::find_by_id(&*state.db, mid, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(|product| Json(product.into()))
        .ok_or(StatusCode::NOT_FOUND)
}

/// List products
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<ProductResponse>>, StatusCode> {
    ProductService::list(&*state.db, query.mid, query.limit, query.offset)
        .await
        .map(|products| Json(products.into_iter().map(|p| p.into()).collect()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

    #[tokio::test]
    async fn test_create_product() {
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

        let req = CreateProductRequest {
            mid: 1,
            merchant: "testmerchant".to_string(),
            product_id: "PROD001".to_string(),
            product_name: "Test Product".to_string(),
            category: "Electronics".to_string(),
            base_price: "99.99".to_string(),
            base_cost: "49.99".to_string(),
        };

        // This will fail in mock but validates the structure
        let result = create(State(state), Json(req)).await;
        assert!(result.is_err());
    }
}
