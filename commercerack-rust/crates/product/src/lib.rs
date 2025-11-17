//! Product management module using SeaORM

use anyhow::Result;
use chrono::Utc;
use sea_orm::*;
use ::entity::prelude::*;
use rust_decimal::Decimal;

pub mod sku;

/// Product service for managing product operations
pub struct ProductService;

impl ProductService {
    /// Create new product
    pub async fn create(
        db: &DatabaseConnection,
        mid: i32,
        merchant: &str,
        product_id: &str,
        product_name: &str,
        category: &str,
        base_price: Decimal,
        base_cost: Decimal,
    ) -> Result<Product> {
        let now = Utc::now().timestamp() as i32;

        let product = ::entity::products::ActiveModel {
            mid: Set(mid),
            merchant: Set(merchant.to_string()),
            product: Set(product_id.to_string()),
            ts: Set(now),
            product_name: Set(product_name.to_string()),
            category: Set(category.to_string()),
            base_price: Set(base_price),
            base_cost: Set(base_cost),
            supplier: Set(String::new()),
            supplier_id: Set(String::new()),
            upc: Set(String::new()),
            created_gmt: Set(now),
            lastsold_gmt: Set(None),
            ..Default::default()
        };

        let result = product.insert(db).await?;
        Ok(result)
    }

    /// Find product by ID
    pub async fn find_by_id(
        db: &DatabaseConnection,
        mid: i32,
        id: i32,
    ) -> Result<Option<Product>> {
        let product = Products::find()
            .filter(::entity::products::Column::Mid.eq(mid))
            .filter(::entity::products::Column::Id.eq(id))
            .one(db)
            .await?;

        Ok(product)
    }

    /// Find product by merchant product ID
    pub async fn find_by_product_id(
        db: &DatabaseConnection,
        mid: i32,
        product_id: &str,
    ) -> Result<Option<Product>> {
        let product = Products::find()
            .filter(::entity::products::Column::Mid.eq(mid))
            .filter(::entity::products::Column::Product.eq(product_id))
            .one(db)
            .await?;

        Ok(product)
    }

    /// List products with pagination
    pub async fn list(
        db: &DatabaseConnection,
        mid: i32,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Product>> {
        let products = Products::find()
            .filter(::entity::products::Column::Mid.eq(mid))
            .order_by_asc(::entity::products::Column::ProductName)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?;

        Ok(products)
    }

    /// Update product
    pub async fn update(
        db: &DatabaseConnection,
        product: Product,
    ) -> Result<Product> {
        let mut active: ::entity::products::ActiveModel = product.into();
        active.ts = Set(Utc::now().timestamp() as i32);

        let result = active.update(db).await?;
        Ok(result)
    }

    /// Delete product
    pub async fn delete(
        db: &DatabaseConnection,
        mid: i32,
        id: i32,
    ) -> Result<()> {
        Products::delete_many()
            .filter(::entity::products::Column::Mid.eq(mid))
            .filter(::entity::products::Column::Id.eq(id))
            .exec(db)
            .await?;

        Ok(())
    }

    /// Update product price
    pub async fn update_price(
        db: &DatabaseConnection,
        mid: i32,
        id: i32,
        base_price: Decimal,
        base_cost: Option<Decimal>,
    ) -> Result<Product> {
        let product = Self::find_by_id(db, mid, id).await?
            .ok_or_else(|| anyhow::anyhow!("Product not found"))?;

        let mut active: ::entity::products::ActiveModel = product.into();
        active.base_price = Set(base_price);
        if let Some(cost) = base_cost {
            active.base_cost = Set(cost);
        }
        active.ts = Set(Utc::now().timestamp() as i32);

        let result = active.update(db).await?;
        Ok(result)
    }

    /// Mark product as sold
    pub async fn mark_sold(
        db: &DatabaseConnection,
        mid: i32,
        id: i32,
    ) -> Result<Product> {
        let product = Self::find_by_id(db, mid, id).await?
            .ok_or_else(|| anyhow::anyhow!("Product not found"))?;

        let mut active: ::entity::products::ActiveModel = product.into();
        active.lastsold_gmt = Set(Some(Utc::now().timestamp() as i32));

        let result = active.update(db).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests will be added when we have a test database setup
    // For now, compilation success validates the API design
}
