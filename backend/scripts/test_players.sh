#!/bin/bash

# Testing Player API endpoints
# First, start your server: cargo run

echo "=== Testing Player API ==="
echo ""

echo "1. Testing Player Not Found (404):"
curl -s http://localhost:8080/api/players/99999 | jq .
echo ""

echo "2. Testing Input Validation (400) - Create player with empty name:"
curl -s -X POST http://localhost:8080/api/players \
  -H "Content-Type: application/json" \
  -d '{"name": "", "country_id": 1}' | jq .
echo ""

echo "3. Testing Input Validation (400) - Create player with too long name:"
curl -s -X POST http://localhost:8080/api/players \
  -H "Content-Type: application/json" \
  -d '{"name": "'"$(printf '%*s' 256 | tr ' ' 'A')"'", "country_id": 1}' | jq .
echo ""

echo "4. Testing Input Validation (400) - Create player with invalid country_id:"
curl -s -X POST http://localhost:8080/api/players \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Player", "country_id": -1}' | jq .
echo ""

echo "5. Testing Successful Player Creation (201):"
PLAYER_RESPONSE=$(curl -s -X POST http://localhost:8080/api/players \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "country_id": 1}')
echo $PLAYER_RESPONSE | jq .
PLAYER_ID=$(echo $PLAYER_RESPONSE | jq -r '.id')
echo ""

echo "6. Testing Get Created Player (200):"
curl -s http://localhost:8080/api/players/$PLAYER_ID | jq .
echo ""

echo "7. Testing Update Player (200):"
curl -s -X PUT http://localhost:8080/api/players/$PLAYER_ID \
  -H "Content-Type: application/json" \
  -d '{"name": "John Updated Doe", "country_id": 2}' | jq .
echo ""

echo "8. Testing List Players (200):"
curl -s http://localhost:8080/api/players | jq .
echo ""

echo "9. Testing List Players with Filters:"
echo "   - Filter by name:"
curl -s "http://localhost:8080/api/players?name=John&page=1&page_size=5" | jq .
echo ""

echo "   - Filter by country:"
curl -s "http://localhost:8080/api/players?country_id=1&page=1&page_size=5" | jq .
echo ""

echo "10. Testing Delete Player (200):"
curl -s -X DELETE http://localhost:8080/api/players/$PLAYER_ID | jq .
echo ""

echo "11. Testing Delete Non-existent Player (404):"
curl -s -X DELETE http://localhost:8080/api/players/99999 | jq .
echo ""

echo "=== Player API Testing Complete ==="
