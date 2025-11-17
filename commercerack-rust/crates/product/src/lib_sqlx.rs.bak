//! Product and SKU management module

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod sku;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: i32,
    pub mid: i32,
    pub merchant: String,
    pub product: String,
    pub ts: i32,
    pub product_name: String,
    pub category: String,
    pub data: String,
    pub salesrank: i32,
    pub created_gmt: i32,
    pub lastsold_gmt: i32,
    pub base_price: Option<rust_decimal::Decimal>,
    pub base_cost: Option<rust_decimal::Decimal>,
    pub supplier: Option<String>,
    pub supplier_id: Option<String>,
    pub mfg: Option<String>,
    pub mfg_id: Option<String>,
    pub upc: String,
    pub options: i32,
    pub profile: String,
    pub mkt: i64,
    pub prod_is: i32,
    pub mkt_bitstr: String,
    pub mkterr_bitstr: String,
}

impl Product {
    /// Create new product
    pub async fn create(
        pool: &PgPool,
        mid: i32,
        product_code: &str,
        product_name: &str,
    ) -> Result<Self> {
        let now = Utc::now().timestamp() as i32;

        let product = sqlx::query_as!(
            Product,
            r#"
            INSERT INTO products (mid, product, product_name, created_gmt, ts)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, mid, merchant, product, ts, product_name, category, data,
                      salesrank, created_gmt, lastsold_gmt,
                      base_price as "base_price: rust_decimal::Decimal",
                      base_cost as "base_cost: rust_decimal::Decimal",
                      supplier, supplier_id, mfg, mfg_id, upc, options, profile,
                      mkt, prod_is, mkt_bitstr, mkterr_bitstr
            "#,
            mid, product_code, product_name, now, now
        )
        .fetch_one(pool)
        .await?;

        Ok(product)
    }

    /// Get product by ID
    pub async fn get(pool: &PgPool, mid: i32, id: i32) -> Result<Option<Self>> {
        let product = sqlx::query_as!(
            Product,
            r#"SELECT id, mid, merchant, product, ts, product_name, category, data,
                      salesrank, created_gmt, lastsold_gmt,
                      base_price as "base_price: rust_decimal::Decimal",
                      base_cost as "base_cost: rust_decimal::Decimal",
                      supplier, supplier_id, mfg, mfg_id, upc, options, profile,
                      mkt, prod_is, mkt_bitstr, mkterr_bitstr
               FROM products WHERE mid = $1 AND id = $2"#,
            mid, id
        )
        .fetch_optional(pool)
        .await?;

        Ok(product)
    }

    /// Get product by product code
    pub async fn get_by_code(pool: &PgPool, mid: i32, product_code: &str) -> Result<Option<Self>> {
        let product = sqlx::query_as!(
            Product,
            r#"SELECT id, mid, merchant, product, ts, product_name, category, data,
                      salesrank, created_gmt, lastsold_gmt,
                      base_price as "base_price: rust_decimal::Decimal",
                      base_cost as "base_cost: rust_decimal::Decimal",
                      supplier, supplier_id, mfg, mfg_id, upc, options, profile,
                      mkt, prod_is, mkt_bitstr, mkterr_bitstr
               FROM products WHERE mid = $1 AND product = $2"#,
            mid, product_code
        )
        .fetch_optional(pool)
        .await?;

        Ok(product)
    }

    /// Update product
    pub async fn update(&mut self, pool: &PgPool) -> Result<()> {
        self.ts = Utc::now().timestamp() as i32;

        sqlx::query!(
            r#"UPDATE products
               SET product_name = $1, category = $2, base_price = $3,
                   base_cost = $4, supplier = $5, supplier_id = $6,
                   mfg = $7, mfg_id = $8, upc = $9, ts = $10
               WHERE mid = $11 AND id = $12"#,
            self.product_name, self.category,
            self.base_price as Option<rust_decimal::Decimal>,
            self.base_cost as Option<rust_decimal::Decimal>,
            self.supplier, self.supplier_id, self.mfg, self.mfg_id,
            self.upc, self.ts, self.mid, self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Delete product
    pub async fn delete(pool: &PgPool, mid: i32, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM products WHERE mid = $1 AND id = $2", mid, id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// List products for merchant
    pub async fn list(pool: &PgPool, mid: i32, limit: i64, offset: i64) -> Result<Vec<Self>> {
        let products = sqlx::query_as!(
            Product,
            r#"SELECT id, mid, merchant, product, ts, product_name, category, data,
                      salesrank, created_gmt, lastsold_gmt,
                      base_price as "base_price: rust_decimal::Decimal",
                      base_cost as "base_cost: rust_decimal::Decimal",
                      supplier, supplier_id, mfg, mfg_id, upc, options, profile,
                      mkt, prod_is, mkt_bitstr, mkterr_bitstr
               FROM products WHERE mid = $1
               ORDER BY created_gmt DESC
               LIMIT $2 OFFSET $3"#,
            mid, limit, offset
        )
        .fetch_all(pool)
        .await?;

        Ok(products)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_product_lifecycle() {
        if std::env::var("DATABASE_URL").is_err() {
            eprintln!("Skipping test: DATABASE_URL not set");
            return;
        }

        let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let mid = 1;
        let product_code = "TEST-PROD-001";
        let product_name = "Test Product";

        // Create
        let mut product = Product::create(&pool, mid, product_code, product_name)
            .await
            .unwrap();
        assert_eq!(product.product, product_code);
        assert_eq!(product.product_name, product_name);
        assert!(product.id > 0);

        // Get by ID
        let fetched = Product::get(&pool, mid, product.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(fetched.id, product.id);

        // Get by code
        let by_code = Product::get_by_code(&pool, mid, product_code)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(by_code.id, product.id);

        // Update
        product.product_name = "Updated Product".to_string();
        product.base_price = Some(rust_decimal::Decimal::new(1999, 2)); // $19.99
        product.update(&pool).await.unwrap();

        let updated = Product::get(&pool, mid, product.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.product_name, "Updated Product");
        assert_eq!(updated.base_price, Some(rust_decimal::Decimal::new(1999, 2)));

        // List
        let list = Product::list(&pool, mid, 10, 0).await.unwrap();
        assert!(!list.is_empty());

        // Delete
        Product::delete(&pool, mid, product.id).await.unwrap();
        let deleted = Product::get(&pool, mid, product.id).await.unwrap();
        assert!(deleted.is_none());
    }
}
