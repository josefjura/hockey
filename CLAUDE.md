# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a hockey management application built as a single Rust binary using Axum, Maud, and HTMX. The system manages hockey tournaments, teams, players, and their relationships across seasons and events.

### Authentication System

Session-based authentication with cookies:

**Key Features**:
- Session cookies (HttpOnly, Secure, SameSite)
- Bcrypt password hashing
- CSRF protection
- Session expiration
- No client-side token management required

## Architecture

### Tech Stack
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

### Database Schema
The database follows a hierarchical structure:
- `country` → `event` → `season` → `team_participation` → `player_contract`
- Junction tables: `team_participation` (teams in seasons), `player_contract` (players in team participations)
- Authentication: `users` table with bcrypt password hashing

## Development Commands
```bash
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
# Database is automatically created on first run from migrations/
# Migrations are applied automatically at startup
```

## Environment Configuration
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

### Project Structure
```
src/
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

## Testing

- Unit tests in `src/*/tests.rs` or `#[cfg(test)]` modules
- Integration tests in `tests/` directory
- Run with `cargo test`
- Prefer `#[sqlx::test]` for database tests (automatic setup/teardown)
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

## API Structure

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
2. **Database Changes**: Create new migration files in `migrations/`
3. **Templates**: Edit Maud templates in `src/views/`
4. **HTMX**: Add HTMX attributes to templates for dynamic behavior
5. **Web Components**: Edit Lit components in `web_components/`, compile with npm
6. **Authentication**: All protected routes check session cookie

## Common Patterns

### Service Layer Pattern
Each domain follows this structure:
```rust
// routes/{domain}.rs - Route handlers returning HTML
// business/{domain}.rs - Business logic and validation
// service/{domain}.rs - Data access with SQLx
// views/pages/{domain}.rs - Maud templates
```

### Route Handler Naming Conventions
**IMPORTANT**: Follow these naming conventions for all route handlers to maintain consistency:

#### Function Names:
- `{resource}_get` - Full page GET endpoints (e.g., `teams_get`, `events_get`)
- `{resource}_list_partial` - HTMX HTML fragments for list updates (e.g., `teams_list_partial`)
- `{resource}_create_form` - GET endpoints returning create form/modal HTML (e.g., `team_create_form`)
- `{resource}_edit_form` - GET endpoints returning edit form/modal HTML (e.g., `team_edit_form`)
- `{resource}_create` - POST endpoints for creation (e.g., `team_create`)
- `{resource}_update` - POST endpoints for updates (e.g., `team_update`)
- `{resource}_delete` - POST endpoints for deletion (e.g., `team_delete`)
- `{resource}_detail` - GET endpoints for detail views (e.g., `match_detail`)
- `{resource}_{action}_api` - Future JSON API endpoints (e.g., `teams_list_api`)

#### URL Patterns:
- `GET /{resources}` - Full page (calls `{resource}_get`)
- `GET /{resources}/list` - Partial update (calls `{resource}_list_partial`)
- `GET /{resources}/new` - Create form (calls `{resource}_create_form`)
- `POST /{resources}` - Create action (calls `{resource}_create`)
- `GET /{resources}/:id/edit` - Edit form (calls `{resource}_edit_form`)
- `POST /{resources}/:id` - Update action (calls `{resource}_update`)
- `POST /{resources}/:id/delete` - Delete action (calls `{resource}_delete`)
- `GET /{resources}/:id` - Detail view (calls `{resource}_detail`)

#### Examples:
```rust
// Good (route handlers in routes/*.rs)
pub async fn teams_get(...) -> impl IntoResponse { }
pub async fn teams_list_partial(...) -> impl IntoResponse { }
pub async fn team_create_form(...) -> impl IntoResponse { }
pub async fn team_create(...) -> impl IntoResponse { }

// Good (view templates in views/pages/*.rs)
pub fn teams_page(...) -> Markup { }  // Called by teams_get
pub fn team_list_content(...) -> Markup { }  // Called by teams_list_partial

// Bad (inconsistent naming)
pub async fn teams_list_get(...) -> impl IntoResponse { }
pub async fn teams_list_htmx(...) -> impl IntoResponse { }
pub async fn event_create_get(...) -> impl IntoResponse { }
pub async fn event_create_post(...) -> impl IntoResponse { }
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

### Error Handling
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

2. **Validation**: Use validation helpers from `src/validation.rs`
   ```rust
   use crate::validation::validate_name;

   let name = match validate_name(&form.name) {
       Ok(n) => n,
       Err(error) => return Html(error_modal(&t, Some(error)).into_string()),
   };
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