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
jsonpath "$.position_iso" != null

### Set audiobook position (Moby Dick)
PUT http://localhost:3000/audiobook/994c2844-c63f-85b8-e150-880099884444/position
Content-Type: application/json

{
  "event_type": "PLAY",
  "position_iso": "PT3600S"
}

HTTP 200

### Get audiobook position after setting
GET http://localhost:3000/audiobook/994c2844-c63f-85b8-e150-880099884444/position

HTTP 200
[Asserts]
jsonpath "$.position_iso" == "PT3600S"

### Get audiobook position with invalid UUID
GET http://localhost:3000/audiobook/00000000-0000-0000-0000-000000000000/position

HTTP 404
