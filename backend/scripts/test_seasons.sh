#!/bin/bash

# Testing Season API endpoints
# First, start your server: cargo run
# Note: You need to have events created first for season creation to work

echo "=== Testing Season API ==="
echo ""

echo "1. Testing Season Not Found (404):"
curl -s http://localhost:8080/api/seasons/99999 | jq .
echo ""

echo "2. Testing Input Validation (400) - Create season with invalid year:"
curl -s -X POST http://localhost:8080/api/seasons \
  -H "Content-Type: application/json" \
  -d '{"year": 1800, "event_id": 1}' | jq .
echo ""

echo "3. Testing Input Validation (400) - Create season with invalid event_id:"
curl -s -X POST http://localhost:8080/api/seasons \
  -H "Content-Type: application/json" \
  -d '{"year": 2024, "event_id": -1}' | jq .
echo ""

echo "4. Testing Successful Season Creation (201):"
SEASON_RESPONSE=$(curl -s -X POST http://localhost:8080/api/seasons \
  -H "Content-Type: application/json" \
  -d '{"year": 2024, "event_id": 1}')
echo $SEASON_RESPONSE | jq .
SEASON_ID=$(echo $SEASON_RESPONSE | jq -r '.id')
echo ""

echo "5. Testing Get Created Season (200):"
curl -s http://localhost:8080/api/seasons/$SEASON_ID | jq .
echo ""

echo "6. Testing Update Season (200):"
curl -s -X PUT http://localhost:8080/api/seasons/$SEASON_ID \
  -H "Content-Type: application/json" \
  -d '{"year": 2025, "event_id": 1}' | jq .
echo ""

echo "7. Testing List Seasons (200):"
curl -s http://localhost:8080/api/seasons | jq .
echo ""

echo "8. Testing List Seasons Simple (200):"
curl -s http://localhost:8080/api/seasons/list | jq .
echo ""

echo "9. Testing List Seasons with Filters:"
echo "   - Filter by year:"
curl -s "http://localhost:8080/api/seasons?year=2024&page=1&page_size=5" | jq .
echo ""

echo "   - Filter by event:"
curl -s "http://localhost:8080/api/seasons?event_id=1&page=1&page_size=5" | jq .
echo ""

echo "10. Testing Season Team Players endpoint:"
curl -s "http://localhost:8080/api/seasons/$SEASON_ID/team/1/players" | jq .
echo ""

echo "11. Testing Delete Season (200):"
curl -s -X DELETE http://localhost:8080/api/seasons/$SEASON_ID | jq .
echo ""

echo "12. Testing Delete Non-existent Season (404):"
curl -s -X DELETE http://localhost:8080/api/seasons/99999 | jq .
echo ""

echo "=== Season API Testing Complete ==="
