use axum::{response::Json, routing::get, Router};
use serde_json::json;

pub fn docs_routes() -> Router {
    Router::new()
        .route("/api-docs", get(get_api_docs))
}

async fn get_api_docs() -> Json<serde_json::Value> {
    Json(json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Rust CRUD API",
            "description": "A complete REST API built with Rust, featuring user management, JWT authentication, Google OAuth integration, and PostgreSQL database with SeaORM.",
            "version": "1.0.0",
            "contact": {
                "name": "API Support",
                "email": "support@example.com"
            }
        },
        "servers": [
            {
                "url": "http://localhost:3000",
                "description": "Development server"
            }
        ],
        "tags": [
            {
                "name": "Users",
                "description": "User management endpoints"
            },
            {
                "name": "Authentication",
                "description": "Authentication endpoints"
            },
            {
                "name": "System",
                "description": "System endpoints"
            }
        ],
        "paths": {
            "/health": {
                "get": {
                    "tags": ["System"],
                    "summary": "Health check",
                    "description": "Returns the current status of the API server",
                    "responses": {
                        "200": {
                            "description": "API is running",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "status": {"type": "string"},
                                            "message": {"type": "string"},
                                            "timestamp": {"type": "string"}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/users": {
                "post": {
                    "tags": ["Users"],
                    "summary": "Create a new user",
                    "description": "Creates a new user account with the provided information. Username and email must be unique.",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["username", "name", "email", "password"],
                                    "properties": {
                                        "username": {"type": "string", "description": "Username must be unique"},
                                        "name": {"type": "string", "description": "Full name of the user"},
                                        "email": {"type": "string", "format": "email", "description": "Email address must be unique"},
                                        "password": {"type": "string", "description": "Password will be hashed before storage"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {"description": "User created successfully"},
                        "400": {"description": "Validation error"},
                        "409": {"description": "Username or email already exists"}
                    }
                },
                "get": {
                    "tags": ["Users"],
                    "summary": "List users",
                    "description": "Retrieves a paginated list of users",
                    "parameters": [
                        {
                            "name": "page",
                            "in": "query",
                            "description": "Page number (default: 1)",
                            "schema": {"type": "integer"}
                        },
                        {
                            "name": "per_page",
                            "in": "query",
                            "description": "Items per page (default: 10)",
                            "schema": {"type": "integer"}
                        }
                    ],
                    "responses": {
                        "200": {"description": "Users retrieved successfully"}
                    }
                }
            },
            "/api/users/{id}": {
                "get": {
                    "tags": ["Users"],
                    "summary": "Get user by ID",
                    "description": "Retrieves a user by their unique identifier",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "description": "User ID",
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {"description": "User found"},
                        "404": {"description": "User not found"}
                    }
                },
                "put": {
                    "tags": ["Users"],
                    "summary": "Update user",
                    "description": "Updates an existing user's information. Only provided fields will be updated.",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "description": "User ID",
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "username": {"type": "string"},
                                        "name": {"type": "string"},
                                        "email": {"type": "string"},
                                        "password": {"type": "string"},
                                        "is_active": {"type": "boolean"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {"description": "User updated successfully"},
                        "404": {"description": "User not found"},
                        "400": {"description": "Validation error"}
                    }
                },
                "delete": {
                    "tags": ["Users"],
                    "summary": "Delete user (soft delete)",
                    "description": "Soft deletes a user by setting the deleted_at timestamp. The user can be restored later.",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "description": "User ID",
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "204": {"description": "User deleted successfully"},
                        "404": {"description": "User not found"}
                    }
                }
            },
            "/api/users/{id}/restore": {
                "post": {
                    "tags": ["Users"],
                    "summary": "Restore deleted user",
                    "description": "Restores a soft-deleted user by clearing the deleted_at timestamp",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "description": "User ID",
                            "schema": {"type": "string", "format": "uuid"}
                        }
                    ],
                    "responses": {
                        "200": {"description": "User restored successfully"},
                        "404": {"description": "User not found or not deleted"}
                    }
                }
            },
            "/api/auth/login": {
                "post": {
                    "tags": ["Authentication"],
                    "summary": "User login",
                    "description": "Authenticates a user with username/email and password. Returns a JWT token and user information.",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["username_or_email", "password"],
                                    "properties": {
                                        "username_or_email": {"type": "string", "description": "Username or email address"},
                                        "password": {"type": "string", "description": "User password"}
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {"description": "Login successful"},
                        "401": {"description": "Invalid credentials"}
                    }
                }
            },
            "/api/auth/me": {
                "get": {
                    "tags": ["Authentication"],
                    "summary": "Get current user",
                    "description": "Retrieves the current authenticated user's information. Requires a valid JWT token in the Authorization header.",
                    "security": [{"bearerAuth": []}],
                    "responses": {
                        "200": {"description": "User information retrieved"},
                        "401": {"description": "Invalid or missing token"},
                        "404": {"description": "User not found"}
                    }
                }
            },
            "/api/auth/google": {
                "get": {
                    "tags": ["Authentication"],
                    "summary": "Get Google OAuth URL",
                    "description": "Returns the Google OAuth authorization URL for user authentication",
                    "responses": {
                        "200": {"description": "Google OAuth URL generated"}
                    }
                }
            },
            "/api/auth/google/callback": {
                "get": {
                    "tags": ["Authentication"],
                    "summary": "Google OAuth callback",
                    "description": "Handles the callback from Google OAuth after user authorization. Creates a new user if they don't exist, or logs in existing user.",
                    "parameters": [
                        {
                            "name": "code",
                            "in": "query",
                            "required": true,
                            "description": "Authorization code from Google",
                            "schema": {"type": "string"}
                        }
                    ],
                    "responses": {
                        "200": {"description": "OAuth login successful"},
                        "400": {"description": "Invalid authorization code"},
                        "500": {"description": "OAuth error"}
                    }
                }
            }
        },
        "components": {
            "securitySchemes": {
                "bearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "JWT"
                }
            }
        }
    }))
}

