# Hockey Backend

A Rust-based API backend for hockey management using Axum, SQLx, and SQLite.

> Part of the Hockey Management monorepo

## Architecture

### 🎯 Clean Three-Layer Architecture

- **Routes Layer** (`src/*/routes.rs`): HTTP concerns only
- **Business Logic Layer** (`src/*/business.rs`): Domain rules and validation  
- **Service Layer** (`src/*/service/`): Data access and persistence

Each route handler delegates to business logic, which handles domain validation before calling the service layer for persistence. This separation ensures clean, testable, and maintainable code.

#### Match Entity Features
The match module demonstrates advanced domain logic patterns:
- **Complex validation rules**: Multi-field validation with business constraints
- **Score event management**: Nested entity operations with validation
- **Match statistics**: Aggregated data retrieval with relationships
- **Comprehensive test coverage**: 12 integration tests covering all scenarios

#### Player Entity Features
The player module demonstrates comprehensive validation patterns:
- **Name validation**: Non-empty names with length constraints
- **Photo path validation**: Optional URLs with format validation (relative paths or HTTP/HTTPS)
- **Country relationship**: Foreign key validation with positive ID constraints
- **Filtering capabilities**: Search by name patterns and country relationships
- **Complete CRUD operations**: Create, read, update, delete with validation

### 🔧 Error Handling

Structured error handling with `AppError` enum:
- Automatic HTTP status code mapping
- Consistent JSON error responses
- UUID tracking for error correlation
- Proper error propagation between layers

#### Domain-Specific Errors
- `TeamNotFound` - Team entity lookup failures
- `EventNotFound` - Event entity lookup failures  
- `MatchNotFound` - Match entity lookup failures
- `ScoreEventNotFound` - Score event lookup failures
- `InvalidInput` - Business rule validation failures

## Getting Started

```bash
# Install dependencies
cargo build

# Run database migrations
cargo install sqlx-cli
sqlx migrate run

# Start the server
cargo run

# Run tests
cargo test
```

## Documentation

- [`tests/README.md`](tests/README.md) - Testing strategy and patterns
- [`scripts/test_teams.sh`](scripts/test_teams.sh) - Manual API testing script for teams
- [`scripts/test_events.sh`](scripts/test_events.sh) - Manual API testing script for events

## API Testing

```bash
# Make the test scripts executable
chmod +x test_errors.sh test_events.sh

# Start the server
cargo run

# In another terminal, test the APIs
./test_errors.sh   # Test team endpoints
./test_events.sh   # Test event endpoints
```

## Project Structure

```
src/
├── config.rs          # Configuration management
├── errors.rs          # Centralized error handling
├── http.rs            # HTTP context and setup
├── lib.rs             # Library exports
├── main.rs            # Application entry point
├── team/              # Team module (example)
│   ├── business.rs    # Business logic layer
│   ├── mod.rs         # Module exports
│   ├── routes.rs      # HTTP routes layer
│   └── service/       # Data access layer
├── event/             # Event module (example)
│   ├── business.rs    # Business logic layer
│   ├── mod.rs         # Module exports
│   ├── routes.rs      # HTTP routes layer
│   └── service/       # Data access layer
├── match/             # Match module (example)
│   ├── business.rs    # Business logic layer
│   ├── mod.rs         # Module exports
│   ├── routes.rs      # HTTP routes layer
│   └── service/       # Data access layer
└── player/            # Player module (example)
    ├── business.rs    # Business logic layer
    ├── mod.rs         # Module exports
    ├── routes.rs      # HTTP routes layer
    └── service/       # Data access layer
```