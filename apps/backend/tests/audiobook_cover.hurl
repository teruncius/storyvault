# Test audiobook cover endpoint

### Try to access audiobook cover without authentication (should fail)
GET http://localhost:3000/audiobook/994c2844-c63f-85b8-e150-880099884444/cover

HTTP 401

### Login first to get session cookie
POST http://localhost:3000/auth/login
Content-Type: application/json

{
  "email": "alice@example.com",
  "password": "password123"
}

HTTP 200

### Get audiobook cover (Moby Dick - WebP)
GET http://localhost:3000/audiobook/994c2844-c63f-85b8-e150-880099884444/cover

HTTP 200
[Asserts]
header "Content-Type" == "image/webp"

### Get audiobook cover with invalid UUID (should fail)
GET http://localhost:3000/audiobook/00000000-0000-0000-0000-000000000000/cover

HTTP 404
