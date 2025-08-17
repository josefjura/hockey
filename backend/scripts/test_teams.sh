#!/bin/bash

# Demonstration of the new error handling in action
# First, start your server: cargo run

echo "=== Testing the New Error Handling ==="
echo ""

echo "1. Testing Team Not Found (404):"
curl -s http://localhost:8080/api/teams/99999 | jq .
echo ""

echo "2. Testing Invalid Team ID Format (400) - if you had path validation:"
# This would work if you added path parameter validation
echo "   (Would need to add path validation for non-numeric IDs)"
echo ""

echo "3. Testing Input Validation (400) - Create team with empty name:"
curl -s -X POST http://localhost:8080/api/teams \
  -H "Content-Type: application/json" \
  -d '{"name": "", "country_id": 1}' | jq .
echo ""

echo "4. Testing Input Validation (400) - Create team with too long name:"
curl -s -X POST http://localhost:8080/api/teams \
  -H "Content-Type: application/json" \
  -d '{"name": "This is a very long team name that exceeds the 100 character limit and should trigger our validation error", "country_id": 1}' | jq .
echo ""

echo "5. Testing Database Error (500) - Invalid country_id:"
curl -s -X POST http://localhost:8080/api/teams \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Team", "country_id": -1}' | jq .
echo ""

echo "6. Testing Successful Request (201):"
curl -s -X POST http://localhost:8080/api/teams \
  -H "Content-Type: application/json" \
  -d '{"name": "Valid Team", "country_id": 1}' | jq .
