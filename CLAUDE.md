# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a hockey management application with a Rust backend (Axum + SQLx) and Next.js frontend. The system manages hockey tournaments, teams, players, and their relationships across seasons and events.

## Architecture

### Backend (Rust)
- **Framework**: Axum web framework with aide for OpenAPI documentation
- **Database**: SQLite with SQLx for type-safe queries
- **Authentication**: JWT tokens with bcrypt password hashing
- **API Documentation**: Auto-generated OpenAPI specs accessible at `/docs`
- **Port**: Runs on port 8080 (configurable)

### Frontend (Next.js)
- **Framework**: Next.js 15 with App Router
- **Authentication**: NextAuth.js with credentials provider
- **Internationalization**: next-intl for multi-language support
- **Styling**: Tailwind CSS v4
- **State Management**: TanStack Query for server state
- **Port**: Runs on port 3000 (default)

### Database Schema
The database follows a hierarchical structure:
- `country` → `event` → `season` → `team_participation` → `player_contract`
- Junction tables: `team_participation` (teams in seasons), `player_contract` (players in team participations)
- Authentication: `users` table with bcrypt password hashing

## Development Commands

### Backend
```bash
cd backend
cargo run                    # Start development server
cargo build                  # Build the project
cargo test                   # Run tests
cargo check                  # Check code without building
```

### Frontend
```bash
cd frontend
npm run dev                  # Start development server with Turbopack
npm run build                # Build for production
npm start                    # Start production server
npm run lint                 # Run ESLint
```

### Database
```bash
cd backend
# Database is automatically created on first run
# Migrations are applied automatically at startup
```

## Environment Configuration

### Backend Environment Variables
- `DATABASE_URL`: SQLite database file path
- `HMAC_KEY`: JWT signing key for authentication

### Frontend Environment Variables
- `NEXTAUTH_SECRET`: NextAuth.js secret key
- `NEXTAUTH_URL`: Application URL for NextAuth.js

## Key File Locations

### Backend Structure
- `src/main.rs`: Application entry point and OpenAPI configuration
- `src/http.rs`: HTTP server setup and route configuration
- `src/config.rs`: Configuration management
- `src/*/routes.rs`: API route definitions for each domain
- `src/*/service/mod.rs`: Business logic and database operations
- `src/*/service/fixtures/*.sql`: SQL queries for each service
- `migrations/`: Database migration files (chronological order)

### Frontend Structure
- `src/app/`: Next.js App Router pages and API routes
- `src/components/`: Organized React components by purpose:
  - `ui/`: Reusable UI components (Badge, Pager, TableSkeleton, etc.)
  - `layout/`: Layout-specific components (AdminLayoutClient, AdminSidebar)
  - `shared/`: App-specific shared components (LocaleSwitcher, AuthProvider)
  - `features/`: Feature-specific components organized by domain
- `src/ui/pages/`: Client page components that pair with server pages
- `src/hooks/`: Custom React hooks (useDebounce, etc.)
- `src/queries/`: TanStack Query definitions organized by domain
- `src/auth.ts`: NextAuth.js configuration
- `src/middleware.ts`: Next.js middleware for auth and i18n
- `src/i18n/`: Internationalization configuration
- `src/types/`: TypeScript type definitions
- `src/utils/`: Utility functions and helpers
- `messages/`: Translation files (cs.json, en.json)

## Testing

### Backend Tests
- Unit tests are located in `src/*/service/test.rs`
- Tests use fixture data from `src/*/service/fixtures/`
- Run with `cargo test`

### Frontend Tests
- ESLint configuration in `eslint.config.mjs`
- Run linting with `npm run lint`

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

All API endpoints follow RESTful conventions:
- `/auth/*`: Authentication endpoints
- `/event/*`: Event management
- `/country/*`: Country management
- `/team/*`: Team management
- `/player/*`: Player management
- `/season/*`: Season management
- `/team-participation/*`: Team participation management
- `/player-contract/*`: Player contract management
- `/docs`: OpenAPI documentation

## Development Workflow

1. **Backend Changes**: Modify Rust code, restart with `cargo run`
2. **Frontend Changes**: Hot reload enabled with `npm run dev`
3. **Database Changes**: Create new migration files in `migrations/`
4. **API Changes**: Update OpenAPI documentation is auto-generated
5. **Authentication**: All protected routes require JWT authentication

## Common Patterns

### Backend Service Pattern
Each domain follows this structure:
```rust
// routes.rs - API endpoint definitions
// service/mod.rs - Business logic
// service/fixtures/*.sql - SQL queries
// service/test.rs - Unit tests
```

### Frontend Component Pattern
- **Server Components**: Data fetching with `ensureQueryData` for initial state
- **Client Components**: Interactivity with `useSuspenseQuery` for seamless loading
- **Page Structure**: Server page components import client page components from `ui/pages/`
- **Component Organization**:
  - `ui/`: Generic, reusable components (Button, Badge, Table, etc.)
  - `layout/`: Layout-specific components for app structure
  - `shared/`: App-specific shared components across features
  - `features/[domain]/`: Domain-specific components (countries, teams, players)
- **Data Fetching**: TanStack Query with proper hydration and Suspense boundaries
- **State Management**: Minimal local state, server state via TanStack Query
- **Loading States**: Suspense with skeleton components, no manual loading flags
- **Internationalization**: next-intl with translation keys

### Error Handling and Resilience
**CRITICAL**: Always implement proper error boundaries to prevent infinite loops and crashes from malformed API responses.

#### Multi-Layer Error Protection:
1. **API Response Validation**: Validate expected data structure in query functions
   ```typescript
   const validatePaginatedResponse = <T>(data: any): PaginatedResponse<T> => {
     if (!data || typeof data !== 'object') {
       throw new Error('API response is not an object');
     }
     if (Array.isArray(data)) {
       throw new Error('API returned array instead of paginated response - backend may be outdated');
     }
     // Validate required fields...
   }
   ```

2. **React Error Boundaries**: Use `ErrorBoundary` component to catch rendering errors
   ```typescript
   <ErrorBoundary fallback={<ErrorMessage />}>
     <TableComponent data={data.items || []} />
   </ErrorBoundary>
   ```

3. **Query Error Boundaries**: Use `QueryErrorBoundary` for TanStack Query errors
   ```typescript
   <QueryErrorBoundary fallback={<APIErrorMessage />}>
     <Suspense fallback={<Loading />}>
       <DataComponent />
     </Suspense>
   </QueryErrorBoundary>
   ```

4. **Runtime Data Validation**: Additional checks in components as failsafe
   ```typescript
   if (!data || typeof data !== 'object') {
     throw new Error('Invalid data received from API')
   }
   if (!Array.isArray(data.items)) {
     throw new Error('API data.items is not an array')
   }
   ```

5. **Graceful Fallbacks**: Default values prevent crashes
   ```typescript
   data={data.items || []}
   totalItems={data.total || 0}
   ```

#### Error Boundary Components:
- `ErrorBoundary`: General error boundary for component-level errors
- `QueryErrorBoundary`: Specialized for TanStack Query with reset functionality

**Required for all pages with data tables**: Teams, Events, Players, Countries (Management)

### Database Query Pattern
- Use SQLx QueryBuilder for dynamic queries
- Entity structs for database operations
- DTO structs for API responses
- Fixture SQL files for complex queries

## Development Notes

- **Development Environment**:
  - I'm usually running the frontend and backend in the background, so if you want to test something, just tell me