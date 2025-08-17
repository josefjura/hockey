# Hockey Management System - Monorepo

A modern full-stack web application for managing hockey tournaments, teams, players, and seasons. Built with a Rust backend API and Next.js frontend.

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
- **Authentication System** with session management
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
- **Authentication**: bcrypt for password hashing
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

# Install dependencies and run migrations
cargo run

# API will be available at http://localhost:8080
# OpenAPI docs at http://localhost:8080/api-docs
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

Default login credentials:
- Email: `admin@hockey.local`
- Password: `admin123`

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

### Frontend (.env.local)
```bash
NEXTAUTH_SECRET=your-nextauth-secret
NEXTAUTH_URL=http://localhost:3000
BACKEND_URL=http://localhost:8080
```

### Backend (.env)
```bash
DATABASE_URL=sqlite:./database.db
RUST_LOG=info
```

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