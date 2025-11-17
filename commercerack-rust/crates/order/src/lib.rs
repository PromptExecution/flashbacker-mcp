//! Order management module

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: i32,
    pub merchant: String,
    pub mid: i32,
    pub prt: i16,
    pub orderid: String,
    pub bs_settlement: i32,
    pub v: Option<i16>,
    pub created_gmt: i32,
    pub modified_gmt: i32,
    pub paid_gmt: i32,
    pub paid_txn: String,
    pub inv_gmt: i32,
    pub shipped_gmt: i32,
    pub synced_gmt: i32,
    pub customer: i32,
    pub pool: String,
    pub order_bill_name: String,
    pub order_bill_email: String,
    pub order_bill_zone: String,
    pub order_bill_phone: String,
    pub order_ship_name: String,
    pub order_ship_zone: String,
    pub review_status: String,
    pub order_payment_status: String,
    pub order_payment_method: String,
    pub order_payment_lookup: String,
    pub order_erefid: Option<String>,
    pub order_total: rust_decimal::Decimal,
    pub order_special: String,
    pub ship_method: String,
    pub mkt: Option<i32>,
    pub mkt_bitstr: String,
    pub flags: i32,
    pub items: i16,
    pub yaml: String,
    pub cartid: Option<String>,
    pub sdomain: Option<String>,
}

impl Order {
    /// Create new order
    pub async fn create(
        pool: &PgPool,
        mid: i32,
        orderid: &str,
        customer: i32,
        order_total: rust_decimal::Decimal,
    ) -> Result<Self> {
        let now = Utc::now().timestamp() as i32;

        let order = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders (mid, orderid, customer, order_total, created_gmt, modified_gmt, pool)
            VALUES ($1, $2, $3, $4, $5, $6, 'RECENT')
            RETURNING id, merchant, mid, prt, orderid, bs_settlement, v, created_gmt,
                      modified_gmt, paid_gmt, paid_txn, inv_gmt, shipped_gmt, synced_gmt,
                      customer, pool as "pool: String", order_bill_name, order_bill_email,
                      order_bill_zone, order_bill_phone, order_ship_name, order_ship_zone,
                      review_status, order_payment_status, order_payment_method,
                      order_payment_lookup, order_erefid,
                      order_total as "order_total: rust_decimal::Decimal",
                      order_special, ship_method, mkt, mkt_bitstr, flags, items, yaml,
                      cartid, sdomain
            "#,
            mid, orderid, customer, order_total, now, now
        )
        .fetch_one(pool)
        .await?;

