# Integration Tests

This directory contains clean, organized integration tests that demonstrate the three-layer architecture of our application.

## Test Files

- **`team.rs`** - Complete test suite for team entity with business logic, HTTP, and service layer tests
- **`event.rs`** - Complete test suite for event entity following the same pattern
- **`match.rs`** - Complete test suite for match entity with comprehensive validation tests
- **`player.rs`** - Complete test suite for player entity with filtering and CRUD validation tests
- **`integration_tests.rs`** - Main integration test runner that includes all entity tests

## Test Structure

### 🎯 Business Logic Tests (Unit Tests)
Tests the domain logic without HTTP concerns:
- `business_logic_get_nonexistent_team_returns_error` - Domain error handling
- `business_logic_create_team_validation` - Input validation rules
- `business_logic_logo_path_validation` - Business rule validation

#### Match Entity Validation Examples
The match entity demonstrates comprehensive validation patterns:
- **Team validation**: Home and away teams must be different, IDs must be positive
- **Score validation**: Scores cannot be negative
- **Date validation**: Match dates must be in valid ISO 8601 format
- **Status validation**: Match status must be one of: scheduled, in_progress, finished, cancelled, postponed
- **Venue validation**: Venue names cannot be empty and must be under 200 characters
- **Score event validation**: Period ranges (1-5), time constraints, player uniqueness rules

#### Player Entity Validation Examples
The player entity demonstrates comprehensive validation patterns:
- **Name validation**: Non-empty names with length constraints (max 100 characters)
- **Photo path validation**: Optional URLs with format validation (relative paths or HTTP/HTTPS URLs)
- **Country relationship**: Foreign key validation with positive ID constraints
- **Filtering capabilities**: Search by name patterns and country relationships with proper validation
- **Complete CRUD operations**: Create, read, update, delete with comprehensive business rule validation

### 🌐 HTTP Integration Tests (End-to-End)
Tests the full HTTP request/response cycle:
- `http_get_nonexistent_team_returns_404` - Error to HTTP status mapping
- `http_invalid_team_id_returns_400` - Parameter validation
- `http_database_error_returns_500` - Error handling under failure

### 🗄️ Service Layer Tests (Data Access)
Tests database operations in isolation:
- `service_layer_get_team_by_id` - Database queries
- `service_layer_database_operations` - CRUD operations

## Benefits of This Architecture

### 🔧 Testability
- **Business Logic**: Test domain rules without HTTP setup
- **HTTP Layer**: Test request/response handling without business complexity
- **Service Layer**: Test database operations without business logic

### 🎯 Clear Separation
- Each layer has focused, specific tests
- Tests are easy to understand and maintain
- No cross-cutting concerns between layers

### 🚀 Fast Feedback
- Business logic tests run quickly (no HTTP server)
- Service layer tests focus on data operations
- HTTP tests verify the complete integration

## Running Tests

```bash
# Run all tests (including unit tests and integration tests)
cargo test

# Run all integration tests
cargo test --test integration_tests

# Run specific entity integration tests
cargo test --test team
cargo test --test event
cargo test --test match
cargo test --test player

# Run only business logic tests
cargo test business_logic

# Run only HTTP tests
cargo test http_

# Run only service layer tests
cargo test service_layer

# Run a specific test
cargo test business_logic_create_team_validation
```

## SQL Fixtures Pattern

**⚡ Major Improvement**: Our integration tests now use SQL fixtures for clean, maintainable test data setup instead of manual database creation.

### 🎯 Fixture-Based Testing Benefits

- **Maintainable**: SQL fixtures are easier to read and modify than manual Rust setup code
- **Consistent**: All tests use the same well-structured test data
- **Reliable**: Dynamic ID resolution eliminates hardcoded ID assumptions
- **Fast**: sqlx handles fixture loading efficiently
- **Clean**: No database pollution between tests thanks to sqlx::test isolation

### 📁 Available Fixtures

```
tests/fixtures/
├── events.sql                    # Basic event test data
├── players.sql                   # Basic player test data  
├── match_deps.sql               # Dependencies for match tests (events, seasons, teams)
├── matches.sql                  # Complete match test data with dependencies
├── player_contract_deps.sql     # Dependencies for player contract tests
├── player_contracts.sql         # Complete player contract test data
├── team_participation_deps.sql  # Dependencies for team participation tests
├── team_participations.sql     # Complete team participation test data
├── season_deps.sql              # Dependencies for season tests (events)
└── seasons.sql                  # Complete season test data with events
```

### 🔧 Using Fixtures in Tests

**Before (Manual Setup):**
```rust
#[sqlx::test]
async fn some_test(pool: SqlitePool) {
    // 50+ lines of CREATE TABLE and INSERT statements
    sqlx::query("CREATE TABLE IF NOT EXISTS event...").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO country...").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO event...").execute(&pool).await.unwrap();
    // ... more manual setup
    
    // Actual test with hardcoded IDs
    let result = some_function(&ctx, 1, 2, 3).await;
}
```

