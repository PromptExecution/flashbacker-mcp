//! Order management module using SeaORM

use anyhow::Result;
use chrono::Utc;
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr, Set};
use ::entity::prelude::{Orders, Order as OrderModel};
use rust_decimal::Decimal;

/// Order service for managing order operations
pub struct OrderService;

impl OrderService {
    /// Create new order
    pub async fn create(
        db: &DatabaseConnection,
        mid: i32,
        orderid: &str,
        cartid: &str,
        customer: i32,
        pool: &str,
        total: Decimal,
    ) -> Result<OrderModel> {
        let now = Utc::now().timestamp() as i32;

        let order = ::entity::orders::ActiveModel {
            mid: Set(mid),
            orderid: Set(orderid.to_string()),
            cartid: Set(cartid.to_string()),
            customer: Set(customer),
            pool: Set(pool.to_string()),
            total: Set(total),
            created_gmt: Set(now),
            paid_gmt: Set(None),
            shipped_gmt: Set(None),
            ..Default::default()
        };

        let result = order.insert(db).await?;
        Ok(result)
    }

    /// Find order by ID
    pub async fn find_by_id(
        db: &DatabaseConnection,
        mid: i32,
        id: i32,
    ) -> Result<Option<OrderModel>> {
        let order = Orders::find()
            .filter(::entity::orders::Column::Mid.eq(mid))
            .filter(::entity::orders::Column::Id.eq(id))
            .one(db)
            .await?;

        Ok(order)
    }

    /// Find order by order ID
    pub async fn find_by_orderid(
        db: &DatabaseConnection,
        mid: i32,
        orderid: &str,
    ) -> Result<Option<OrderModel>> {
        let order = Orders::find()
            .filter(::entity::orders::Column::Mid.eq(mid))
            .filter(::entity::orders::Column::Orderid.eq(orderid))
            .one(db)
            .await?;

        Ok(order)
    }

    /// Find order by cart ID
    pub async fn find_by_cartid(
        db: &DatabaseConnection,
        mid: i32,
        cartid: &str,
    ) -> Result<Option<OrderModel>> {
        let order = Orders::find()
            .filter(::entity::orders::Column::Mid.eq(mid))
            .filter(::entity::orders::Column::Cartid.eq(cartid))
            .one(db)
            .await?;

        Ok(order)
    }

    /// List orders by customer
    pub async fn list_by_customer(
        db: &DatabaseConnection,
        mid: i32,
        customer: i32,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<OrderModel>> {
        let orders = Orders::find()
            .filter(::entity::orders::Column::Mid.eq(mid))
            .filter(::entity::orders::Column::Customer.eq(customer))
            .order_by_desc(::entity::orders::Column::CreatedGmt)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?;

        Ok(orders)
    }

    /// List orders by pool
    pub async fn list_by_pool(
        db: &DatabaseConnection,
        mid: i32,
        pool: &str,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<OrderModel>> {
        let orders = Orders::find()
            .filter(::entity::orders::Column::Mid.eq(mid))
            .filter(::entity::orders::Column::Pool.eq(pool))
            .order_by_desc(::entity::orders::Column::CreatedGmt)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?;

        Ok(orders)
    }

    /// Update order
    pub async fn update(
        db: &DatabaseConnection,
        order: OrderModel,
    ) -> Result<OrderModel> {
        let active: ::entity::orders::ActiveModel = order.into();
        let result = active.update(db).await?;
        Ok(result)
    }

    /// Mark order as paid
    pub async fn mark_paid(
        db: &DatabaseConnection,
        mid: i32,
        id: i32,
    ) -> Result<OrderModel> {
        let order = Self::find_by_id(db, mid, id).await?
            .ok_or_else(|| anyhow::anyhow!("Order not found"))?;

        let mut active: ::entity::orders::ActiveModel = order.into();
        active.paid_gmt = Set(Some(Utc::now().timestamp() as i32));

        let result = active.update(db).await?;
        Ok(result)
    }

    /// Mark order as shipped
    pub async fn mark_shipped(
        db: &DatabaseConnection,
        mid: i32,
        id: i32,
    ) -> Result<OrderModel> {
        let order = Self::find_by_id(db, mid, id).await?
            .ok_or_else(|| anyhow::anyhow!("Order not found"))?;

        let mut active: ::entity::orders::ActiveModel = order.into();
        active.shipped_gmt = Set(Some(Utc::now().timestamp() as i32));

        let result = active.update(db).await?;
        Ok(result)
    }

    /// Delete order
    pub async fn delete(
        db: &DatabaseConnection,
        mid: i32,
        id: i32,
    ) -> Result<()> {
        Orders::delete_many()
            .filter(::entity::orders::Column::Mid.eq(mid))
            .filter(::entity::orders::Column::Id.eq(id))
            .exec(db)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests will be added when we have a test database setup
    // For now, compilation success validates the API design
}
