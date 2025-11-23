# Test audiobook endpoint

### Try to access audiobook list without authentication (should fail)
GET http://localhost:3000/audiobook

HTTP 401

### Login first to get session cookie
POST http://localhost:3000/auth/login
Content-Type: application/json

{
  "email": "alice@example.com",
  "password": "password123"
}

HTTP 200

### Get all audiobooks
GET http://localhost:3000/audiobook

HTTP 200
[Asserts]
jsonpath "$[*]" count == 3
jsonpath "$[0].id" exists
jsonpath "$[0].title" exists
jsonpath "$[0].author" exists
jsonpath "$[0].year" exists
jsonpath "$[0].cover_url" exists
jsonpath "$[0].position_url" exists
jsonpath "$[0].stream_url" exists
jsonpath "$[1].id" exists
jsonpath "$[1].title" exists
jsonpath "$[1].author" exists
jsonpath "$[2].id" exists
jsonpath "$[2].title" exists
jsonpath "$[2].author" exists
