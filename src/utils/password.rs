use bcrypt::{hash, verify, DEFAULT_COST};
use crate::errors::{AppError, Result};

pub struct PasswordUtil;

impl PasswordUtil {
    pub fn hash_password(password: &str) -> Result<String> {
        hash(password, DEFAULT_COST)
            .map_err(|e| AppError::Bcrypt(e))
    }

    pub fn verify_password(password: &str, hashed: &str) -> Result<bool> {
        verify(password, hashed)
            .map_err(|e| AppError::Bcrypt(e))
    }
}
