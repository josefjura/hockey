# Hockey Management Application - Rewrite Analysis

## Executive Summary

This document provides a comprehensive analysis of the current hockey management application built with Rust (Axum) backend and Next.js frontend. The purpose is to inform a complete rewrite using Axum/Maud/HTMX/Web Components/Lit to create a simplified, single-binary deployment architecture.

**Current Architecture:**
- Backend: Rust (Axum) + SQLx + SQLite
- Frontend: Next.js 15 + TanStack Query + NextAuth.js
- Authentication: OAuth2-inspired JWT (RS256) with refresh tokens
- Database: SQLite with 213+ countries, comprehensive hockey data model

**Target Architecture:**
- Single binary: Axum + Maud (HTML templating) + HTMX + Lit (Web Components)
- Session-based authentication (simpler than JWT for HTMX)
- Embedded static assets
- Two table component strategies:
  - Small datasets: Client-heavy Lit components
  - Large datasets: Server-rendered HTMX datagrids

---

## Table of Contents

1. [Current Features & Functionality](#current-features--functionality)
2. [Backend Architecture Deep Dive](#backend-architecture-deep-dive)
3. [Frontend Architecture Deep Dive](#frontend-architecture-deep-dive)
4. [Database Schema](#database-schema)
5. [Authentication & Authorization](#authentication--authorization)
6. [UI/UX Patterns & Workflows](#uiux-patterns--workflows)
7. [Internationalization](#internationalization)
8. [Reusable Patterns to Preserve](#reusable-patterns-to-preserve)
9. [Architecture Design for Rewrite](#architecture-design-for-rewrite)
10. [Implementation Plan](#implementation-plan)

---

## Current Features & Functionality

### Admin Dashboard
- **Purpose**: Overview of system state with quick actions
- **Features**:
  - Stats cards showing total teams, players, events, seasons
  - Trend indicators (up/down from last period)
  - Recent activity feed
  - Quick action buttons for common tasks (add new player, add new match,.. )
- **Data**: Aggregated counts from various entities

### Teams Management
- **CRUD Operations**: Create, read, update, delete teams
- **Features**:
  - Paginated table with sorting
  - Search by team name
  - Filter by country
  - Inline edit/delete actions
  - Modal forms for create/edit
  - Country flag display
- **Fields**: name (optional), country (required), logo_path
- **Validation**: Name max 100 chars, logo URL validation, country must exist

### Players Management
- **CRUD Operations**: Create, read, update, delete players
- **Features**:
  - Paginated table with sorting
  - Search by player name
  - Filter by nationality (country)
  - Inline edit/delete actions
  - Modal forms with photo upload support
- **Fields**: name (required), country (required), photo_path
- **Validation**: Name required and non-empty, positive country_id

### Events Management
- **CRUD Operations**: Create, read, update, delete events
- **Features**:
  - Paginated table (tournaments/competitions)
  - Search by event name
  - Filter by host country
  - Optional country assignment
- **Fields**: name (required, max 255 chars), country_id (optional)
- **Examples**: Olympic Games, World Championship, IIHF tournaments

### Seasons Management
- **CRUD Operations**: Create, read, update, delete seasons
- **Features**:
  - Paginated table of event years
  - Filter by event
  - Filter by year
  - Display name customization
- **Fields**: year (required), event_id (required), display_name (optional)
- **Business Logic**: Each season is a specific year/edition of an event

### Matches Management
- **CRUD Operations**: Create, read, update, delete matches
- **Features**:
  - Paginated table with comprehensive filtering
  - Filter by season, team (home/away), status, date range
  - Match detail view with score tracking
  - Score event management (individual goals)
  - Unidentified goal support (scored but scorer unknown initially)
  - Progressive goal identification (convert unidentified → identified)
- **Fields**:
  - season_id, home_team_id, away_team_id (required)
  - home_score_unidentified, away_score_unidentified (default 0)
  - match_date (ISO 8601), status (enum), venue (optional)
- **Score Events**:
  - scorer_id, assist1_id, assist2_id (optional)
  - period, time_minutes, time_seconds, goal_type
  - Supports unknown scorers for later identification

### Management / Settings
- **Country Management**:
  - View 213+ countries (including historical)
  - Enable/disable countries
  - Filter by IIHF membership
  - Filter by historical status
  - Display ISO2, IOC codes
- **System Configuration**: (potential future expansion)

### Authentication
- **Login**: Email/password credentials provider
- **Session Management**: NextAuth.js with JWT
- **Token Management**: Access tokens (15 min) + refresh tokens (7 days)
- **Logout**: Token revocation
- **Protected Routes**: All admin routes require authentication

---

## Backend Architecture Deep Dive

### Three-Layer Architecture Pattern

Every domain follows a consistent pattern:

```
┌─────────────────────────────────────────────────────┐
│                   HTTP Layer                        │
│  Routes (src/{domain}/routes.rs)                    │
│  - Extract params, handle status codes             │
│  - JSON serialization/deserialization              │
│  - OpenAPI documentation                           │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│                 Business Logic Layer                │
│  Business (src/{domain}/business.rs)                │
│  - Validation rules                                 │
│  - Domain logic                                     │
│  - Entity to DTO mapping                           │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│                 Data Access Layer                   │
│  Service (src/{domain}/service/mod.rs)              │
│  - Database queries (SQLx QueryBuilder)            │
│  - Entity structs                                  │
│  - Result<T, sqlx::Error>                          │
└─────────────────────────────────────────────────────┘
```

### API Endpoints Summary

#### Authentication (Public, Rate-Limited)
- `POST /auth/login` - Email/password authentication
- `POST /auth/refresh` - Token refresh with rotation
- `POST /auth/logout` - Token revocation

#### Events (Protected)
- `POST /event` - Create event
- `GET /event` - List events (paginated, filterable)
- `GET /event/{id}` - Get single event
- `PUT /event/{id}` - Update event
- `DELETE /event/{id}` - Delete event

#### Countries (Protected, Read-Mostly)
- `GET /country` - List countries (213+, paginated, filterable)
- `GET /country/{id}` - Get single country
- `PATCH /country/{id}` - Update enabled status

#### Teams (Protected)
- `POST /team` - Create team
- `GET /team` - List teams (paginated, filterable)
- `GET /team/list` - Simple list for dropdowns
- `GET /team/{id}` - Get single team
- `GET /team/{id}/detail` - Get team with participations/roster
- `PUT /team/{id}` - Update team
- `DELETE /team/{id}` - Delete team

#### Players (Protected)
- `POST /player` - Create player
- `GET /player` - List players (paginated, filterable)
- `GET /player/{id}` - Get single player
- `PUT /player/{id}` - Update player
- `DELETE /player/{id}` - Delete player

#### Seasons (Protected)
- `POST /season` - Create season
- `GET /season` - List seasons (paginated, filterable)
- `GET /season/list` - Simple list for dropdowns
- `GET /season/{id}` - Get single season
- `GET /season/{season_id}/team/{team_id}/players` - Get roster
- `PUT /season/{id}` - Update season
- `DELETE /season/{id}` - Delete season

#### Team Participation (Protected)
- `POST /team-participation` - Create participation
- `GET /team-participation` - List all
- `POST /team-participation/find-or-create` - Upsert operation
- `GET /team-participation/{id}` - Get single
- `DELETE /team-participation/{id}` - Delete

#### Player Contracts (Protected)
- `POST /player-contract` - Create contract
- `GET /player-contract` - List all
- `GET /player-contract/{id}` - Get single
- `DELETE /player-contract/{id}` - Delete

#### Matches (Protected)
- `POST /match` - Create match
- `GET /match` - List matches (paginated, complex filtering)
- `GET /match/{id}` - Get single match
- `GET /match/{id}/stats` - Get match with statistics
- `PUT /match/{id}` - Update match
- `DELETE /match/{id}` - Delete match

#### Score Events (Protected)
- `POST /match/{id}/score-events` - Create goal
- `GET /match/{id}/score-events` - List goals for match
- `GET /match/{match_id}/score-events/{event_id}` - Get single goal
- `DELETE /match/{match_id}/score-events/{event_id}` - Delete goal
- `POST /match/{id}/identify-goal` - Convert unidentified → identified

#### Infrastructure
- `GET /health` - Health check with DB connectivity
- `GET /ready` - Readiness check for K8s
- `GET /docs` - OpenAPI documentation

### Service Layer Patterns

**Filter Pattern**:
Every list endpoint supports filtering via `{Domain}Filters` struct:
```rust
pub struct EventFilters {
    pub name: Option<String>,      // LIKE search
    pub country_id: Option<i64>,   // Exact match
}
```

**Pagination Pattern**:
Generic `Paging` struct enforces limits:
```rust
pub struct Paging {
    page: usize,        // min: 1
    page_size: usize,   // range: 1-100
}

pub struct PagedResult<T> {
    items: Vec<T>,
    total: usize,
    page: usize,
    page_size: usize,
    total_pages: usize,
    has_next: bool,
    has_previous: bool,
}
```

**Query Building**:
Dynamic WHERE clauses with SQLx QueryBuilder:
```rust
fn apply_event_filters(query_builder: &mut QueryBuilder, filters: &EventFilters) {
    if let Some(name) = &filters.name {
        query_builder.push(" AND e.name LIKE '%' || ")
            .push_bind(name)
            .push(" || '%'");
    }
    if let Some(country_id) = filters.country_id {
        query_builder.push(" AND e.country_id = ")
            .push_bind(country_id);
    }
}
```

**Entity/DTO Pattern**:
Separate types for database operations and API responses:
- `CreateEventEntity` - INSERT operations
- `UpdateEventEntity` - UPDATE operations
- `EventWithCountryEntity` - Complex queries with JOINs
- `Event` - API response DTO

### Business Rules

**Events**:
- Name: required, max 255 chars, not empty
- Country ID: optional, but if provided must be positive

**Teams**:
- Name: optional, max 100 chars
- Country ID: required, must be positive
- Logo path: must be HTTP/HTTPS URL or absolute path

**Players**:
- Name: required, not empty
- Country ID: required, must be positive

**Matches**:
- Season, team IDs: required, must be positive
- Home team ≠ Away team
- Scores: non-negative
- Match date: ISO 8601 format
- Status: enum values (scheduled, in_progress, finished, cancelled)

**Scoring**:
- Supports unidentified goals (team scored but scorer unknown)
- Progressive identification via `identify_goal` endpoint
- Period: 1-3 (regular), 4 (OT), 5 (SO)
- Time: 0-60 minutes, 0-59 seconds

### Error Handling

**Error Types**:
```rust
pub enum AppError {
    TeamNotFound { id: i64 },
    EventNotFound { id: i64 },
    MatchNotFound { id: i64 },
    InvalidInput { message: String },
    Database(sqlx::Error),
    Internal(anyhow::Error),
    Unauthorized,
}
```

**HTTP Mapping**:
- 400 BAD_REQUEST - InvalidInput
- 401 UNAUTHORIZED - Auth failures
- 404 NOT_FOUND - Entity not found
- 500 INTERNAL_SERVER_ERROR - Database/internal errors

**Error Response**:
```rust
pub struct ErrorResponse {
    error: String,                    // Human-readable message
    error_id: Uuid,                   // Unique tracking ID
    error_details: Option<Value>,     // Structured details
}
```

---

## Frontend Architecture Deep Dive

### Next.js 15 App Router Structure

```
src/app/
├── [locale]/                          # i18n wrapper
│   ├── page.tsx                       # Landing page (public)
│   ├── auth/signin/page.tsx           # Sign-in (public)
│   └── (admin)/                       # Protected route group
│       ├── layout.tsx                 # Admin layout with sidebar
│       ├── dashboard/page.tsx
│       ├── teams/page.tsx
│       ├── players/page.tsx
│       ├── events/page.tsx
│       ├── seasons/page.tsx
│       ├── matches/
│       │   ├── page.tsx               # Match list
│       │   └── [id]/page.tsx          # Match detail
│       └── management/page.tsx        # Countries/settings
```

### Component Organization

```
src/components/
├── ui/                                # Reusable UI primitives
│   ├── teams-table.tsx
│   ├── players-table.tsx
│   ├── events-table.tsx
│   ├── matches-table.tsx
│   ├── *-create-dialog.tsx
│   ├── *-edit-dialog.tsx
│   ├── badge.tsx
│   ├── pager.tsx
│   ├── stats-card.tsx
│   └── table-skeleton.tsx
├── layout/                            # Layout components
│   ├── admin-layout-client.tsx
│   └── admin-sidebar.tsx
├── shared/                            # Shared across features
│   ├── auth-provider.tsx
│   ├── locale-switcher.tsx
│   └── quick-actions.tsx
├── error-boundary.tsx
└── query-error-boundary.tsx
```

### Data Fetching Pattern

**Server Components** (for initial data):
```typescript
// src/queries/teams.ts
export const fetchTeamList = async (
  page: number,
  searchTerm?: string,
  pageSize: number = 15,
  accessToken?: string
): Promise<PaginatedResponse<Team>> => {
  if (accessToken) {
    const client = createClientApiClient(accessToken);
    return client(`/team?page=${page}&page_size=${pageSize}&name=${searchTerm || ''}`);
  }
  return apiGet(`/team?page=${page}&page_size=${pageSize}&name=${searchTerm || ''}`);
}

export const teamQueries = {
  list: (search, page, size, token) =>
    queryOptions({
      queryKey: ['teams', search, page, size, token],
      queryFn: () => fetchTeamList(page, search, size, token),
      staleTime: 5 * 60 * 1000, // 5 minutes
    }),
}
```

**Client Components** (with interactivity):
```typescript
const { data } = useSuspenseQuery(
  teamQueries.list(debouncedSearchTerm, page, pageSize, session?.accessToken)
)
```

### Form Pattern (React Hook Form + Zod)

```typescript
const teamCreateSchema = z.object({
  name: z.string().optional(),
  country_id: z.string().min(1, 'Country is required'),
})

const { register, handleSubmit, watch, reset, formState } =
  useForm<TeamCreateFormData>({
    resolver: zodResolver(teamCreateSchema),
    defaultValues: { name: '', country_id: '' },
  })

const createTeamMutation = useCreateTeam()
const onSubmit = async (data: TeamCreateFormData) => {
  await createTeamMutation.mutateAsync({
    teamData: {
      name: data.name?.trim() || null,
      country_id: data.country_id
    },
    accessToken: session?.accessToken
  })
  reset()
  onClose()
}
```

### Table Pattern (TanStack Table)

**Features**:
- Column sorting (asc/desc)
- Inline actions (edit, delete)
- Responsive sizing
- Empty state handling

**Example**:
```typescript
const columns = [
  columnHelper.accessor('name', {
    header: ({ column }) => (
      <button onClick={() => column.toggleSorting()}>
        Team Name {sortIcon}
      </button>
    ),
    cell: info => <span className="font-semibold">{info.getValue()}</span>,
  }),
  columnHelper.accessor('country.name', {
    header: 'Country',
    cell: info => (
      <div className="flex items-center">
        <img src={`/flags/${info.row.original.country.iso2_code}.svg`} />
        {info.getValue()}
      </div>
    ),
  }),
  // ... actions column
]
```

### Error Handling Layers

1. **API Response Validation**:
```typescript
const validatePaginatedResponse = <T>(data: unknown): PaginatedResponse<T> => {
  if (!data || typeof data !== 'object') {
    throw new Error('API response is not an object');
  }
  if (Array.isArray(data)) {
    throw new Error('API returned array instead of paginated response');
  }
  return data as PaginatedResponse<T>;
}
```

2. **React Error Boundaries**:
```typescript
<ErrorBoundary fallback={<ErrorMessage />}>
  <TableComponent data={data.items || []} />
</ErrorBoundary>
```

3. **Query Error Boundaries**:
```typescript
<QueryErrorBoundary fallback={<APIErrorMessage />}>
  <Suspense fallback={<Loading />}>
    <DataComponent />
  </Suspense>
</QueryErrorBoundary>
```

4. **Graceful Fallbacks**:
```typescript
data={data?.items || []}
totalItems={data?.total || 0}
```

### UI/UX Patterns

**Search + Debounce**:
```typescript
const [searchTerm, setSearchTerm] = useState('')
const debouncedSearchTerm = useDebounce(searchTerm, 300)

useEffect(() => {
  setPage(0)  // Reset pagination on search change
}, [debouncedSearchTerm])
```

**Pagination**:
- Shows "X to Y of Z results"
- Ellipsis for large page counts
- Mobile: Previous/Next only
- Desktop: Full page numbers
- Disabled state for boundaries

**Loading States**:
- Skeleton screens with animate-pulse
- Spinner in submit buttons
- Suspense boundaries with fallbacks
- Disabled inputs during mutations

**User Feedback**:
- Toast notifications (react-hot-toast)
- Success: `toast.success('Team created')`
- Error: `toast.error('Failed to create team')`
- Loading indicators in buttons

---

## Database Schema

### Entity Relationship Overview

```
Authentication:
users (1) ──→ (N) refresh_tokens

Core Hierarchy:
country (1) ──→ (N) event (1) ──→ (N) season
country (1) ──→ (N) team
country (1) ──→ (N) player

Junction Tables:
season (1) ──→ (N) team_participation (N) ──→ (1) team
team_participation (1) ──→ (N) player_contract (N) ──→ (1) player

Match System:
season (1) ──→ (N) match
team (1) ──→ (N) match (as home/away)
match (1) ──→ (N) score_event (N) ──→ (1) player (scorer/assists)
```

### Key Tables

#### users
- id (PK), email (UNIQUE), password_hash, name, created_at, updated_at
- Authentication: bcrypt password hashing

#### refresh_tokens
- id (PK), token (UNIQUE, SHA-256 hash), user_id (FK), expires_at, created_at, revoked_at
- Token Management: 7-day expiry, revocation support

#### country
- id (PK), name, iihf, iso2Code, iocCode, isHistorical, years, enabled
- 213+ countries including historical (Soviet Union, East Germany, etc.)
- Historical countries have years field: "1952-1991", "1956-1988"

#### event
- id (PK), name, country_id (FK, optional), created_at, updated_at
- Tournaments/competitions (Olympics, World Championship)

#### season
- id (PK), year, display_name, event_id (FK), created_at, updated_at
- Specific year/edition of event

#### team
- id (PK), name, country_id (FK), logo_path, created_at, updated_at
- Hockey teams representing countries

#### player
- id (PK), name, country_id (FK), photo_path, created_at, updated_at
- Individual players with nationality

#### team_participation
- id (PK), team_id (FK), season_id (FK), created_at, updated_at
- Junction: Teams in specific seasons

#### player_contract
- id (PK), team_participation_id (FK), player_id (FK), created_at, updated_at
- Junction: Players contracted to teams in seasons

#### match
- id (PK), season_id (FK), home_team_id (FK), away_team_id (FK)
- home_score_unidentified, away_score_unidentified
- match_date, status, venue, created_at, updated_at
- Constraints: home_team_id ≠ away_team_id, scores ≥ 0

#### score_event
- id (PK), match_id (FK CASCADE), team_id (FK)
- scorer_id (FK, nullable), assist1_id (FK, nullable), assist2_id (FK, nullable)
- period (1-5), time_minutes (0-60), time_seconds (0-59), goal_type
- Constraints: Valid period/time ranges

### Indexes

**refresh_tokens**: token, user_id, expires_at, (user_id, revoked_at)
**match**: season_id, home_team_id, away_team_id, match_date, status
**score_event**: match_id, team_id, scorer_id

### Business Rules at DB Level

- Team self-play prevention: `home_team_id ≠ away_team_id`
- Non-negative scores: `home_score_unidentified ≥ 0`
- Valid periods: `1 ≤ period ≤ 5`
- Valid time: `0 ≤ time_minutes ≤ 60`, `0 ≤ time_seconds ≤ 59`
- Unique email per user
- Unique refresh tokens
- Cascade delete: user → refresh_tokens, match → score_events

---

## Authentication & Authorization

### Current JWT System (RS256)

**Token Types**:
- Access Token: 15 minutes, used for API calls
- Refresh Token: 7 days, stored in database

**Flow**:
1. Login: Email/password → access + refresh tokens
2. API calls: `Authorization: Bearer <access_token>`
3. Token refresh: POST /auth/refresh with refresh token
4. Token rotation: Old refresh token revoked, new tokens issued
5. Logout: Refresh token marked as revoked

**Security Features**:
- RSA 4096-bit keys (RS256)
- Bcrypt password hashing
- SHA-256 refresh token hashing in DB
- Token revocation support
- Rate limiting (10 req/sec on auth endpoints)
- CORS validation (strict in production)

**NextAuth.js Integration**:
- Credentials provider
- JWT callback handles token refresh
- Session includes accessToken
- Middleware protects routes
- 401 responses trigger signin redirect

### Proposed Session-Based Auth (for HTMX)

**Simpler approach for HTMX**:
- Session cookies instead of JWT
- Server-side session storage
- No complex token rotation
- CSRF protection
- Same security guarantees

**Benefits**:
- Simpler implementation
- Better HTMX integration
- No client-side token management
- Standard cookie-based auth

---

## UI/UX Patterns & Workflows

### Tables (Primary Pattern)

**Two strategies needed**:

1. **Client-Heavy (Small Datasets)**:
   - Lit Web Components
   - Client-side sorting/filtering
   - Instant interactions
   - Example: Countries list (213 items)

2. **Server-Heavy (Large Datasets)**:
   - HTMX-driven datagrids
   - Server-side pagination/sorting/filtering
   - Each interaction makes request
   - Example: Matches with complex filters

**Common Features**:
- Column sorting (asc/desc toggles)
- Inline actions (edit, delete)
- Search/filter inputs
- Pagination controls
- Empty state handling
- Loading states
- Responsive design

### Forms

**Patterns**:
- Modal dialogs for create/edit
- Inline validation
- Keyboard shortcuts (Shift+Enter, Escape)
- Auto-focus management
- Disabled during submission
- Success/error notifications
- Country/team selection with previews

**Validation**:
- Client-side validation first
- Server-side validation always
- Clear error messages
- Field-level feedback
- Cross-field validation (e.g., home ≠ away team)

### Search & Filtering

**Patterns**:
- Debounced search input (300ms)
- Multiple filters (dropdowns)
- Reset pagination on filter change
- Clear filters button
- Filter badges showing active filters

### Navigation

**Sidebar Pattern**:
- Icon + label + description
- Active route highlighting
- Collapsible on mobile
- User info at bottom
- Logout button

### Feedback Mechanisms

- Toast notifications
- Loading spinners
- Skeleton screens
- Progress indicators
- Success/error states
- Disabled states

---

## Internationalization

### Current Setup

**Languages**: English (en), Czech (cs)
**Structure**: `messages/en.json`, `messages/cs.json`
**Implementation**: next-intl with route prefixing

**Message Organization**:
```json
{
  "Navigation": {
    "dashboard": "Dashboard",
    "teams": "Teams",
    "signOut": "Sign Out"
  },
  "Teams": {
    "title": "Teams Management",
    "searchPlaceholder": "Search teams...",
    "createButton": "Create Team"
  }
}
```

### For Rewrite

**Keep i18n requirement**:
- Czech + English support
- Server-side message loading
- Route-based locale switching
- Message files in same structure

**Options**:
- fluent-rs (Rust i18n library)
- Simple HashMap-based approach
- Template engine integration (Maud)

---

## Reusable Patterns to Preserve

### Backend Patterns

1. **Three-Layer Architecture**: Keep separation of routes, business logic, service
2. **Pagination**: Reuse `Paging` struct and PagedResult pattern
3. **Filtering**: Dynamic filter builders for all list endpoints
4. **Error Handling**: Typed errors with HTTP mapping
5. **Validation**: Business rule validation in business layer
6. **Query Building**: SQLx QueryBuilder for dynamic queries

### Frontend Patterns (Translate to HTMX/Lit)

1. **Table Components**: Two-strategy approach (client/server)
2. **Modal Forms**: Create/edit dialogs with validation
3. **Search + Debounce**: Debounced inputs for filtering
4. **Pagination**: Consistent pager component
5. **Toast Notifications**: Success/error feedback
6. **Loading States**: Skeleton screens, spinners
7. **Error Boundaries**: Graceful error handling

### Data Patterns

1. **Entity/DTO Separation**: Keep for type safety
2. **Cascading Deletes**: Preserve where appropriate
3. **Historical Data**: Country years pattern
4. **Junction Tables**: team_participation, player_contract
5. **Score Tracking**: Unidentified → identified goal flow

---

## Architecture Design for Rewrite

### Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Backend | Axum | Web framework |
| Templating | Maud | Type-safe HTML generation |
| Interactivity | HTMX | Dynamic HTML updates |
| Components | Lit | Web Components for rich client-side |
| Database | SQLx + SQLite | Type-safe queries |
| Auth | Sessions + Cookies | Simplified auth for HTMX |
| i18n | fluent-rs | Message localization |
| Styling | Tailwind CSS | Utility-first CSS |

### Application Structure

```
src/
├── main.rs                    # Entry point, server setup
├── config.rs                  # Configuration management
├── routes/                    # Route handlers (return HTML)
│   ├── auth.rs               # Login, logout
│   ├── dashboard.rs
│   ├── teams.rs
│   ├── players.rs
│   ├── events.rs
│   ├── seasons.rs
│   ├── matches.rs
│   └── management.rs
├── views/                     # Maud templates
│   ├── layout.rs             # Base layout with sidebar
│   ├── components/           # Reusable template components
│   │   ├── table.rs
│   │   ├── form.rs
│   │   ├── pagination.rs
│   │   └── modal.rs
│   └── pages/                # Page templates
│       ├── dashboard.rs
│       ├── teams.rs
│       └── ...
├── business/                  # Business logic (preserve from current)
├── service/                   # Data access (preserve from current)
├── auth/                      # Session-based auth
│   ├── session.rs
│   ├── middleware.rs
│   └── password.rs
├── i18n/                      # Internationalization
│   ├── mod.rs
│   └── messages/
│       ├── en.ftl
│       └── cs.ftl
└── web_components/            # Lit components (compiled separately)
    ├── small-table.ts        # Client-side table for small datasets
    └── ...
```

### Static Assets

```
static/
├── css/
│   └── styles.css            # Tailwind output
├── js/
│   ├── htmx.min.js
│   └── components.js         # Compiled Lit components
├── flags/                    # Country flags
└── images/
```

### Component Strategy

#### Server-Rendered Components (HTMX)
**Use for**: Large datasets, complex filtering, pagination
**Examples**: Matches table, Teams table, Players table

**Pattern**:
```rust
// Route handler returns HTML
pub async fn teams_list(
    Extension(ctx): Extension<AppContext>,
    Query(params): Query<TeamsListParams>,
) -> Result<Html<String>, AppError> {
    let teams = service::get_teams(&ctx.db, params).await?;
    Ok(Html(views::teams::teams_table(teams, ctx.locale)))
}

// HTMX in template
html! {
    div hx-get="/teams/list?page=2" hx-trigger="click" hx-target="#table-body" {
        "Load More"
    }
}
```

#### Client-Side Components (Lit)
**Use for**: Small datasets, rich interactions, instant feedback
**Examples**: Country selector, player picker, inline editors

**Pattern**:
```typescript
@customElement('hockey-small-table')
export class SmallTable extends LitElement {
  @property({ type: Array }) data = [];
  @property({ type: Boolean }) sortable = false;

  render() {
    return html`
      <table>
        ${this.data.map(row => html`<tr>...</tr>`)}
      </table>
    `;
  }
}
```

### Authentication Strategy

**Session-Based Auth**:
```rust
// Session struct
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub expires_at: DateTime<Utc>,
}

// Middleware
pub async fn require_auth<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, Redirect> {
    let session = get_session_from_cookie(&req)?;
    if session.is_expired() {
        return Err(Redirect::to("/auth/signin"));
    }
    req.extensions_mut().insert(session);
    Ok(next.run(req).await)
}
```

**CSRF Protection**:
- Token in session
- Hidden input in all forms
- Validation middleware

### Internationalization Strategy

**fluent-rs Integration**:
```rust
// i18n/mod.rs
pub struct I18n {
    bundles: HashMap<String, FluentBundle>,
}

impl I18n {
    pub fn t(&self, locale: &str, key: &str) -> String {
        self.bundles.get(locale)
            .and_then(|b| b.get_message(key))
            .map(|m| m.value())
            .unwrap_or_else(|| key.to_string())
    }
}

// Usage in templates
html! {
    h1 { (ctx.i18n.t("teams.title")) }
}
```

### Table Components Design

#### Small Table (Lit Component)
**Features**:
- Client-side sorting
- Client-side filtering
- Instant feedback
- Max 500 rows

**Implementation**:
```typescript
@customElement('hockey-table-small')
export class SmallTable extends LitElement {
  @property({ type: Array }) columns = [];
  @property({ type: Array }) data = [];
  @state() private sortColumn: string = '';
  @state() private sortDirection: 'asc' | 'desc' = 'asc';
  @state() private filterText: string = '';

  get filteredData() {
    return this.data.filter(row =>
      Object.values(row).some(val =>
        String(val).toLowerCase().includes(this.filterText.toLowerCase())
      )
    );
  }

  get sortedData() {
    if (!this.sortColumn) return this.filteredData;
    return [...this.filteredData].sort((a, b) => {
      // Sorting logic
    });
  }

  render() {
    return html`
      <input @input=${this.onFilterInput} placeholder="Search...">
      <table>
        <thead>
          <tr>
            ${this.columns.map(col => html`
              <th @click=${() => this.toggleSort(col.key)}>
                ${col.label}
                ${this.renderSortIcon(col.key)}
              </th>
            `)}
          </tr>
        </thead>
        <tbody>
          ${this.sortedData.map(row => html`
            <tr>
              ${this.columns.map(col => html`
                <td>${row[col.key]}</td>
              `)}
            </tr>
          `)}
        </tbody>
      </table>
    `;
  }
}
```

#### Large Table (HTMX Datagrid)
**Features**:
- Server-side pagination
- Server-side sorting
- Server-side filtering
- Optimized for thousands of rows

**Implementation**:
```rust
// Maud template
pub fn teams_table(teams: PagedResult<Team>, filters: TeamFilters, i18n: &I18n) -> Markup {
    html! {
        div.datagrid {
            // Search input with HTMX
            input.search-input
                hx-get="/teams/list"
                hx-trigger="keyup changed delay:300ms"
                hx-target="#teams-table-body"
                hx-include="[name='filters']"
                name="search"
                placeholder=(i18n.t("teams.search_placeholder"));

            // Table
            table {
                thead {
                    tr {
                        th {
                            a hx-get=(format!("/teams/list?sort=name&dir={}", toggle_dir(filters.sort_dir)))
                              hx-target="#teams-table-body" {
                                (i18n.t("teams.name"))
                                (sort_icon(&filters))
                            }
                        }
                        // More columns...
                    }
                }
                tbody#teams-table-body {
                    @for team in teams.items {
                        tr {
                            td { (team.name) }
                            // More cells...
                            td {
                                button hx-get=(format!("/teams/{}/edit", team.id))
                                       hx-target="#modal-container" {
                                    "Edit"
                                }
                                button hx-delete=(format!("/teams/{}", team.id))
                                       hx-confirm="Are you sure?" {
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            }

            // Pagination
            (pagination_component(&teams))
        }
    }
}
```

### Form Handling

**Modal Forms with HTMX**:
```rust
// Create form route
pub async fn team_create_form(Extension(ctx): Extension<AppContext>) -> Html<String> {
    let countries = service::get_countries(&ctx.db).await?;
    Ok(Html(views::teams::create_modal(countries, ctx.i18n)))
}

// Submit handler
pub async fn team_create(
    Extension(ctx): Extension<AppContext>,
    Form(data): Form<CreateTeamData>,
) -> Result<Response, AppError> {
    // Validate
    validate_team_data(&data)?;

    // Create
    let team = business::create_team(&ctx.db, data).await?;

    // Return: close modal + refresh table
    Ok(Response::builder()
        .header("HX-Trigger", "teamCreated")
        .body(html! { div { "Success!" } }.into())
        .unwrap())
}

// Template
pub fn create_modal(countries: Vec<Country>, i18n: &I18n) -> Markup {
    html! {
        div.modal#team-create-modal {
            form hx-post="/teams" hx-target="this" {
                div.form-group {
                    label { (i18n.t("teams.name")) }
                    input type="text" name="name";
                }
                div.form-group {
                    label { (i18n.t("teams.country")) }
                    select name="country_id" required {
                        @for country in countries {
                            option value=(country.id) { (country.name) }
                        }
                    }
                }
                button type="submit" { (i18n.t("common.save")) }
            }
        }
    }
}
```

### Error Handling

**Preserve error types**:
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
                let html = views::errors::not_found(entity, id);
                (StatusCode::NOT_FOUND, Html(html.into())).into_response()
            }
            AppError::InvalidInput { message } => {
                let html = views::errors::validation_error(message);
                (StatusCode::BAD_REQUEST, Html(html.into())).into_response()
            }
            // ... other cases
        }
    }
}
```

### Toast Notifications

**Client-Side Toast System**:
```typescript
// web_components/toast.ts
@customElement('hockey-toast')
export class Toast extends LitElement {
  @property({ type: String }) message = '';
  @property({ type: String }) type: 'success' | 'error' | 'info' = 'info';
  @property({ type: Boolean }) visible = false;

  show(message: string, type: string) {
    this.message = message;
    this.type = type;
    this.visible = true;
    setTimeout(() => { this.visible = false; }, 3000);
  }

  render() {
    return html`
      <div class="toast ${this.type} ${this.visible ? 'visible' : 'hidden'}">
        ${this.message}
      </div>
    `;
  }
}

// Usage: HTMX triggers
// Server sends: HX-Trigger: {"showToast": {"message": "Team created", "type": "success"}}
// Client listens:
document.body.addEventListener('showToast', (e) => {
  const toast = document.querySelector('hockey-toast');
  toast.show(e.detail.message, e.detail.type);
});
```

---

## Implementation Plan

### Phase 1: Foundation (Week 1-2)

**Goals**: Basic server, auth, one complete CRUD flow

**Tasks**:
1. Project setup
   - Initialize Rust project with Axum
   - Set up SQLx with existing database
   - Configure Maud for templating
   - Add HTMX and Tailwind CSS
   - Set up Lit for Web Components

2. Authentication system
   - Session management (in-memory or Redis)
   - Login/logout routes
   - Session middleware
   - CSRF protection
   - Password hashing (bcrypt)

3. Base layout
   - Admin layout with sidebar
   - Navigation component
   - Locale switcher
   - Flash message system
   - Base styles (Tailwind)

4. i18n setup
   - fluent-rs integration
   - Load en.ftl and cs.ftl
   - Template helpers
   - Route locale handling

5. First CRUD: Teams
   - List teams (HTMX datagrid)
   - Create team (modal form)
   - Edit team (modal form)
   - Delete team (confirmation)
   - Country selector component (Lit)

**Deliverable**: Working auth + teams management

### Phase 2: Core Entities (Week 3-4)

**Goals**: All main CRUD operations

**Tasks**:
1. Events management
   - List, create, edit, delete
   - Country filter
   - Search by name

2. Players management
   - List, create, edit, delete
   - Country filter
   - Search by name
   - Photo upload support

3. Seasons management
   - List, create, edit, delete
   - Event filter
   - Year filter

4. Countries management
   - List with filters (IIHF, historical)
   - Enable/disable toggle
   - Small table (Lit component)

**Deliverable**: All basic entities manageable

### Phase 3: Table Components (Week 5)

**Goals**: Finalize table strategies

**Tasks**:
1. Small Table Lit Component
   - Client-side sorting
   - Client-side filtering
   - Column configuration
   - Action buttons
   - Empty state
   - Loading state
   - Used for: Countries (213 items)

2. Large Table HTMX Component
   - Server-side pagination
   - Server-side sorting
   - Server-side filtering
   - Multiple filter inputs
   - Pagination controls
   - Loading indicators
   - Used for: Teams, Players, Events, Seasons

3. Pagination Component
   - Reusable Maud component
   - Desktop: full page numbers
   - Mobile: prev/next only
   - Result count display

**Deliverable**: Robust, reusable table system

### Phase 4: Matches & Scoring (Week 6-7)

**Goals**: Complex match management

**Tasks**:
1. Matches list
   - Complex filtering (season, team, status, date)
   - HTMX datagrid
   - Status badges
   - Date display

2. Match create/edit
   - Season selector
   - Team selectors (home/away)
   - Score inputs
   - Date/time picker
   - Venue input
   - Validation (home ≠ away)

3. Match detail page
   - Match info display
   - Score events list
   - Add goal (score event)
   - Identify unidentified goal
   - Delete goal
   - Score totals

4. Score events management
   - Scorer/assist selectors
   - Period/time inputs
   - Goal type selector
   - Team validation

**Deliverable**: Complete match/scoring system

### Phase 5: Dashboard & Polish (Week 8)

**Goals**: Dashboard, final polish, testing

**Tasks**:
1. Dashboard
   - Stats cards (total teams, players, events, seasons)
   - Recent activity feed
   - Quick actions
   - Aggregated queries

2. UI Polish
   - Loading states everywhere
   - Error states
   - Empty states
   - Toast notifications
   - Confirmation dialogs
   - Keyboard shortcuts

3. Responsive design
   - Mobile navigation
   - Mobile tables
   - Mobile forms
   - Touch-friendly interactions

4. Testing
   - Manual testing all flows
   - Edge cases
   - Error scenarios
   - Different screen sizes

**Deliverable**: Production-ready application

### Phase 6: Deployment & Documentation (Week 9)

**Goals**: Single-binary deployment, documentation

**Tasks**:
1. Asset embedding
   - Embed static files in binary
   - rust-embed crate
   - CSS, JS, flags, images

2. Build optimization
   - Release build
   - Minify CSS/JS
   - Optimize images
   - Compression

3. Deployment
   - Systemd service
   - Nginx reverse proxy (optional)
   - Environment configuration
   - Database migrations

4. Documentation
   - README with setup instructions
   - DEPLOYMENT.md
   - API documentation (minimal, internal)
   - User guide (Czech + English)

**Deliverable**: Deployed, documented application

---

## Migration Strategy

### Data Migration
- **No migration needed**: Reuse existing SQLite database
- **Preserve schema**: All tables remain unchanged
- **Reuse migrations**: Copy migrations/ directory

### Code Reuse
- **Business logic**: Port validation rules, business rules
- **Service layer**: Port SQL queries (SQLx compatible)
- **Filters**: Reuse filter patterns
- **Pagination**: Reuse Paging struct

### Feature Parity Checklist
- [ ] Authentication (login, logout, sessions)
- [ ] Teams (CRUD, search, filter)
- [ ] Players (CRUD, search, filter)
- [ ] Events (CRUD, search, filter)
- [ ] Seasons (CRUD, search, filter)
- [ ] Countries (list, enable/disable, filters)
- [ ] Matches (CRUD, complex filters)
- [ ] Score events (add, edit, delete, identify)
- [ ] Dashboard (stats, activity)
- [ ] i18n (Czech, English)
- [ ] Responsive design
- [ ] Toast notifications
- [ ] Error handling

---

## Key Decisions

### Technology Choices

| Decision | Rationale |
|----------|-----------|
| Axum over Actix | Better async/await integration, tower middleware |
| Maud over Askama | Type-safe, compile-time HTML, Rust syntax |
| HTMX over Alpine.js | Better for server-rendered updates |
| Lit over vanilla Web Components | Better DX, reactive properties, templating |
| Sessions over JWT | Simpler for HTMX, no token management complexity |
| fluent-rs over simple HashMap | Professional i18n, plural rules, ICU support |
| SQLx over Diesel | Already in use, async-first, compile-time checks |

### Architecture Principles

1. **Server-First**: Default to server-rendered, use client-side sparingly
2. **Simplicity**: Single binary, minimal dependencies, straightforward code
3. **Type Safety**: Leverage Rust's type system, compile-time guarantees
4. **Progressive Enhancement**: Works without JS, enhanced with JS
5. **Separation of Concerns**: Routes, business, service layers preserved
6. **Reusability**: Components, templates, utilities
7. **Performance**: Minimize JS bundle, lazy load components
8. **Accessibility**: Semantic HTML, keyboard navigation, ARIA labels

---

## Appendix: Quick Reference

### File Count Estimates

**Backend (Rust)**:
- Routes: ~10 files
- Views: ~20 files (layouts + pages + components)
- Business: ~10 files (one per domain)
- Service: ~10 files (one per domain)
- Auth: ~3 files
- i18n: ~2 files + message files
- Total: ~55 Rust files

**Frontend (Lit)**:
- Components: ~5 files (small-table, country-selector, toast, etc.)
- Total: ~5 TypeScript files

**Static Assets**:
- CSS: 1 file (Tailwind output)
- JS: 2 files (HTMX + compiled Lit components)
- Flags: 213 SVG files
- Total: ~220 static files

**Lines of Code Estimate**: 15,000-20,000 lines (Rust + TypeScript + templates)

### Development Timeline

- **Phase 1**: 2 weeks (foundation + teams)
- **Phase 2**: 2 weeks (core entities)
- **Phase 3**: 1 week (table components)
- **Phase 4**: 2 weeks (matches/scoring)
- **Phase 5**: 1 week (dashboard/polish)
- **Phase 6**: 1 week (deployment/docs)
- **Total**: 9 weeks

### Success Criteria

- [ ] Feature parity with current application
- [ ] Single binary deployment
- [ ] Fast page loads (<500ms)
- [ ] Works without JavaScript (progressive enhancement)
- [ ] Mobile-responsive
- [ ] Czech + English i18n
- [ ] Production-ready error handling
- [ ] Comprehensive user feedback (toasts, loading states)
- [ ] Accessible (keyboard navigation, screen readers)
- [ ] Maintainable codebase

---

**Document Version**: 1.0
**Last Updated**: 2025-12-16
**Author**: Claude Code (AI Assistant)
**Purpose**: Inform complete rewrite of hockey management application
