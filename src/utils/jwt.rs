use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use crate::errors::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,
    pub iat: usize,
    pub username: String,
    pub email: String,
}

impl Claims {
    pub fn new(user_id: String, username: String, email: String, expiration_hours: u64) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expiration_hours as i64);
        
        Claims {
            sub: user_id,
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            username,
            email,
        }
    }
}

pub struct JwtUtil;

impl JwtUtil {
    pub fn generate_token(
        user_id: String,
        username: String,
        email: String,
        secret: &str,
        expiration_hours: u64,
    ) -> Result<String> {
        let claims = Claims::new(user_id, username, email, expiration_hours);
        let header = Header::new(Algorithm::HS256);
        
        encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
            .map_err(|e| AppError::Jwt(e))
    }

    pub fn verify_token(token: &str, secret: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Jwt(e))
    }

    pub fn extract_bearer_token(auth_header: &str) -> Result<String> {
        if auth_header.starts_with("Bearer ") {
            Ok(auth_header[7..].to_string())
        } else {
            Err(AppError::Authentication("Invalid authorization header format".to_string()))
        }
    }
}
