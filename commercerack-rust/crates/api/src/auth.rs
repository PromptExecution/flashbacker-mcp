//! JWT authentication middleware and utilities

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct Claims {
    pub sub: String,      // Subject (customer ID)
    pub mid: i32,         // Merchant ID
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
}

impl Claims {
    /// Create new claims with 24h expiration
    pub fn new(customer_id: i32, mid: i32) -> Self {
        let now = Utc::now();
        Self {
            sub: customer_id.to_string(),
            mid,
            iat: now.timestamp(),
            exp: (now + Duration::hours(24)).timestamp(),
        }
    }

    /// Encode claims into JWT token
    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    /// Decode JWT token into claims
    pub fn decode(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}

/// Axum extractor for JWT authentication
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            ))?;

        // Parse Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header format".to_string(),
            ))?;

        // Decode and validate JWT
        // TODO: Get secret from config
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-key".to_string());

        Claims::decode(token, &secret).map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                format!("Invalid token: {}", e),
            )
        })
    }
}
