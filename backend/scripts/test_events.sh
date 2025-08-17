#!/bin/bash

# Demonstration of the new event error handling and business logic in action
# First, start your server: cargo run

echo "=== Testing Event API with New Architecture ==="
echo ""

echo "1. Testing Event Not Found (404):"
curl -s http://localhost:8080/api/events/99999 | jq .
echo ""

echo "2. Testing Input Validation (400) - Create event with empty name:"
curl -s -X POST http://localhost:8080/api/events \
  -H "Content-Type: application/json" \
  -d '{"name": "", "country_id": null}' | jq .
echo ""

echo "3. Testing Input Validation (400) - Create event with too long name:"
curl -s -X POST http://localhost:8080/api/events \
  -H "Content-Type: application/json" \
  -d '{"name": "'"$(printf '%*s' 256 | tr ' ' 'A')"'", "country_id": null}' | jq .
echo ""

echo "4. Testing Input Validation (400) - Create event with invalid country_id:"
curl -s -X POST http://localhost:8080/api/events \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Event", "country_id": -1}' | jq .
echo ""

echo "5. Testing Successful Event Creation (201):"
EVENT_RESPONSE=$(curl -s -X POST http://localhost:8080/api/events \
  -H "Content-Type: application/json" \
  -d '{"name": "World Championship", "country_id": null}')
echo $EVENT_RESPONSE | jq .
EVENT_ID=$(echo $EVENT_RESPONSE | jq -r '.id')
echo ""

echo "6. Testing Get Created Event (200):"
curl -s http://localhost:8080/api/events/$EVENT_ID | jq .
echo ""

echo "7. Testing Update Event (200):"
curl -s -X PUT http://localhost:8080/api/events/$EVENT_ID \
  -H "Content-Type: application/json" \
  -d '{"name": "Updated World Championship", "country_id": 1}' | jq .
echo ""

echo "8. Testing List Events (200):"
curl -s http://localhost:8080/api/events | jq .
echo ""

echo "9. Testing List Events with Filters:"
curl -s "http://localhost:8080/api/events?name=World&page=1&page_size=5" | jq .
echo ""

echo "10. Testing Delete Event (200):"
curl -s -X DELETE http://localhost:8080/api/events/$EVENT_ID | jq .
echo ""

echo "11. Testing Delete Non-existent Event (404):"
curl -s -X DELETE http://localhost:8080/api/events/99999 | jq .
echo ""

echo "=== Event API Testing Complete ==="
