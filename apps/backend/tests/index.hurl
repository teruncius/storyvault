# Test index endpoint

### Get index
GET http://localhost:3000/api/

HTTP 200
[Asserts]
jsonpath "$.health_url" == "http://localhost:3000/api/health"
jsonpath "$.auth_me_url" == "http://localhost:3000/api/auth/me"
jsonpath "$.user_url" == "http://localhost:3000/api/user"
jsonpath "$.audiobook_url" == "http://localhost:3000/api/audiobook"
