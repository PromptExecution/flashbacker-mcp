//! Customer management module

use anyhow::Result;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod auth;
pub mod address;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Customer {
    pub cid: i32,
    pub mid: i32,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub created_gmt: i32,
    pub modified_gmt: i32,
    pub passhash: String,
    pub passsalt: String,
}

impl Customer {
    /// Create new customer
    pub async fn create(
        pool: &PgPool,
        mid: i32,
        email: &str,
        password: Option<&str>,
    ) -> Result<Self> {
        let now = Utc::now().timestamp() as i32;
        let (passhash, passsalt) = if let Some(pwd) = password {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let hash = argon2.hash_password(pwd.as_bytes(), &salt)
                .map_err(|e| anyhow::anyhow!("Password hashing failed: {:?}", e))?
                .to_string();
            (hash, salt.to_string())
        } else {
            (String::new(), String::new())
        };

        let customer = sqlx::query_as!(
            Customer,
            r#"
            INSERT INTO customers (mid, email, created_gmt, modified_gmt, passhash, passsalt)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING cid, mid, email, firstname, lastname,
                      created_gmt, modified_gmt, passhash, passsalt
            "#,
            mid, email, now, now, passhash, passsalt
        )
        .fetch_one(pool)
        .await?;

        Ok(customer)
    }

    /// Get customer by ID
    pub async fn get(pool: &PgPool, mid: i32, cid: i32) -> Result<Option<Self>> {
        let customer = sqlx::query_as!(
            Customer,
            "SELECT cid, mid, email, firstname, lastname,
                    created_gmt, modified_gmt, passhash, passsalt
             FROM customers WHERE mid = $1 AND cid = $2",
            mid, cid
        )
        .fetch_optional(pool)
        .await?;

        Ok(customer)
    }

    /// Get customer by email
    pub async fn get_by_email(pool: &PgPool, mid: i32, email: &str) -> Result<Option<Self>> {
        let customer = sqlx::query_as!(
            Customer,
            "SELECT cid, mid, email, firstname, lastname,
                    created_gmt, modified_gmt, passhash, passsalt
             FROM customers WHERE mid = $1 AND email = $2",
            mid, email
        )
        .fetch_optional(pool)
        .await?;

        Ok(customer)
    }

    /// Update customer
    pub async fn update(&mut self, pool: &PgPool) -> Result<()> {
        self.modified_gmt = Utc::now().timestamp() as i32;

        sqlx::query!(
            "UPDATE customers
             SET email = $1, firstname = $2, lastname = $3, modified_gmt = $4
             WHERE mid = $5 AND cid = $6",
            self.email, self.firstname, self.lastname,
            self.modified_gmt, self.mid, self.cid
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Delete customer
    pub async fn delete(pool: &PgPool, mid: i32, cid: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM customers WHERE mid = $1 AND cid = $2",
            mid, cid
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Verify password
    pub fn verify_password(&self, password: &str) -> Result<bool> {
        if self.passhash.is_empty() {
            return Ok(false);
        }
        let parsed_hash = PasswordHash::new(&self.passhash)
            .map_err(|e| anyhow::anyhow!("Invalid password hash: {:?}", e))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Set new password
    pub async fn set_password(&mut self, pool: &PgPool, password: &str) -> Result<()> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Password hashing failed: {:?}", e))?
            .to_string();

        self.passhash = hash.clone();
        self.passsalt = salt.to_string();
        self.modified_gmt = Utc::now().timestamp() as i32;

        sqlx::query!(
            "UPDATE customers SET passhash = $1, passsalt = $2, modified_gmt = $3
             WHERE mid = $4 AND cid = $5",
            hash, salt.to_string(), self.modified_gmt, self.mid, self.cid
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// List customers for merchant
    pub async fn list(pool: &PgPool, mid: i32, limit: i64, offset: i64) -> Result<Vec<Self>> {
        let customers = sqlx::query_as!(
            Customer,
            "SELECT cid, mid, email, firstname, lastname,
                    created_gmt, modified_gmt, passhash, passsalt
             FROM customers WHERE mid = $1
             ORDER BY created_gmt DESC
             LIMIT $2 OFFSET $3",
            mid, limit, offset
        )
        .fetch_all(pool)
        .await?;

        Ok(customers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_customer_lifecycle() {
        // Test requires DATABASE_URL
        if std::env::var("DATABASE_URL").is_err() {
            eprintln!("Skipping test: DATABASE_URL not set");
            return;
        }

        let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let mid = 1;
        let email = "test@example.com";

        // Create
        let mut customer = Customer::create(&pool, mid, email, Some("password123"))
            .await
            .unwrap();
        assert_eq!(customer.email, email);
        assert!(!customer.passhash.is_empty());

        // Get by ID
        let fetched = Customer::get(&pool, mid, customer.cid)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(fetched.cid, customer.cid);

        // Get by email
        let by_email = Customer::get_by_email(&pool, mid, email)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(by_email.cid, customer.cid);

        // Verify password
        assert!(customer.verify_password("password123").unwrap());
        assert!(!customer.verify_password("wrong").unwrap());

        // Update
        customer.firstname = "John".to_string();
        customer.update(&pool).await.unwrap();

        let updated = Customer::get(&pool, mid, customer.cid)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.firstname, "John");

        // Set new password
        customer.set_password(&pool, "newpass456").await.unwrap();
        assert!(customer.verify_password("newpass456").unwrap());

        // List
        let list = Customer::list(&pool, mid, 10, 0).await.unwrap();
        assert!(!list.is_empty());

        // Delete
        Customer::delete(&pool, mid, customer.cid).await.unwrap();
        let deleted = Customer::get(&pool, mid, customer.cid).await.unwrap();
        assert!(deleted.is_none());
    }
}
