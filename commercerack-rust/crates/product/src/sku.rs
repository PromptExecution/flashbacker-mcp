//! SKU management (placeholder - to be implemented with SeaORM)
//!
//! TODO: Implement SKU entity and service layer
//! For now, this is a stub to allow compilation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SKU {
    pub id: i32,
    pub pid: i32,
    pub mid: i32,
    pub sku: String,
    pub title: String,
    pub price: rust_decimal::Decimal,
    pub cost: rust_decimal::Decimal,
    pub upc: String,
    pub inv_available: i32,
    pub qty_onshelf: i32,
}

// TODO: Implement SKUService with SeaORM
// pub struct SKUService;
//
// impl SKUService {
//     pub async fn create(db: &DatabaseConnection, sku: SKU) -> Result<SKU> { ... }
//     pub async fn find_by_id(db: &DatabaseConnection, mid: i32, id: i32) -> Result<Option<SKU>> { ... }
//     pub async fn find_by_product(db: &DatabaseConnection, mid: i32, pid: i32) -> Result<Vec<SKU>> { ... }
//     pub async fn update(db: &DatabaseConnection, sku: SKU) -> Result<SKU> { ... }
//     pub async fn delete(db: &DatabaseConnection, mid: i32, id: i32) -> Result<()> { ... }
// }
