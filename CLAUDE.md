# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a hockey management application being rewritten as a single Rust binary using Axum, Maud, and HTMX. The system manages hockey tournaments, teams, players, and their relationships across seasons and events.

**Status**: Active rewrite from Next.js/React to HTMX/Maud architecture (see REWRITE_ANALYSIS.md)

### Authentication System (New Architecture)

The rewrite uses simplified session-based authentication instead of JWT:

**Key Features**:
- Session cookies (HttpOnly, Secure, SameSite)
- Bcrypt password hashing (preserved from old system)
- CSRF protection
- Session expiration

**Why the change**: Simpler for server-rendered HTMX architecture, no client-side token management needed.

## Architecture

### New Frontend (Rust - Active Development in `frontend/`)
- **Framework**: Axum web framework
- **Templating**: Maud for type-safe HTML generation
- **Interactivity**: HTMX for dynamic updates
- **Components**: Lit Web Components for rich client-side features
- **Database**: SQLite with SQLx for type-safe queries
- **Authentication**: Session-based with cookies
- **Internationalization**: fluent-rs for Czech/English support
- **Styling**: Tailwind CSS
- **Deployment**: Single binary with embedded assets
- **Port**: Runs on port 8080

### Old Backend (Rust - Reference in `backend/`)
- Preserved for SQL query reference
- Business logic patterns can be ported
- Service layer patterns reusable

### Old Frontend (Next.js - Archived in `backup/`)
- Gitignored, kept as reference only
- UI patterns and workflows documented in REWRITE_ANALYSIS.md

### Database Schema
The database follows a hierarchical structure:
- `country` → `event` → `season` → `team_participation` → `player_contract`
- Junction tables: `team_participation` (teams in seasons), `player_contract` (players in team participations)
- Authentication: `users` table with bcrypt password hashing, `refresh_tokens` table for token management

## Development Commands

### New Frontend (Active Development)
```bash
cd frontend
cargo run                    # Start development server (creates DB from migrations)
cargo build                  # Build the project
cargo test                   # Run tests
cargo check                  # Check code without building
cargo fmt                    # Format code
cargo clippy                 # Lint code
cargo run --bin create_admin # Create an admin user
cargo watch -x run           # Hot reload (requires cargo-watch)
```

### Database
```bash
cd frontend
# Database is automatically created on first run from migrations/
# Migrations are applied automatically at startup
```

### Reference Old Code
```bash
# SQL queries
cat backend/src/*/service/fixtures/*.sql

# Business logic patterns
cat backend/src/*/business.rs

# Service patterns
cat backend/src/*/service/mod.rs
```

## Environment Configuration

### New Frontend Environment Variables
- `DATABASE_URL`: SQLite database file path (default: `sqlite:./hockey.db`)
- `SESSION_SECRET`: Secret key for session encryption (generate with `openssl rand -hex 32`)
- `ENVIRONMENT`: Environment mode - `development` or `production`
- `PORT`: Server port (default: 8080)

### Generate Secrets
```bash
# Generate session secret
openssl rand -hex 32
```

See README.md for complete setup instructions.

## Key File Locations

### New Frontend Structure (Active Development)
```
frontend/
├── src/
│   ├── main.rs                    # Entry point, server setup
│   ├── config.rs                  # Configuration management
│   ├── routes/                    # Route handlers (return HTML)
│   │   ├── auth.rs               # Login, logout
│   │   ├── dashboard.rs
│   │   ├── teams.rs
│   │   └── ...
│   ├── views/                     # Maud templates
│   │   ├── layout.rs             # Base layout with sidebar
│   │   ├── components/           # Reusable template components
│   │   │   ├── table.rs
│   │   │   ├── form.rs
│   │   │   └── pagination.rs
│   │   └── pages/                # Page templates
│   ├── business/                  # Business logic
│   ├── service/                   # Data access
│   ├── auth/                      # Session management
│   │   ├── session.rs
│   │   ├── middleware.rs
│   │   └── password.rs
│   └── i18n/                      # Internationalization
│       ├── mod.rs
│       └── messages/
│           ├── en.ftl
│           └── cs.ftl
├── web_components/                # Lit components (compiled separately)
│   └── small-table.ts
├── static/                        # Static assets (embedded)
│   ├── css/
│   ├── js/
│   └── flags/
├── migrations/                    # SQLx migrations
└── Cargo.toml
```

### Reference Directories
- `backend/`: Old Rust API (for SQL queries and business logic patterns)
- `backup/`: Old Next.js frontend (gitignored, UI patterns reference)

