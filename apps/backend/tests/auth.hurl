# Test authentication endpoints

### Login
POST http://localhost:3000/auth/login
Content-Type: application/json

{
  "email": "alice@example.com",
  "password": "password123"
}

HTTP 200
[Asserts]
jsonpath "$.message" == "Login successful"

### Get current user (me)
GET http://localhost:3000/auth/me

HTTP 200
[Asserts]
jsonpath "$.email" == "alice@example.com"
jsonpath "$.first_name" == "Alice"
jsonpath "$.last_name" == "Smith"

### Logout
POST http://localhost:3000/auth/logout

HTTP 200

### Try to access /auth/me after logout (should fail)
GET http://localhost:3000/auth/me

HTTP 401
