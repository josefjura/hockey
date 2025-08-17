#!/bin/bash

# Testing Country API endpoints
# First, start your server: cargo run

echo "=== Testing Country API ==="
echo ""

echo "1. Testing Country Not Found (404):"
curl -s http://localhost:8080/api/countries/99999 | jq .
echo ""

echo "2. Testing Get Country by ID (200):"
curl -s http://localhost:8080/api/countries/1 | jq .
echo ""

echo "3. Testing List All Countries (200):"
curl -s http://localhost:8080/api/countries | jq .
echo ""

echo "4. Testing List Countries with Filters:"
echo "   - Filter by name:"
curl -s "http://localhost:8080/api/countries?name=Czech&page=1&page_size=5" | jq .
echo ""

echo "   - Filter by enabled status:"
curl -s "http://localhost:8080/api/countries?enabled=true&page=1&page_size=10" | jq .
echo ""

echo "   - Filter by IIHF status:"
curl -s "http://localhost:8080/api/countries?iihf=true&page=1&page_size=10" | jq .
echo ""

echo "   - Filter by ISO2 code:"
curl -s "http://localhost:8080/api/countries?iso2_code=CZ" | jq .
echo ""

echo "5. Testing Update Country Status (200):"
curl -s -X PATCH http://localhost:8080/api/countries/1 \
  -H "Content-Type: application/json" \
  -d '{"enabled": true}' | jq .
echo ""

echo "6. Testing Complex Filter Combination:"
curl -s "http://localhost:8080/api/countries?enabled=true&iihf=true&page=1&page_size=5" | jq .
echo ""

echo "=== Country API Testing Complete ==="
