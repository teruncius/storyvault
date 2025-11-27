# Test health check endpoint

### Get health status
GET http://localhost:3000/api/health

HTTP 200
[Asserts]
jsonpath "$.status" == "ok"
