# Test audiobook position endpoint

### Login first to get session cookie
POST http://localhost:3000/auth/login
Content-Type: application/json

{
  "email": "alice@example.com",
  "password": "password123"
}

HTTP 200

### Get audiobook position (Moby Dick)
GET http://localhost:3000/audiobook/994c2844-c63f-85b8-e150-880099884444/position

HTTP 200
[Asserts]
jsonpath "$.id" == "994c2844-c63f-85b8-e150-880099884444"
jsonpath "$.position_iso" == "PT0S"

### Set audiobook position (Moby Dick)
PUT http://localhost:3000/audiobook/994c2844-c63f-85b8-e150-880099884444/position
Content-Type: application/json

{
  "position_iso": "PT3600S"
}

HTTP 200

### Get audiobook position after setting (should still be PT0S as it's a placeholder)
GET http://localhost:3000/audiobook/994c2844-c63f-85b8-e150-880099884444/position

HTTP 200
[Asserts]
jsonpath "$.id" == "994c2844-c63f-85b8-e150-880099884444"
jsonpath "$.position_iso" == "PT0S"

### Get audiobook position with invalid UUID (placeholder returns 200)
GET http://localhost:3000/audiobook/00000000-0000-0000-0000-000000000000/position

HTTP 200