**After (Fixture-Based):**
```rust
#[sqlx::test(fixtures("entity_deps"))]
async fn some_test(pool: SqlitePool) {
    // Get dynamic IDs from fixtures
    let entity_id: i64 = sqlx::query_scalar("SELECT id FROM entity LIMIT 1")
        .fetch_one(&pool).await.unwrap();
    
    // Actual test with dynamic IDs
    let result = some_function(&ctx, entity_id).await;
}
```

### 🏗️ Creating New Fixtures

When adding new fixtures, follow this pattern:

1. **Dependencies Only**: Create fixtures with just the required dependencies
   ```sql
   -- Example: season_deps.sql
   INSERT INTO event (name, country_id, created_at, updated_at) VALUES 
       ('World Championship', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
       ('European Championship', 2, '2024-01-02 12:00:00', '2024-01-02 12:00:00');
   ```

2. **Complete Test Data**: Create fixtures with the full entity hierarchy
   ```sql
   -- Example: seasons.sql (includes events + seasons)
   INSERT INTO event (name, country_id, created_at, updated_at) VALUES 
       ('World Championship', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00');
   
   INSERT INTO season (year, display_name, event_id, created_at, updated_at) VALUES 
       (2023, '2023 World Championship', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00');
   ```

### 🎯 Dynamic ID Resolution

Always use dynamic ID resolution instead of hardcoded IDs:

```rust
// ✅ Good: Dynamic ID resolution
let season_id: i64 = sqlx::query_scalar("SELECT id FROM season LIMIT 1")
    .fetch_one(&pool).await.unwrap();

let team_id: i64 = sqlx::query_scalar("SELECT id FROM team WHERE name = 'Home Team'")
    .fetch_one(&pool).await.unwrap();

// ❌ Bad: Hardcoded IDs
let season_id = 1;
let team_id = 2;
```

### 🏆 Migration Success

**Entities Successfully Migrated to Fixtures:**
- ✅ **PlayerContract** - `player_contract_deps.sql` and `player_contracts.sql`
- ✅ **TeamParticipation** - `team_participation_deps.sql` and `team_participations.sql`  
- ✅ **Match** - `match_deps.sql` and `matches.sql`
- ✅ **Season** - `season_deps.sql` and `seasons.sql`

**Remaining Entities for Migration:**
- 🔄 **Player** - Some tests still use manual setup
- 🔄 **Team** - Service layer tests could benefit from fixtures
- 🔄 **Event** - Basic fixtures exist, integration needed

### 📝 Fixture Best Practices

1. **Reference Migration Data**: Use existing country IDs (1, 2) from database migrations
2. **Avoid ID Conflicts**: Don't specify explicit IDs in fixtures (let auto-increment work)
3. **Meaningful Names**: Use descriptive names that make test assertions clear
4. **Minimal Data**: Include only the data needed for tests
5. **Consistent Timestamps**: Use consistent created_at/updated_at values

## Test Patterns

### Business Logic Test Pattern
```rust
#[sqlx::test]
async fn business_logic_validates_input(pool: SqlitePool) {
    let ctx = ApiContext::new(pool, config);
    
    // Test business rules directly
    let result = TeamBusinessLogic::create_team(&ctx, invalid_input).await;
    
    // Assert business error
    assert!(matches!(result, Err(AppError::InvalidInput { .. })));
}
```

### HTTP Test Pattern
```rust
#[sqlx::test]
async fn http_endpoint_returns_proper_status(pool: SqlitePool) {
    let server = create_test_server(pool).await;
    
    let response = server.get("/api/teams/invalid").await;
    
    // Test HTTP concerns
    response.assert_status(StatusCode::BAD_REQUEST);
    let body: Value = response.json();
    assert_eq!(body["error"], "Expected error message");
}
```

### Service Layer Test Pattern
```rust
#[sqlx::test(fixtures("entity_deps"))] 
async fn service_layer_handles_data(pool: SqlitePool) {
    // Get dynamic IDs from fixtures
    let parent_id: i64 = sqlx::query_scalar("SELECT id FROM parent_entity LIMIT 1")
        .fetch_one(&pool).await.unwrap();
    
    // Test service operation
    let result = service::create_data(&pool, CreateEntity {
        name: "Test Data".to_string(),
        parent_id,
    }).await;
    
    // Assert data correctness
    assert!(result.is_ok());
    let entity = service::get_data(&pool, result.unwrap()).await.unwrap();
    assert_eq!(entity.name, "Test Data");
}
```

## Adding New Tests

When adding tests for new features:

1. **Business Logic**: Test domain rules and validation
2. **HTTP Layer**: Test request/response mapping and status codes  
3. **Service Layer**: Test database operations and data integrity

This ensures comprehensive coverage across all architectural layers.
