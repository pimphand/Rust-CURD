# Rust CRUD API with JWT Authentication and Google OAuth

A complete REST API built with Rust, featuring user management, JWT authentication, Google OAuth integration, and PostgreSQL database with SeaORM.

## Features

- ✅ **User CRUD Operations** with soft delete
- ✅ **JWT Authentication** with secure token generation
- ✅ **Google OAuth Login** integration
- ✅ **PostgreSQL Database** with SeaORM
- ✅ **Axum Web Framework** with middleware
- ✅ **Password Hashing** with bcrypt
- ✅ **Error Handling** with custom error types
- ✅ **Logging** with tracing
- ✅ **CORS Support** for cross-origin requests
- ✅ **Swagger UI Documentation** with interactive API explorer
- ✅ **OpenAPI 3.0 Specification** for API documentation

## Project Structure

```
├── Cargo.toml                # Project dependencies
├── src/
│   ├── main.rs               # Application entry point
│   ├── lib.rs                # Library exports
│   ├── config/               # Configuration management
│   │   └── mod.rs
│   ├── db/                   # Database connection
│   │   └── mod.rs
│   ├── models/               # Data models
│   │   └── user.rs
│   ├── entities/             # SeaORM entities
│   │   └── user.rs
│   ├── routes/               # API routes
│   │   ├── mod.rs
│   │   └── user_route.rs
│   ├── services/             # Business logic
│   │   └── user_service.rs
│   ├── utils/                # Utility functions
│   │   ├── mod.rs
│   │   ├── jwt.rs
│   │   ├── password.rs
│   │   └── oauth.rs
│   └── errors.rs             # Error definitions
├── migrations/               # Database migrations
│   └── 001_create_users_table.sql
└── .env                      # Environment variables
```

## Setup Instructions

### 1. Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- Google OAuth credentials

### 2. Database Setup

1. Create a PostgreSQL database:
```sql
CREATE DATABASE rust_crud;
```

2. Run the migration:
```bash
psql -d rust_crud -f migrations/001_create_users_table.sql
```

### 3. Environment Configuration

Copy the `.env` file and update the values:

```bash
# Database Configuration
DATABASE_URL=postgres://username:password@localhost/rust_crud

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRATION=86400

# Google OAuth Configuration
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret
GOOGLE_REDIRECT_URI=http://localhost:3000/api/auth/google/callback

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
```

### 4. Google OAuth Setup

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select existing one
3. Enable Google+ API
4. Create OAuth 2.0 credentials
5. Add authorized redirect URIs: `http://localhost:3000/api/auth/google/callback`
6. Copy Client ID and Client Secret to `.env` file

### 5. Run the Application

```bash
# Install dependencies
cargo build

# Run the application
cargo run
```

The server will start on `http://localhost:3000`

## API Documentation

### Swagger UI
Access the interactive API documentation at: `http://localhost:3000/swagger-ui`

### OpenAPI JSON
Get the raw OpenAPI specification at: `http://localhost:3000/api-docs`

## API Endpoints

### System
- `GET /` - Root endpoint
- `GET /health` - Health check
- `GET /api-docs` - OpenAPI documentation (JSON)
- `GET /swagger-ui` - Swagger UI interface

### Authentication

- `POST /api/auth/login` - Login with username/email and password
- `GET /api/auth/me` - Get current user info (requires JWT token)
- `GET /api/auth/google` - Get Google OAuth URL
- `GET /api/auth/google/callback` - Google OAuth callback

### User Management

- `POST /api/users` - Create new user
- `GET /api/users` - List users (with pagination)
- `GET /api/users/:id` - Get user by ID
- `PUT /api/users/:id` - Update user
- `DELETE /api/users/:id` - Soft delete user
- `POST /api/users/:id/restore` - Restore deleted user

## API Usage Examples

### 1. Create User

```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "name": "John Doe",
    "email": "john@example.com",
    "password": "securepassword"
  }'
```

### 2. Login

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username_or_email": "johndoe",
    "password": "securepassword"
  }'
```

### 3. Get Current User

```bash
curl -X GET http://localhost:3000/api/auth/me \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### 4. Update User

```bash
curl -X PUT http://localhost:3000/api/users/USER_ID \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "name": "John Updated",
    "email": "john.updated@example.com"
  }'
```

### 5. Google OAuth Flow

1. Get OAuth URL:
```bash
curl -X GET http://localhost:3000/api/auth/google
```

2. User visits the returned URL and authorizes
3. Google redirects to callback with authorization code
4. Server exchanges code for user info and returns JWT token

## Database Schema

### Users Table

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| username | VARCHAR(255) | Unique username |
| name | VARCHAR(255) | Full name |
| email | VARCHAR(255) | Unique email |
| password | VARCHAR(255) | Hashed password |
| is_active | BOOLEAN | Account status |
| created_at | TIMESTAMP | Creation time |
| updated_at | TIMESTAMP | Last update time |
| deleted_at | TIMESTAMP | Soft delete timestamp |

## Security Features

- **Password Hashing**: Uses bcrypt for secure password storage
- **JWT Tokens**: Secure authentication with configurable expiration
- **Soft Delete**: Users are not permanently deleted
- **Input Validation**: Comprehensive validation for all inputs
- **CORS Protection**: Configurable cross-origin resource sharing
- **Error Handling**: Secure error messages without sensitive data exposure

## Development

### Adding New Features

1. Create models in `src/models/`
2. Add entities in `src/entities/`
3. Implement business logic in `src/services/`
4. Create routes in `src/routes/`
5. Add database migrations in `migrations/`

### Logging

The application uses structured logging with tracing. Set the `RUST_LOG` environment variable to control log levels:

```bash
RUST_LOG=debug cargo run
```

## Production Considerations

1. **Environment Variables**: Use secure secrets management
2. **Database**: Use connection pooling for better performance
3. **JWT Secret**: Use a strong, randomly generated secret
4. **HTTPS**: Always use HTTPS in production
5. **Rate Limiting**: Implement rate limiting for API endpoints
6. **Monitoring**: Add application monitoring and health checks

## Testing

### Manual Testing
Run the provided test script to test all API endpoints:

```bash
# Make sure the server is running first
cargo run

# In another terminal, run the test script
./test_api.sh
```

### API Testing with curl

```bash
# Health check
curl http://localhost:3000/health

# Create user
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser", "name": "Test User", "email": "test@example.com", "password": "password123"}'

# Login
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username_or_email": "testuser", "password": "password123"}'
```

## License

This project is open source and available under the MIT License.
