# Hockey Management System - Frontend

A modern web application for managing hockey tournaments, teams, players, and seasons built with Next.js 15.

## 🚀 Features

- **Complete Hockey Management** - Teams, Players, Events, and Seasons
- **Real-time Data Management** with optimistic updates
- **Multi-language Support** (English & Czech)
- **Modern UI/UX** with responsive design
- **Form Validation** with real-time feedback
- **Search & Pagination** for all data tables
- **Authentication System** with session management

## 🛠️ Tech Stack

- **Framework**: [Next.js 15](https://nextjs.org/) with App Router
- **Language**: TypeScript
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/)
- **Authentication**: [NextAuth.js](https://next-auth.js.org/) 
- **State Management**: [TanStack Query](https://tanstack.com/query) for server state
- **Forms**: [React Hook Form](https://react-hook-form.com/) + [Zod](https://zod.dev/) validation
- **Internationalization**: [next-intl](https://next-intl-docs.vercel.app/)
- **UI Components**: [Headless UI](https://headlessui.com/)

## 📋 Prerequisites

- Node.js 18+
- Yarn (preferred) or npm

## 🚀 Getting Started

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd hockey/frontend

# Install dependencies
yarn install

# Start development server
yarn dev
```

The application will be available at `http://localhost:3000`

### Development Commands

```bash
# Development server with Turbopack
yarn dev

# Build for production  
yarn build

# Start production server
yarn start

# Run linting
yarn lint
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

```bash
NEXTAUTH_SECRET=your-nextauth-secret
NEXTAUTH_URL=http://localhost:3000
BACKEND_URL=http://localhost:8080
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