        Ok(order)
    }

    /// Get order by ID
    pub async fn get(pool: &PgPool, mid: i32, id: i32) -> Result<Option<Self>> {
        let order = sqlx::query_as!(
            Order,
            r#"SELECT id, merchant, mid, prt, orderid, bs_settlement, v, created_gmt,
                      modified_gmt, paid_gmt, paid_txn, inv_gmt, shipped_gmt, synced_gmt,
                      customer, pool as "pool: String", order_bill_name, order_bill_email,
                      order_bill_zone, order_bill_phone, order_ship_name, order_ship_zone,
                      review_status, order_payment_status, order_payment_method,
                      order_payment_lookup, order_erefid,
                      order_total as "order_total: rust_decimal::Decimal",
                      order_special, ship_method, mkt, mkt_bitstr, flags, items, yaml,
                      cartid, sdomain
               FROM orders WHERE mid = $1 AND id = $2"#,
            mid, id
        )
        .fetch_optional(pool)
        .await?;

        Ok(order)
    }

    /// Get order by order ID string
    pub async fn get_by_orderid(pool: &PgPool, mid: i32, orderid: &str) -> Result<Option<Self>> {
        let order = sqlx::query_as!(
            Order,
            r#"SELECT id, merchant, mid, prt, orderid, bs_settlement, v, created_gmt,
                      modified_gmt, paid_gmt, paid_txn, inv_gmt, shipped_gmt, synced_gmt,
                      customer, pool as "pool: String", order_bill_name, order_bill_email,
                      order_bill_zone, order_bill_phone, order_ship_name, order_ship_zone,
                      review_status, order_payment_status, order_payment_method,
                      order_payment_lookup, order_erefid,
                      order_total as "order_total: rust_decimal::Decimal",
                      order_special, ship_method, mkt, mkt_bitstr, flags, items, yaml,
                      cartid, sdomain
               FROM orders WHERE mid = $1 AND orderid = $2"#,
            mid, orderid
        )
        .fetch_optional(pool)
        .await?;

        Ok(order)
    }

    /// Get orders by customer
    pub async fn get_by_customer(pool: &PgPool, mid: i32, customer: i32) -> Result<Vec<Self>> {
        let orders = sqlx::query_as!(
            Order,
            r#"SELECT id, merchant, mid, prt, orderid, bs_settlement, v, created_gmt,
                      modified_gmt, paid_gmt, paid_txn, inv_gmt, shipped_gmt, synced_gmt,
                      customer, pool as "pool: String", order_bill_name, order_bill_email,
                      order_bill_zone, order_bill_phone, order_ship_name, order_ship_zone,
                      review_status, order_payment_status, order_payment_method,
                      order_payment_lookup, order_erefid,
                      order_total as "order_total: rust_decimal::Decimal",
                      order_special, ship_method, mkt, mkt_bitstr, flags, items, yaml,
                      cartid, sdomain
               FROM orders WHERE mid = $1 AND customer = $2
               ORDER BY created_gmt DESC"#,
            mid, customer
        )
        .fetch_all(pool)
        .await?;

        Ok(orders)
    }

    /// Update order
    pub async fn update(&mut self, pool: &PgPool) -> Result<()> {
        self.modified_gmt = Utc::now().timestamp() as i32;

        sqlx::query!(
            r#"UPDATE orders
               SET order_bill_name = $1, order_bill_email = $2,
                   order_ship_name = $3, order_payment_status = $4,
                   order_total = $5, modified_gmt = $6
               WHERE mid = $7 AND id = $8"#,
            self.order_bill_name, self.order_bill_email, self.order_ship_name,
            self.order_payment_status, self.order_total, self.modified_gmt,
            self.mid, self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// List orders for merchant
    pub async fn list(pool: &PgPool, mid: i32, limit: i64, offset: i64) -> Result<Vec<Self>> {
        let orders = sqlx::query_as!(
            Order,
            r#"SELECT id, merchant, mid, prt, orderid, bs_settlement, v, created_gmt,
                      modified_gmt, paid_gmt, paid_txn, inv_gmt, shipped_gmt, synced_gmt,
                      customer, pool as "pool: String", order_bill_name, order_bill_email,
                      order_bill_zone, order_bill_phone, order_ship_name, order_ship_zone,
                      review_status, order_payment_status, order_payment_method,
                      order_payment_lookup, order_erefid,
                      order_total as "order_total: rust_decimal::Decimal",
                      order_special, ship_method, mkt, mkt_bitstr, flags, items, yaml,
                      cartid, sdomain
               FROM orders WHERE mid = $1
               ORDER BY created_gmt DESC
               LIMIT $2 OFFSET $3"#,
            mid, limit, offset
        )
        .fetch_all(pool)
        .await?;

        Ok(orders)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_order_lifecycle() {
        if std::env::var("DATABASE_URL").is_err() {
            eprintln!("Skipping test: DATABASE_URL not set");
            return;
        }

        let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let mid = 1;
        let orderid = "TEST-ORD-001";
        let customer = 1;
        let order_total = rust_decimal::Decimal::new(9999, 2); // $99.99

        // Create
        let mut order = Order::create(&pool, mid, orderid, customer, order_total)
            .await
            .unwrap();
        assert_eq!(order.orderid, orderid);
        assert_eq!(order.customer, customer);
        assert_eq!(order.order_total, order_total);
        assert!(order.id > 0);

        // Get by ID
        let fetched = Order::get(&pool, mid, order.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(fetched.id, order.id);

        // Get by orderid
        let by_orderid = Order::get_by_orderid(&pool, mid, orderid)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(by_orderid.id, order.id);

        // Get by customer
        let by_customer = Order::get_by_customer(&pool, mid, customer).await.unwrap();
        assert!(!by_customer.is_empty());

        // Update
        order.order_bill_name = "John Doe".to_string();
        order.order_total = rust_decimal::Decimal::new(12999, 2); // $129.99
        order.update(&pool).await.unwrap();

        let updated = Order::get(&pool, mid, order.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.order_bill_name, "John Doe");
        assert_eq!(updated.order_total, rust_decimal::Decimal::new(12999, 2));

        // List
        let list = Order::list(&pool, mid, 10, 0).await.unwrap();
        assert!(!list.is_empty());

        // Cleanup - delete test order
        sqlx::query!("DELETE FROM orders WHERE mid = $1 AND id = $2", mid, order.id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
