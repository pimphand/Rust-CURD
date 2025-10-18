use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    /// Username must be unique
    pub username: String,
    /// Full name of the user
    pub name: String,
    /// Email address must be unique
    pub email: String,
    /// Password will be hashed before storage
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    /// Username must be unique
    pub username: Option<String>,
    /// Full name of the user
    pub name: Option<String>,
    /// Email address must be unique
    pub email: Option<String>,
    /// Password will be hashed before storage
    pub password: Option<String>,
    /// Account status
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    /// Unique user identifier
    pub id: Uuid,
    /// Username
    pub username: String,
    /// Full name
    pub name: String,
    /// Email address
    pub email: String,
    /// Account status
    pub is_active: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            name: user.name,
            email: user.email,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

// Add conversion from SeaORM Model to UserResponse
impl From<crate::entities::user::Model> for UserResponse {
    fn from(user: crate::entities::user::Model) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            name: user.name,
            email: user.email,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
