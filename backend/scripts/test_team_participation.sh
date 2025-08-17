#!/bin/bash

# Testing Team Participation API endpoints
# First, start your server: cargo run
# Note: You need to have teams and seasons created first

echo "=== Testing Team Participation API ==="
echo ""

echo "1. Testing Team Participation Not Found (404):"
curl -s http://localhost:8080/api/team-participation/99999 | jq .
echo ""

echo "2. Testing Input Validation (400) - Create participation with invalid team_id:"
curl -s -X POST http://localhost:8080/api/team-participation \
  -H "Content-Type: application/json" \
  -d '{"team_id": -1, "season_id": 1}' | jq .
echo ""

echo "3. Testing Input Validation (400) - Create participation with invalid season_id:"
curl -s -X POST http://localhost:8080/api/team-participation \
  -H "Content-Type: application/json" \
  -d '{"team_id": 1, "season_id": -1}' | jq .
echo ""

echo "4. Testing Successful Team Participation Creation (201):"
PARTICIPATION_RESPONSE=$(curl -s -X POST http://localhost:8080/api/team-participation \
  -H "Content-Type: application/json" \
  -d '{"team_id": 1, "season_id": 1}')
echo $PARTICIPATION_RESPONSE | jq .
PARTICIPATION_ID=$(echo $PARTICIPATION_RESPONSE | jq -r '.id')
echo ""

echo "5. Testing Get Created Team Participation (200):"
curl -s http://localhost:8080/api/team-participation/$PARTICIPATION_ID | jq .
echo ""

echo "6. Testing List Team Participations (200):"
curl -s http://localhost:8080/api/team-participation | jq .
echo ""

echo "7. Testing Find or Create Team Participation (existing):"
curl -s -X POST http://localhost:8080/api/team-participation/find-or-create \
  -H "Content-Type: application/json" \
  -d '{"team_id": 1, "season_id": 1}' | jq .
echo ""

echo "8. Testing Find or Create Team Participation (new):"
curl -s -X POST http://localhost:8080/api/team-participation/find-or-create \
  -H "Content-Type: application/json" \
  -d '{"team_id": 2, "season_id": 1}' | jq .
echo ""

echo "9. Testing Duplicate Participation Creation (should fail with 409 or 400):"
curl -s -X POST http://localhost:8080/api/team-participation \
  -H "Content-Type: application/json" \
  -d '{"team_id": 1, "season_id": 1}' | jq .
echo ""

echo "10. Testing Delete Team Participation (200):"
curl -s -X DELETE http://localhost:8080/api/team-participation/$PARTICIPATION_ID | jq .
echo ""

echo "11. Testing Delete Non-existent Team Participation (404):"
curl -s -X DELETE http://localhost:8080/api/team-participation/99999 | jq .
echo ""

echo "=== Team Participation API Testing Complete ==="
