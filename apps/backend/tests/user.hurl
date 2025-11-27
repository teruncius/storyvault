# Test user endpoint

### Try to access user list without authentication (should fail)
GET http://localhost:3000/api/user

HTTP 401

### Login first to get session cookie
POST http://localhost:3000/api/auth/login
Content-Type: application/json

{
  "email": "alice@example.com",
  "password": "password123"
}

HTTP 200

### Get all users
GET http://localhost:3000/api/user

HTTP 200
[Asserts]
jsonpath "$[0].email" == "alice@example.com"
jsonpath "$[0].first_name" == "Alice"
jsonpath "$[0].last_name" == "Smith"
jsonpath "$[0].avatar_url" == "https://example.com/avatars/alice.png"
jsonpath "$[1].email" == "bob@example.com"
jsonpath "$[1].first_name" == "Bob"
jsonpath "$[1].last_name" == "Jones"
jsonpath "$[1].avatar_url" == "https://example.com/avatars/bob.png"
jsonpath "$[*]" count == 2
