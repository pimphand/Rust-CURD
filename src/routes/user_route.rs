use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::{
    models::user::{CreateUser, UpdateUser, UserResponse},
    services::user_service::UserService,
    utils::{jwt::JwtUtil, oauth::GoogleOAuth},
    errors::{AppError, Result},
};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username or email address
    pub username_or_email: String,
    /// User password
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    /// JWT access token
    pub token: String,
    /// User information
    pub user: UserResponse,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    /// Page number (default: 1)
    pub page: Option<u64>,
    /// Items per page (default: 10)
    pub per_page: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    /// List of items
    pub data: Vec<T>,
    /// Current page number
    pub page: u64,
    /// Items per page
    pub per_page: u64,
    /// Total number of items
    pub total: u64,
}

pub fn user_routes() -> Router<UserService> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .route("/users/:id/restore", post(restore_user))
        .route("/auth/login", post(login))
        .route("/auth/me", get(get_current_user))
        .route("/auth/google", get(google_oauth_login))
        .route("/auth/google/callback", get(google_oauth_callback))
}

/// Create a new user
/// 
/// Creates a new user account with the provided information.
/// Username and email must be unique.
async fn create_user(
    State(user_service): State<UserService>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<UserResponse>> {
    let user = user_service.create(payload).await?;
    Ok(Json(user))
}

/// Get user by ID
/// 
/// Retrieves a user by their unique identifier.
async fn get_user(
    State(user_service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>> {
    let user = user_service.find_by_id(id).await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    Ok(Json(user))
}

/// Update user
/// 
/// Updates an existing user's information.
/// Only provided fields will be updated.
async fn update_user(
    State(user_service): State<UserService>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<UserResponse>> {
    let user = user_service.update(id, payload).await?;
    Ok(Json(user))
}

/// Delete user (soft delete)
/// 
/// Soft deletes a user by setting the deleted_at timestamp.
/// The user can be restored later.
async fn delete_user(
    State(user_service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    user_service.soft_delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Restore deleted user
/// 
/// Restores a soft-deleted user by clearing the deleted_at timestamp.
async fn restore_user(
    State(user_service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>> {
    let user = user_service.restore(id).await?;
    Ok(Json(user))
}

/// List users
/// 
/// Retrieves a paginated list of users.
async fn list_users(
    State(user_service): State<UserService>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<UserResponse>>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    
    let users = user_service.list(page, per_page).await?;
    let total = users.len() as u64;
    
    Ok(Json(PaginatedResponse {
        data: users,
        page,
        per_page,
        total, // In a real app, you'd want to get the total count
    }))
}

/// User login
/// 
/// Authenticates a user with username/email and password.
/// Returns a JWT token and user information.
async fn login(
    State(user_service): State<UserService>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    // Try to find user by username first, then by email
    let user = if let Some(user) = user_service.find_by_username_with_password(&payload.username_or_email).await? {
        user
    } else if let Some(user) = user_service.find_by_email_with_password(&payload.username_or_email).await? {
        user
    } else {
        return Err(AppError::Authentication("Invalid credentials".to_string()));
    };

    // Verify password
    if !user_service.verify_password(&user, &payload.password).await? {
        return Err(AppError::Authentication("Invalid credentials".to_string()));
    }

    // Generate JWT token
    let token = JwtUtil::generate_token(
        user.id.to_string(),
        user.username.clone(),
        user.email.clone(),
        "your-secret-key", // In real app, get from config
        24, // 24 hours
    )?;

    Ok(Json(LoginResponse {
        token,
        user: UserResponse::from(user),
    }))
}

/// Get current user
/// 
/// Retrieves the current authenticated user's information.
/// Requires a valid JWT token in the Authorization header.
async fn get_current_user(
    State(user_service): State<UserService>,
    headers: axum::http::HeaderMap,
) -> Result<Json<UserResponse>> {
    let auth_header = headers
        .get("Authorization")
        .ok_or_else(|| AppError::Authentication("Missing authorization header".to_string()))?
        .to_str()
        .map_err(|_| AppError::Authentication("Invalid authorization header".to_string()))?;

    let token = JwtUtil::extract_bearer_token(auth_header)?;
    let claims = JwtUtil::verify_token(&token, "your-secret-key")?; // In real app, get from config
    
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;
    
    let user = user_service.find_by_id(user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    
    Ok(Json(user))
}

/// Get Google OAuth URL
/// 
/// Returns the Google OAuth authorization URL for user authentication.
async fn google_oauth_login() -> Result<Json<serde_json::Value>> {
    // In a real app, you'd get these from config
    let google_oauth = GoogleOAuth::new(
        "your-google-client-id".to_string(),
        "your-google-client-secret".to_string(),
        "http://localhost:3000/api/auth/google/callback".to_string(),
    );

    let (auth_url, _csrf_token) = google_oauth.get_authorization_url();
    
    Ok(Json(json!({
        "auth_url": auth_url,
        "message": "Redirect user to this URL for Google OAuth"
    })))
}

/// Google OAuth callback
/// 
/// Handles the callback from Google OAuth after user authorization.
/// Creates a new user if they don't exist, or logs in existing user.
async fn google_oauth_callback(
    State(user_service): State<UserService>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<LoginResponse>> {
    let code = params.get("code")
        .ok_or_else(|| AppError::OAuth("Missing authorization code".to_string()))?;

    let google_oauth = GoogleOAuth::new(
        "your-google-client-id".to_string(),
        "your-google-client-secret".to_string(),
        "http://localhost:3000/api/auth/google/callback".to_string(),
    );

    // Exchange code for token
    let access_token = google_oauth.exchange_code_for_token(code, "").await?;
    
    // Get user info from Google
    let google_user = google_oauth.get_user_info(&access_token).await?;

    // Check if user exists
    let user = if let Some(existing_user) = user_service.find_by_email(&google_user.email).await? {
        existing_user
    } else {
        // Create new user
        let create_user = CreateUser {
            username: google_user.email.split('@').next().unwrap_or("user").to_string(),
            name: google_user.name,
            email: google_user.email,
            password: "oauth-user".to_string(), // Random password for OAuth users
        };
        user_service.create(create_user).await?
    };

    // Generate JWT token
    let token = JwtUtil::generate_token(
        user.id.to_string(),
        user.username.clone(),
        user.email.clone(),
        "your-secret-key", // In real app, get from config
        24, // 24 hours
    )?;

    Ok(Json(LoginResponse {
        token,
        user,
    }))
}
