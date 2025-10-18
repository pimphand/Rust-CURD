#!/bin/bash

# Rust CRUD API Test Script
# Make sure the server is running on http://localhost:3000

BASE_URL="http://localhost:3000"

echo "🚀 Testing Rust CRUD API"
echo "========================="

# Test health check
echo "1. Testing health check..."
curl -s "$BASE_URL/health" | jq .
echo ""

# Test API docs
echo "2. Testing API documentation..."
curl -s "$BASE_URL/api-docs" | jq '.info'
echo ""

# Test create user
echo "3. Creating a new user..."
USER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "name": "Test User",
    "email": "test@example.com",
    "password": "password123"
  }')

echo "$USER_RESPONSE" | jq .
USER_ID=$(echo "$USER_RESPONSE" | jq -r '.id')
echo ""

# Test login
echo "4. Testing login..."
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username_or_email": "testuser",
    "password": "password123"
  }')

echo "$LOGIN_RESPONSE" | jq .
TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token')
echo ""

# Test get current user
echo "5. Testing get current user..."
curl -s -X GET "$BASE_URL/api/auth/me" \
  -H "Authorization: Bearer $TOKEN" | jq .
echo ""

# Test list users
echo "6. Testing list users..."
curl -s -X GET "$BASE_URL/api/users" | jq .
echo ""

# Test get user by ID
echo "7. Testing get user by ID..."
curl -s -X GET "$BASE_URL/api/users/$USER_ID" | jq .
echo ""

# Test update user
echo "8. Testing update user..."
curl -s -X PUT "$BASE_URL/api/users/$USER_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Test User",
    "email": "updated@example.com"
  }' | jq .
echo ""

# Test soft delete user
echo "9. Testing soft delete user..."
curl -s -X DELETE "$BASE_URL/api/users/$USER_ID"
echo "User deleted (status: $?)"
echo ""

# Test restore user
echo "10. Testing restore user..."
curl -s -X POST "$BASE_URL/api/users/$USER_ID/restore" | jq .
echo ""

echo "✅ All tests completed!"
echo ""
echo "📚 API Documentation:"
echo "- Swagger UI: $BASE_URL/swagger-ui"
echo "- OpenAPI JSON: $BASE_URL/api-docs"
