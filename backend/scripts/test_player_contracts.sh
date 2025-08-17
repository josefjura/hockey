#!/bin/bash

# Testing Player Contract API endpoints
# First, start your server: cargo run
# Note: You need to have team participation and players created first

echo "=== Testing Player Contract API ==="
echo ""

echo "1. Testing Player Contract Not Found (404):"
curl -s http://localhost:8080/api/player-contracts/99999 | jq .
echo ""

echo "2. Testing Input Validation (400) - Create contract with invalid team_participation_id:"
curl -s -X POST http://localhost:8080/api/player-contracts \
  -H "Content-Type: application/json" \
  -d '{"team_participation_id": -1, "player_id": 1}' | jq .
echo ""

echo "3. Testing Input Validation (400) - Create contract with invalid player_id:"
curl -s -X POST http://localhost:8080/api/player-contracts \
  -H "Content-Type: application/json" \
  -d '{"team_participation_id": 1, "player_id": -1}' | jq .
echo ""

echo "4. Testing Successful Player Contract Creation (201):"
CONTRACT_RESPONSE=$(curl -s -X POST http://localhost:8080/api/player-contracts \
  -H "Content-Type: application/json" \
  -d '{"team_participation_id": 1, "player_id": 1}')
echo $CONTRACT_RESPONSE | jq .
CONTRACT_ID=$(echo $CONTRACT_RESPONSE | jq -r '.id')
echo ""

echo "5. Testing Get Created Player Contract (200):"
curl -s http://localhost:8080/api/player-contracts/$CONTRACT_ID | jq .
echo ""

echo "6. Testing List Player Contracts (200):"
curl -s http://localhost:8080/api/player-contracts | jq .
echo ""

echo "7. Testing Duplicate Contract Creation (should fail with 409 or 400):"
curl -s -X POST http://localhost:8080/api/player-contracts \
  -H "Content-Type: application/json" \
  -d '{"team_participation_id": 1, "player_id": 1}' | jq .
echo ""

echo "8. Testing Delete Player Contract (200):"
curl -s -X DELETE http://localhost:8080/api/player-contracts/$CONTRACT_ID | jq .
echo ""

echo "9. Testing Delete Non-existent Player Contract (404):"
curl -s -X DELETE http://localhost:8080/api/player-contracts/99999 | jq .
echo ""

echo "=== Player Contract API Testing Complete ==="
