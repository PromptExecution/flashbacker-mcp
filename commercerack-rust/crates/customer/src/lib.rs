//! Customer management module using SeaORM

use anyhow::Result;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use chrono::Utc;
use sea_orm::*;
use ::entity::prelude::*;

pub mod auth;
pub mod address;

/// Customer service for managing customer operations
pub struct CustomerService;

impl CustomerService {
    /// Create new customer
    pub async fn create(
        db: &DatabaseConnection,
        mid: i32,
        email: &str,
        firstname: &str,
        lastname: &str,
        password: Option<&str>,
    ) -> Result<Customer> {
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

        let customer = ::entity::customers::ActiveModel {
            mid: Set(mid),
            email: Set(email.to_string()),
            firstname: Set(firstname.to_string()),
            lastname: Set(lastname.to_string()),
            created_gmt: Set(now),
            modified_gmt: Set(now),
            passhash: Set(passhash),
            passsalt: Set(passsalt),
            ..Default::default()
        };

        let result = customer.insert(db).await?;
        Ok(result)
    }

    /// Find customer by ID
    pub async fn find_by_id(
        db: &DatabaseConnection,
        mid: i32,
        cid: i32,
    ) -> Result<Option<Customer>> {
        let customer = Customers::find()
            .filter(::entity::customers::Column::Mid.eq(mid))
            .filter(::entity::customers::Column::Cid.eq(cid))
            .one(db)
            .await?;

        Ok(customer)
    }

    /// Find customer by email
    pub async fn find_by_email(
        db: &DatabaseConnection,
        mid: i32,
        email: &str,
    ) -> Result<Option<Customer>> {
        let customer = Customers::find()
            .filter(::entity::customers::Column::Mid.eq(mid))
            .filter(::entity::customers::Column::Email.eq(email))
            .one(db)
            .await?;

        Ok(customer)
    }

    /// Update customer
    pub async fn update(
        db: &DatabaseConnection,
        customer: Customer,
    ) -> Result<Customer> {
        let mut active: ::entity::customers::ActiveModel = customer.into();
        active.modified_gmt = Set(Utc::now().timestamp() as i32);

        let result = active.update(db).await?;
        Ok(result)
    }

    /// Delete customer
    pub async fn delete(
        db: &DatabaseConnection,
        mid: i32,
        cid: i32,
    ) -> Result<()> {
        Customers::delete_many()
            .filter(::entity::customers::Column::Mid.eq(mid))
            .filter(::entity::customers::Column::Cid.eq(cid))
            .exec(db)
            .await?;

        Ok(())
    }

    /// Verify customer password
    pub async fn verify_password(
        customer: &Customer,
        password: &str,
    ) -> Result<bool> {
        if customer.passhash.is_empty() {
            return Ok(false);
        }

        let parsed_hash = PasswordHash::new(&customer.passhash)
            .map_err(|e| anyhow::anyhow!("Invalid password hash: {:?}", e))?;

        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    /// Set customer password
    pub async fn set_password(
        db: &DatabaseConnection,
        mut customer: Customer,
        password: &str,
    ) -> Result<Customer> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Password hashing failed: {:?}", e))?
            .to_string();

        customer.passhash = hash;
        customer.passsalt = salt.to_string();

        Self::update(db, customer).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests will be added when we have a test database setup
    // For now, compilation success validates the API design
}
