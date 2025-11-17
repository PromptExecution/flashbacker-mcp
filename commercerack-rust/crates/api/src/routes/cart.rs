use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use commercerack_cart::{Cart, CartItem};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Deserialize)]
pub struct AddItemRequest {
    pub sku: String,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: String, // Decimal as string from JSON
}

#[derive(Deserialize)]
pub struct UpdateQuantityRequest {
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct CartResponse {
    pub cart_id: String,
    pub items: Vec<CartItem>,
    pub subtotal: Decimal,
    pub item_count: i32,
}

impl From<&Cart> for CartResponse {
    fn from(cart: &Cart) -> Self {
        Self {
            cart_id: cart.cart_id.clone(),
            items: cart.items.clone(),
            subtotal: cart.subtotal(),
            item_count: cart.item_count(),
        }
    }
}

/// Create a new cart
pub async fn create_cart(
    State(state): State<AppState>,
) -> Result<Json<CartResponse>, StatusCode> {
    let mut store = state.cart_store.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let cart_id = store.create_cart();
    let cart = store
        .get_cart(&cart_id)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(CartResponse::from(cart)))
}

/// Get cart by ID
pub async fn get_cart(
    State(state): State<AppState>,
    Path(cart_id): Path<String>,
) -> Result<Json<CartResponse>, StatusCode> {
    let store = state.cart_store.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let cart = store.get_cart(&cart_id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(CartResponse::from(cart)))
}

/// Add item to cart
pub async fn add_item(
    State(state): State<AppState>,
    Path(cart_id): Path<String>,
    Json(req): Json<AddItemRequest>,
) -> Result<Json<CartResponse>, StatusCode> {
    let unit_price = req
        .unit_price
        .parse::<Decimal>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut store = state.cart_store.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let cart = store
        .get_cart_mut(&cart_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    cart.add_item(req.sku, req.product_name, req.quantity, unit_price);

    Ok(Json(CartResponse::from(&*cart)))
}

/// Update item quantity
pub async fn update_quantity(
    State(state): State<AppState>,
    Path((cart_id, sku)): Path<(String, String)>,
    Json(req): Json<UpdateQuantityRequest>,
) -> Result<Json<CartResponse>, StatusCode> {
    let mut store = state.cart_store.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let cart = store
        .get_cart_mut(&cart_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if !cart.update_quantity(&sku, req.quantity) {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(CartResponse::from(&*cart)))
}

/// Remove item from cart
pub async fn remove_item(
    State(state): State<AppState>,
    Path((cart_id, sku)): Path<(String, String)>,
) -> Result<Json<CartResponse>, StatusCode> {
    let mut store = state.cart_store.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let cart = store
        .get_cart_mut(&cart_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if !cart.remove_item(&sku) {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(CartResponse::from(&*cart)))
}

/// Clear all items from cart
pub async fn clear_cart(
    State(state): State<AppState>,
    Path(cart_id): Path<String>,
) -> Result<Json<CartResponse>, StatusCode> {
    let mut store = state.cart_store.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let cart = store
        .get_cart_mut(&cart_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    cart.clear();
    Ok(Json(CartResponse::from(&*cart)))
}

/// Delete cart
pub async fn delete_cart(
    State(state): State<AppState>,
    Path(cart_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut store = state.cart_store.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if store.delete_cart(&cart_id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
