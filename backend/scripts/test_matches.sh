#!/bin/bash

# Testing Match API endpoints
# First, start your server: cargo run
# Note: You need to have seasons and teams created first for match creation to work

echo "=== Testing Match API ==="
echo ""

echo "1. Testing Match Not Found (404):"
curl -s http://localhost:8080/api/matches/99999 | jq .
echo ""

echo "2. Testing Input Validation (400) - Create match with same home and away team:"
curl -s -X POST http://localhost:8080/api/matches \
  -H "Content-Type: application/json" \
  -d '{"season_id": 1, "home_team_id": 1, "away_team_id": 1, "match_date": "2024-01-15", "status": "scheduled"}' | jq .
echo ""

echo "3. Testing Input Validation (400) - Create match with invalid season_id:"
curl -s -X POST http://localhost:8080/api/matches \
  -H "Content-Type: application/json" \
  -d '{"season_id": -1, "home_team_id": 1, "away_team_id": 2, "match_date": "2024-01-15", "status": "scheduled"}' | jq .
echo ""

echo "4. Testing Successful Match Creation (201):"
MATCH_RESPONSE=$(curl -s -X POST http://localhost:8080/api/matches \
  -H "Content-Type: application/json" \
  -d '{"season_id": 1, "home_team_id": 1, "away_team_id": 2, "match_date": "2024-01-15", "status": "scheduled"}')
echo $MATCH_RESPONSE | jq .
MATCH_ID=$(echo $MATCH_RESPONSE | jq -r '.id')
echo ""

echo "5. Testing Get Created Match (200):"
curl -s http://localhost:8080/api/matches/$MATCH_ID | jq .
echo ""

echo "6. Testing Get Match with Stats (200):"
curl -s http://localhost:8080/api/matches/$MATCH_ID/stats | jq .
echo ""

echo "7. Testing Update Match (200):"
curl -s -X PUT http://localhost:8080/api/matches/$MATCH_ID \
  -H "Content-Type: application/json" \
  -d '{"season_id": 1, "home_team_id": 1, "away_team_id": 2, "match_date": "2024-01-16", "status": "finished"}' | jq .
echo ""

echo "8. Testing List Matches (200):"
curl -s http://localhost:8080/api/matches | jq .
echo ""

echo "9. Testing List Matches with Filters:"
echo "   - Filter by season:"
curl -s "http://localhost:8080/api/matches?season_id=1&page=1&page_size=5" | jq .
echo ""

echo "   - Filter by team:"
curl -s "http://localhost:8080/api/matches?team_id=1&page=1&page_size=5" | jq .
echo ""

echo "   - Filter by status:"
curl -s "http://localhost:8080/api/matches?status=finished&page=1&page_size=5" | jq .
echo ""

echo "   - Filter by date range:"
curl -s "http://localhost:8080/api/matches?date_from=2024-01-01&date_to=2024-12-31&page=1&page_size=5" | jq .
echo ""

echo "10. Testing Score Events:"
echo "    - Add score event:"
SCORE_RESPONSE=$(curl -s -X POST http://localhost:8080/api/matches/$MATCH_ID/score-events \
  -H "Content-Type: application/json" \
  -d '{"team_id": 1, "player_id": 1, "period": 1, "time_minutes": 10, "time_seconds": 30, "event_type": "goal"}')
echo $SCORE_RESPONSE | jq .
SCORE_ID=$(echo $SCORE_RESPONSE | jq -r '.id')
echo ""

echo "    - Get score events:"
curl -s http://localhost:8080/api/matches/$MATCH_ID/score-events | jq .
echo ""

echo "    - Delete score event:"
curl -s -X DELETE http://localhost:8080/api/matches/$MATCH_ID/score-events/$SCORE_ID | jq .
echo ""

echo "11. Testing Delete Match (200):"
curl -s -X DELETE http://localhost:8080/api/matches/$MATCH_ID | jq .
echo ""

echo "12. Testing Delete Non-existent Match (404):"
curl -s -X DELETE http://localhost:8080/api/matches/99999 | jq .
echo ""

echo "=== Match API Testing Complete ==="
