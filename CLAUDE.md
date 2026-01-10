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

**CRITICAL: Always run `make precommit` before pushing to avoid CI failures!**

```bash
make precommit               # REQUIRED before push: format, clippy, test (fixes issues!)
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

**When creating or modifying features, write tests. When creating or updating tests, consult [TESTING.md](./TESTING.md).**

This project uses a hybrid testing strategy:
- **Backend tests** (Rust): Service layer + route handlers
- **Component tests** (Storybook): Interaction testing with play functions
- **E2E tests** (Playwright): Minimal smoke tests for critical user flows

```bash
# Run all tests
make test-all

# Run specific test suites
make test              # Rust tests only
make test-storybook    # Storybook component tests
make test-e2e          # E2E smoke tests (requires server on :8080)

# Pre-commit checks (format, clippy, Rust tests)
make precommit
```

See **[TESTING.md](./TESTING.md)** for patterns, examples, and best practices.

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
6. **Testing**: Write tests for new features (see [TESTING.md](./TESTING.md))
7. **Authentication**: All protected routes check session cookie
8. **Changelog**: **ALWAYS update CHANGELOG.md** when fixing issues or adding features
9. **Before Push**: **ALWAYS run `make precommit`** to avoid CI failures

## Changelog Maintenance

**CRITICAL: Every closed issue and solved problem MUST be documented in CHANGELOG.md**

### When to Update
- Fixing a bug (GitHub issue or not)
- Adding a new feature
- Changing existing behavior
- Removing functionality
- Security fixes

### How to Update
1. Open `CHANGELOG.md`
2. Add your change under `## [Unreleased]` in the appropriate category:
   - **Added** - New features users can use
   - **Changed** - Changes to existing functionality
   - **Deprecated** - Features that will be removed soon
   - **Removed** - Features that have been removed
   - **Fixed** - Bug fixes
   - **Security** - Security-related changes

3. Write in **user-facing language**, not technical implementation details
4. Reference the GitHub issue number if applicable (e.g., `(#146)`)

### Writing Style
- **Good**: "Player event statistics are now saved reliably without leaving incomplete data if an error occurs (#146)"
- **Bad**: "Refactored create_or_update_player_event_stats to use UPSERT instead of INSERT+UPDATE"

- **Good**: "Added ability to track player statistics across multiple tournaments"
- **Bad**: "Implemented new service layer function for player_event_stats table"

Focus on **what changed for the user**, not how it was implemented.

### Example
```markdown
## [Unreleased]

### Added
- Match scheduling calendar view for easier tournament planning

### Fixed
- Player statistics now save correctly even when network errors occur (#146)
- Team logos display properly on mobile devices
```

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

#### Available Error Infrastructure
- **`src/error.rs`**: Central `AppError` enum with `IntoResponse` implementation (available for new code)
- **`src/views/components/error.rs`**: Error display components (error_message, error_state, error_alert)
- **`src/validation.rs`**: Validation helpers with Result-based error handling

#### Error Patterns:

1. **NEVER use `unwrap_or_default()` without logging**
   ```rust
   // BAD - Silent failure
   let countries = get_countries(&db).await.unwrap_or_default();

   // GOOD - Log the error
   let countries = match get_countries(&db).await {
       Ok(countries) => countries,
       Err(e) => {
           tracing::warn!("Failed to load countries for dropdown: {}", e);
           Vec::new()
       }
   };
   ```

2. **NEVER use `ok().flatten()` - It discards error information**
   ```rust
   // BAD - Can't distinguish between not found and database error
   let team = get_team_by_id(&db, id).await.ok().flatten();

   // GOOD - Handle both cases explicitly
   let team = match get_team_by_id(&db, id).await {
       Ok(Some(team)) => team,
       Ok(None) => {
           return Html(error_message(&t, t.messages.error_team_not_found()).into_string())
               .into_response();
       }
       Err(e) => {
           tracing::error!("Database error fetching team {}: {}", id, e);
           return Html(error_message(&t, t.messages.error_failed_to_load_team()).into_string())
               .into_response();
       }
   };
   ```

3. **Three-branch match for `Result<Option<T>>`**
   ```rust
   let entity = match service::get_by_id(&db, id).await {
       Ok(Some(entity)) => entity,
       Ok(None) => {
           // Not found - return 404-appropriate message
           return Html(error_message(&t, t.messages.error_not_found()).into_string());
       }
       Err(e) => {
           // Database error - log and return 500-appropriate message
           tracing::error!("Database error: {}", e);
           return Html(error_message(&t, t.messages.error_failed_to_load()).into_string());
       }
   };
   ```

4. **Validation**: Use validation helpers from `src/validation.rs`
   ```rust
   use crate::validation::validate_name;

   let name = match validate_name(&form.name) {
       Ok(n) => n,
       Err(error) => return Html(error_modal(&t, Some(error)).into_string()),
   };
   ```

5. **Graceful degradation for non-critical data**
   ```rust
   // Dropdown data can fail gracefully with empty list
   let countries = match get_countries(&db).await {
       Ok(countries) => countries,
       Err(e) => {
           tracing::warn!("Failed to load countries: {}", e);
           Vec::new() // Show empty dropdown instead of failing entire page
       }
   };
   ```

6. **AppError for new routes** (optional)
   The `AppError` enum in `src/error.rs` is available for new code:
   ```rust
   use crate::error::AppError;

   pub async fn handler(...) -> Result<impl IntoResponse, AppError> {
       let entity = service::get_by_id(&db, id)
           .await
           .map_err(|e| AppError::database(e, "fetching entity"))?
           .ok_or_else(|| AppError::not_found_with_id("Entity", id))?;

       Ok(Html(page_content(&entity)))
   }
   ```

#### Logging Levels
- `tracing::error!()` - Critical errors that prevent operation (database errors, auth failures)
- `tracing::warn!()` - Non-critical failures where we can degrade gracefully (dropdown data)
- `tracing::debug!()` - Expected conditions (not found, validation errors)

### Database Query Pattern
- Use SQLx QueryBuilder for dynamic queries
- Entity structs for database operations
- DTO structs for API responses
- Fixture SQL files for complex queries