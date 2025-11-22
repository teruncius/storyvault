# Test index endpoint

### Get index
GET http://localhost:3000/

HTTP 200
[Asserts]
jsonpath "$.health_url" == "http://localhost:3000/health"
jsonpath "$.auth_me_url" == "http://localhost:3000/auth/me"
jsonpath "$.user_url" == "http://localhost:3000/user"
jsonpath "$.audiobook_url" == "http://localhost:3000/audiobook"
