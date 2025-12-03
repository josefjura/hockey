# Hockey Management System - Monorepo

A modern full-stack web application for managing hockey tournaments, teams, players, and seasons. Built with a Rust backend API and Next.js frontend.

## ⚠️ Active Migration

This project is currently undergoing an **OAuth2 Authentication Migration** to resolve critical security vulnerabilities.

- **Track progress**: [View GitHub Milestone](https://github.com/josefjura/hockey/milestone/1)
- **Get started**: See `README_AUTH_MIGRATION.md` for complete workflow
- **Quick view**: `gh issue list --milestone "OAuth2 Authentication Migration"`

## 🏗️ Project Structure

This is a monorepo containing:

- **Frontend**: Next.js 15 application with TypeScript and Tailwind CSS
- **Backend**: Rust API server with SQLx and Axum

```
├── frontend/           # Next.js frontend application
├── backend/           # Rust API server
└── .github/workflows/ # CI/CD pipelines
```

## 🚀 Features

- **Complete Hockey Management** - Teams, Players, Events, and Seasons
- **Real-time Data Management** with optimistic updates
- **Multi-language Support** (English & Czech)
- **Modern UI/UX** with responsive design
- **Form Validation** with real-time feedback
- **Search & Pagination** for all data tables
- **Secure Authentication** - OAuth2-inspired JWT with refresh tokens
- **RESTful API** with OpenAPI documentation

## 🛠️ Tech Stack

### Frontend
- **Framework**: [Next.js 15](https://nextjs.org/) with App Router
- **Language**: TypeScript
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/)
- **Authentication**: [NextAuth.js](https://next-auth.js.org/) 
- **State Management**: [TanStack Query](https://tanstack.com/query) for server state
- **Forms**: [React Hook Form](https://react-hook-form.com/) + [Zod](https://zod.dev/) validation
- **Internationalization**: [next-intl](https://next-intl-docs.vercel.app/)

### Backend
- **Framework**: [Axum](https://github.com/tokio-rs/axum) for async web server
- **Language**: Rust (edition 2021)
- **Database**: SQLite with [SQLx](https://github.com/launchbadge/sqlx)
- **Authentication**: JWT (RS256) with [jsonwebtoken](https://github.com/Keats/jsonwebtoken), bcrypt for password hashing
- **API Documentation**: [aide](https://github.com/tamasfe/aide) for OpenAPI
- **Validation**: Serde for JSON serialization

## 🐳 CI/CD & Deployment

The project uses GitHub Actions for automated building and deployment:

### Docker Images
- **Frontend**: Built and pushed to `ghcr.io/josefjura/hockey/frontend`
- **Backend**: Built and pushed to `ghcr.io/josefjura/hockey/backend`

### Build Process
- Automated builds on push to `master` branch
- Multi-stage Docker builds for optimized images
- Container registry with GitHub Container Registry
- Signed container images with cosign

### Recent Fixes Applied
- ✅ Fixed Next.js async params type compatibility
- ✅ Resolved Docker build context and Dockerfile location issues  
- ✅ Removed SQLx compile-time macros for offline builds
- ✅ Updated to Rust nightly for edition2024 dependency support
- ✅ Fixed git submodule configuration issues

## 📋 Prerequisites

- Node.js 18+
- Yarn (preferred) or npm

## 🚀 Getting Started

### Prerequisites
- Node.js 18+ (for frontend)
- Rust 1.81+ or nightly (for backend)
- SQLite (for database)

### Development Setup

#### Backend API Server
```bash
cd backend

# Generate RSA keys for JWT (first time only)
./scripts/generate_keys.sh

# Create .env file from example
cp .env.example .env
# Edit .env and set your configuration

# Install dependencies and run migrations
cargo run

# API will be available at http://localhost:8080
# OpenAPI docs at http://localhost:8080/api-docs

# Create an admin user (after first run)
cargo run --bin create_admin
```

#### Frontend Application
```bash
cd frontend

# Install dependencies
yarn install

# Start development server
yarn dev

# App will be available at http://localhost:3000
```

### Production Build

#### Using Docker
```bash
# Build backend
docker build -t hockey-backend ./backend

# Build frontend  
docker build -t hockey-frontend ./frontend
```

#### Manual Build
```bash
# Backend
cd backend
cargo build --release

# Frontend
cd frontend
yarn build
```

## 🔐 Authentication

The application uses a secure OAuth2-inspired JWT authentication system with access and refresh tokens.

### Authentication Flow

1. **Login**: User provides email/password credentials
   - Backend validates credentials against bcrypt-hashed passwords
   - Returns short-lived access token (15 min) and refresh token (7 days)
   - Refresh token is stored hashed in the database

2. **API Requests**: Client includes access token in Authorization header
   - Format: `Authorization: Bearer <access_token>`
   - Backend validates JWT signature using RSA public key
   - Extracts user information from token claims

3. **Token Refresh**: When access token expires
   - Client sends refresh token to `/auth/refresh` endpoint
   - Backend validates refresh token (checks expiration, revocation)
   - Returns new access token and refresh token pair

4. **Logout**: Client sends refresh token to `/auth/logout`
   - Backend revokes the refresh token in database
   - Client discards both tokens

### Security Features

- **RSA-256 JWT Signing**: Uses 4096-bit RSA key pairs for token signing/verification
- **Bcrypt Password Hashing**: Passwords hashed with bcrypt (cost factor 12)
- **Refresh Token Rotation**: New refresh token issued on each refresh
- **Token Revocation**: Refresh tokens can be revoked (logout, security events)
- **Short-lived Access Tokens**: 15-minute expiry reduces exposure window
- **CORS Protection**: Configurable CORS with production mode validation

### Creating an Admin User

For security reasons, no default admin credentials are provided. You must create an admin user using the CLI tool:

```bash
cd backend
cargo run --bin create_admin
```

The tool will interactively prompt you for:
- Email address (must be valid format)
- Password (minimum 8 characters, hidden input)
- Password confirmation
- Name (optional)

The password is securely hashed using bcrypt before being stored in the database.

### JWT Key Generation

Before running the backend, generate RSA key pairs for JWT signing:

```bash
cd backend
./scripts/generate_keys.sh
```

This creates:
- `jwt_private.pem`: Private key for signing tokens (4096-bit RSA)
- `jwt_public.pem`: Public key for verifying tokens

**Important**: These keys are gitignored and should never be committed. Backup them securely for production deployments.

## 📡 API Usage

### Authentication

All API requests (except `/auth/login` and `/health`) require authentication via JWT bearer token:

```bash
# 1. Login to get tokens
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "your-password"
  }'

# Response:
# {
#   "access_token": "eyJ...",
#   "refresh_token": "eyJ...",
#   "token_type": "Bearer",
#   "expires_in": 900
# }

# 2. Use access token for API requests
curl http://localhost:8080/team \
  -H "Authorization: Bearer eyJ..."

# 3. Refresh access token when expired
curl -X POST http://localhost:8080/auth/refresh \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "eyJ..."
  }'

# 4. Logout (revoke refresh token)
curl -X POST http://localhost:8080/auth/logout \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "eyJ..."
  }'
```

### Token Lifetimes

- **Access Token**: 15 minutes (configurable via `JWT_ACCESS_TOKEN_DURATION_MINUTES`)
- **Refresh Token**: 7 days (configurable via `JWT_REFRESH_TOKEN_DURATION_DAYS`)

### API Documentation

Interactive API documentation is available at:
- **Development**: http://localhost:8080/docs
- **OpenAPI Spec**: http://localhost:8080/api-doc/openapi.json

## 🌍 Internationalization

The app supports multiple languages:
- 🇺🇸 English (default)
- 🇨🇿 Czech

Language can be switched using the locale switcher in the navigation.

## 📁 Project Structure

```
src/
├── app/                 # Next.js App Router pages
├── components/          # React components
│   ├── ui/             # Reusable UI components
│   ├── layout/         # Layout components
│   ├── shared/         # Shared app components
│   └── features/       # Feature-specific components
├── hooks/              # Custom React hooks
├── queries/            # TanStack Query definitions
├── types/              # TypeScript type definitions
├── utils/              # Utility functions
├── ui/pages/           # Client page components
└── i18n/               # Internationalization setup
```

## 🎨 Design System

The application uses a consistent design system with:
- **Color Scheme**: Professional blue and gray palette
- **Typography**: Clean, readable fonts
- **Components**: Reusable UI components with consistent styling
- **Icons**: Lucide React icons
- **Responsive**: Mobile-first responsive design

## 📊 Data Management

- **Server State**: Managed with TanStack Query for caching and synchronization
- **Optimistic Updates**: Immediate UI feedback with automatic rollback on errors
- **Error Boundaries**: Comprehensive error handling at multiple levels
- **Loading States**: Skeleton components and suspense boundaries

## 🔧 Environment Variables

### Backend (.env)

All backend configuration can be set via environment variables or `.env` file:

```bash
# Database Configuration
DATABASE_URL=sqlite:./data/hockey_data.db

# JWT Authentication - RSA Keys
JWT_PRIVATE_KEY_PATH=jwt_private.pem
JWT_PUBLIC_KEY_PATH=jwt_public.pem

# JWT Token Durations
JWT_ACCESS_TOKEN_DURATION_MINUTES=15
JWT_REFRESH_TOKEN_DURATION_DAYS=7

# HMAC Key (legacy, kept for compatibility)
HMAC_KEY=your-very-secure-hmac-key-for-jwt-signing-should-be-long-and-random

# Server Configuration
HOST=0.0.0.0
PORT=8080

# Environment Mode (development or production)
# In production mode, wildcard CORS is rejected for security
ENVIRONMENT=development

# CORS Configuration
# Development: wildcard allowed
CORS_ORIGINS=*
CORS_METHODS=GET,POST,PUT,DELETE,OPTIONS
CORS_HEADERS=*

# Production: specify explicit origins
# ENVIRONMENT=production
# CORS_ORIGINS=https://yourdomain.com,https://www.yourdomain.com
# CORS_METHODS=GET,POST,PUT,DELETE,OPTIONS
# CORS_HEADERS=content-type,authorization,x-requested-with

# Logging
RUST_LOG=info
```

### Frontend (.env.local)

```bash
# NextAuth Configuration
NEXTAUTH_SECRET=your-secret-nextauth-key-change-this-in-production-min-32-chars
NEXTAUTH_URL=http://localhost:3000
AUTH_TRUST_HOST=true

# Backend API URL
HOCKEY_BACKEND_URL=http://localhost:8080

# Optional: Custom API endpoint
# NEXT_PUBLIC_API_URL=http://localhost:8080
```

### Docker Compose (.env)

For Docker deployments, configure both services:

```bash
# Backend Configuration
HOCKEY_HMAC_KEY=your-secret-hmac-key-change-this-in-production-min-32-chars
DATABASE_URL=sqlite:///app/data/hockey.db
CORS_ORIGINS=http://localhost:3000,http://localhost:4000
CORS_METHODS=GET,POST,PUT,DELETE,OPTIONS
CORS_HEADERS=*

# Frontend Configuration
NEXTAUTH_SECRET=your-secret-nextauth-key-change-this-in-production-min-32-chars
HOCKEY_BACKEND_URL=http://hockey-backend:8080
```

### Environment Variable Reference

#### Backend Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | - | SQLite database file path |
| `JWT_PRIVATE_KEY_PATH` | Yes | `jwt_private.pem` | Path to RSA private key for JWT signing |
| `JWT_PUBLIC_KEY_PATH` | Yes | `jwt_public.pem` | Path to RSA public key for JWT verification |
| `JWT_ACCESS_TOKEN_DURATION_MINUTES` | No | `15` | Access token expiry in minutes |
| `JWT_REFRESH_TOKEN_DURATION_DAYS` | No | `7` | Refresh token expiry in days |
| `HMAC_KEY` | Yes | - | Legacy HMAC key (kept for compatibility) |
| `HOST` | No | `0.0.0.0` | Server bind address |
| `PORT` | No | `8080` | Server port |
| `ENVIRONMENT` | No | `development` | Environment mode (`development` or `production`) |
| `CORS_ORIGINS` | No | `*` | Allowed CORS origins (comma-separated or `*`) |
| `CORS_METHODS` | No | `GET,POST,PUT,DELETE,OPTIONS` | Allowed HTTP methods |
| `CORS_HEADERS` | No | `*` | Allowed request headers |
| `RUST_LOG` | No | `info` | Logging level (`trace`, `debug`, `info`, `warn`, `error`) |

#### Frontend Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `NEXTAUTH_SECRET` | Yes | - | Secret for NextAuth.js session encryption (min 32 chars) |
| `NEXTAUTH_URL` | Yes | - | Full URL where the app is hosted |
| `AUTH_TRUST_HOST` | No | `false` | Trust host header (useful for proxies) |
| `HOCKEY_BACKEND_URL` | Yes | - | Backend API URL for server-side requests |
| `NEXT_PUBLIC_API_URL` | No | - | Backend API URL for client-side requests (optional) |

## 🚨 Troubleshooting

### Common Build Issues

#### "Type 'MatchDetailsProps' does not satisfy the constraint 'PageProps'"
- **Cause**: Next.js 13+ requires async params in page components
- **Solution**: Use `params: Promise<{ id: string }>` and await the params

#### "failed to read dockerfile: open Dockerfile: no such file or directory"
- **Cause**: Docker build context or file path issues
- **Solution**: Ensure workflow uses correct context and file paths

#### "feature `edition2024` is required"
- **Cause**: Dependencies using experimental Rust edition
- **Solution**: Use Rust nightly or downgrade dependencies to edition2021

#### SQLx macro errors during Docker build
- **Cause**: SQLx macros require database access at compile time
- **Solution**: Replace with regular `sqlx::query_as()` calls with `.bind()`

### Git Submodule Issues
If you encounter submodule errors:
```bash
# Remove broken submodule entry
git rm --cached backend
git add backend/
git commit -m "Fix: Convert submodule to regular directory"
```

## 🏗️ Architecture Patterns

- **Server Components**: For initial data fetching and SEO
- **Client Components**: For interactivity with proper hydration
- **Error Boundaries**: Multi-layer error protection
- **Suspense**: Loading states with skeleton components
- **Form Patterns**: Consistent validation and submission flow

## 📱 Features Overview

### Dashboard
- Quick statistics overview
- Recent activity feed
- Quick action buttons for common tasks

### Teams Management
- Create, edit, and delete teams
- Country association with flag display
- Search and pagination

### Players Management  
- Player profiles with nationality
- Team associations through contracts
- Comprehensive player database

### Events Management
- Tournament and competition management
- Host country tracking
- Season organization

### Seasons Management
- Event-specific seasons
- Year-based organization
- Display name customization

## 📄 License

This project is licensed under the MIT License.