## Testing

### Tests
- Unit tests will be in `src/*/tests.rs`
- Integration tests in `tests/`
- Run with `cargo test`
- Linting with `cargo clippy`
- Formatting with `cargo fmt --check`

## Domain Model

### Core Entities
- **Country**: Geographic entities with IIHF membership and ISO codes
- **Event**: Hockey tournaments/competitions hosted by countries
- **Season**: Specific years/editions of events
- **Team**: Hockey teams representing countries
- **Player**: Individual players with nationality
- **TeamParticipation**: Teams participating in specific seasons
- **PlayerContract**: Player assignments to team participations

### Business Rules
- Historical countries (Soviet Union, East Germany) supported with year ranges
- IIHF membership tracking for countries
- Season-specific team and player relationships
- Authentication required for most operations

## API Structure (New HTMX Architecture)

Routes return HTML (full pages or fragments):
- `/`: Dashboard (protected)
- `/auth/login`: Login page (GET) and form submission (POST)
- `/auth/logout`: Logout (POST)
- `/teams`: Teams management page (protected)
- `/teams/list`: HTMX endpoint for table updates
- `/teams/{id}/edit`: Edit form modal
- `/players`: Players management (protected)
- `/events`: Events management (protected)
- `/seasons`: Seasons management (protected)
- `/matches`: Matches list (protected)
- `/matches/{id}`: Match detail with scoring (protected)
- `/management`: Countries and settings (protected)

Protected routes check session cookie. HTMX endpoints return HTML fragments.

## Development Workflow

1. **Code Changes**: Modify Rust code, use `cargo watch -x run` for hot reload
2. **Database Changes**: Create new migration files in `frontend/migrations/`
3. **Templates**: Edit Maud templates in `src/views/`
4. **HTMX**: Add HTMX attributes to templates for dynamic behavior
5. **Web Components**: Edit Lit components in `web_components/`, compile with npm
6. **Authentication**: All protected routes check session cookie

## Common Patterns (New Architecture)

### Backend Service Pattern (Preserved)
Each domain follows this structure:
```rust
// routes/{domain}.rs - Route handlers returning HTML
// business/{domain}.rs - Business logic and validation
// service/{domain}.rs - Data access with SQLx
// views/pages/{domain}.rs - Maud templates
```

### HTMX Patterns
- **Full page loads**: Return complete HTML with layout
- **Partial updates**: HTMX requests return fragments
- **Forms**: Submit with `hx-post`, return updated HTML
- **Pagination**: `hx-get` with page parameter, target table body
- **Search**: Debounced with `hx-trigger="keyup changed delay:300ms"`
- **Modals**: Load form via `hx-get`, submit via `hx-post`

### Component Patterns
- **Maud templates**: Type-safe HTML in Rust
- **Reusable components**: Functions returning `Markup`
- **Lit components**: For rich client-side (small tables, selectors)
- **Progressive enhancement**: Works without JS, enhanced with HTMX/Lit

### Error Handling (New Architecture)
**CRITICAL**: Proper error handling in Axum routes and templates.

#### Error Patterns:
1. **Typed Errors**: Use `AppError` enum with `IntoResponse`
   ```rust
   pub enum AppError {
       NotFound { entity: String, id: i64 },
       InvalidInput { message: String },
       Database(sqlx::Error),
       Unauthorized,
   }

   impl IntoResponse for AppError {
       fn into_response(self) -> Response {
           match self {
               AppError::NotFound { entity, id } => {
                   (StatusCode::NOT_FOUND, html! { /* error page */ }).into_response()
               }
               // ... other cases
           }
       }
   }
   ```

2. **Validation**: Server-side validation in business layer
   ```rust
   if name.trim().is_empty() {
       return Err(AppError::InvalidInput {
           message: "Name cannot be empty".to_string()
       });
   }
   ```

3. **Graceful Fallbacks**: Empty states in templates
   ```rust
   @if teams.is_empty() {
       p { "No teams found." }
   } @else {
       // ... render table
   }
   ```

4. **HTMX Error Responses**: Return error HTML fragments
   ```rust
   // On error, return error message fragment
   html! { div.error { (error_message) } }
   ```

### Database Query Pattern
- Use SQLx QueryBuilder for dynamic queries
- Entity structs for database operations
- DTO structs for API responses
- Fixture SQL files for complex queries

## Development Notes

- **Development Environment**:
  - I'm usually running the frontend and backend in the background, so if you want to test something, just tell me