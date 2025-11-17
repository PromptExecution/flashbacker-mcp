//! Customer address management (placeholder - to be implemented with SeaORM)
//!
//! TODO: Implement CustomerAddress entity and service layer
//! For now, this is a stub to allow compilation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAddress {
    pub id: i32,
    pub cid: i32,
    pub mid: i32,
    pub label: String,
    pub firstname: String,
    pub lastname: String,
    pub company: String,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
    pub phone: String,
}

// TODO: Implement AddressService with SeaORM
// pub struct AddressService;
//
// impl AddressService {
//     pub async fn create(db: &DatabaseConnection, addr: CustomerAddress) -> Result<CustomerAddress> { ... }
//     pub async fn get_by_customer(db: &DatabaseConnection, mid: i32, cid: i32) -> Result<Vec<CustomerAddress>> { ... }
//     pub async fn delete(db: &DatabaseConnection, mid: i32, id: i32) -> Result<()> { ... }
// }
