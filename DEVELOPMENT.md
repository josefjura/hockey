# Development Guide

This guide covers the development workflow, tooling, and pre-commit checks for the Jura Hockey project.

## Quick Start

```bash
# Install dependencies
make install

# Run pre-commit checks before committing
make precommit           # Check both frontend and backend
make precommit-web       # Check only frontend
make precommit-server    # Check only backend
```

## Project Structure

```
hockey/
├── backend/           # Rust backend (Axum + SQLx)
├── frontend/          # Next.js frontend
├── Makefile          # Build automation and pre-commit commands
└── DEVELOPMENT.md    # This file
```

## Development Commands

### Quick Reference

| Command | Description |
|---------|-------------|
| `make help` | Show all available commands |
| `make precommit` | Run all pre-commit checks |
| `make precommit-web` | Run frontend pre-commit checks |
| `make precommit-server` | Run backend pre-commit checks |
| `make dev-web` | Start frontend dev server |
| `make dev-server` | Start backend dev server |

### Frontend Commands

#### Development
```bash
make dev-web              # Start Next.js dev server with Turbopack
cd frontend && yarn dev   # Alternative
```

#### Pre-commit Checks
```bash
make precommit-web        # Run all checks (lint, format, typecheck)
make lint-web             # Run ESLint
make format-check-web     # Check Prettier formatting
make typecheck-web        # Run TypeScript type checking
```

#### Formatting
```bash
make format-web           # Format code with Prettier
cd frontend && yarn format              # Alternative
cd frontend && yarn format:check        # Check without modifying
```

#### Building
```bash
make build-web            # Production build
cd frontend && yarn build # Alternative
```

### Backend Commands

#### Development
```bash
make dev-server           # Start Rust dev server
cd backend && cargo run   # Alternative
```

#### Pre-commit Checks
```bash
make precommit-server     # Run all checks (format, clippy, test)
make format-check-server  # Check rustfmt formatting
make clippy-server        # Run Clippy linter
make test-server          # Run tests
```

#### Formatting
```bash
make format-server        # Format code with rustfmt
cd backend && cargo fmt   # Alternative
```

#### Building
```bash
make build-server         # Production build (release mode)
cd backend && cargo build --release  # Alternative
```

### Combined Commands

```bash
make lint                 # Lint both frontend and backend
make format               # Format both frontend and backend
make format-check         # Check formatting on both
make test                 # Run all tests
make build                # Build both projects
make clean                # Clean all build artifacts
```

## Pre-commit Workflow

### Recommended Workflow

Before committing code, always run:

```bash
make precommit
```

This runs:
- **Frontend**: ESLint → Prettier check → TypeScript type check
- **Backend**: rustfmt check → Clippy → Tests

### What Each Check Does

#### Frontend Checks

1. **ESLint** (`make lint-web`)
   - Checks code quality and common mistakes
   - Enforces Next.js best practices
   - Validates TanStack Query usage
   - Config: `frontend/eslint.config.mjs`

2. **Prettier** (`make format-check-web`)
   - Enforces consistent code formatting
   - Checks without modifying files
   - Config: `frontend/.prettierrc.json`

3. **TypeScript** (`make typecheck-web`)
   - Validates all TypeScript types
   - Catches type errors before build
   - Uses `tsc --noEmit`

#### Backend Checks

1. **rustfmt** (`make format-check-server`)
   - Enforces Rust code formatting standards
   - Checks without modifying files
   - Config: `backend/rustfmt.toml`

2. **Clippy** (`make clippy-server`)
   - Catches common mistakes and anti-patterns
   - Enforces Rust best practices
   - Runs with `-D warnings` (treats warnings as errors)
   - Config: `backend/clippy.toml`

3. **Tests** (`make test-server`)
   - Runs all unit and integration tests
   - Ensures changes don't break existing functionality

### Fixing Issues

If pre-commit checks fail:

**Frontend:**
```bash
# Fix linting issues
cd frontend && yarn lint --fix

# Format code
make format-web

# Fix type errors manually
```

**Backend:**
```bash
# Format code
make format-server

# Fix Clippy warnings manually
# (Most can be auto-fixed with cargo-fix)
cd backend && cargo fix --allow-dirty --allow-staged
```

## Configuration Files

### Frontend Configuration

| File | Purpose |
|------|---------|
| `frontend/package.json` | npm scripts and dependencies |
| `frontend/eslint.config.mjs` | ESLint rules and plugins |
| `frontend/.prettierrc.json` | Prettier formatting rules |
| `frontend/.prettierignore` | Files to ignore in formatting |
| `frontend/tsconfig.json` | TypeScript compiler options |

### Backend Configuration

| File | Purpose |
|------|---------|
| `backend/Cargo.toml` | Dependencies and project metadata |
| `backend/rustfmt.toml` | Rustfmt formatting rules |
| `backend/clippy.toml` | Clippy linting configuration |

## Continuous Integration

The CI pipeline runs the same checks as `make precommit`:

```yaml
# Simplified CI flow
Frontend:
  - Install dependencies (yarn install)
  - Run ESLint (yarn lint)
  - Check formatting (yarn format:check)
  - Type check (yarn typecheck)
  - Build (yarn build)

Backend:
  - Format check (cargo fmt --check)
  - Clippy (cargo clippy -- -D warnings)
  - Tests (cargo test)
  - Build (cargo build --release)
```

**Important:** If `make precommit` passes locally, CI should pass too!

## Code Style Guidelines

### Frontend (TypeScript/React)

- **Indentation**: Tabs (2 spaces width)
- **Line Length**: 100 characters
- **Quotes**: Double quotes
- **Semicolons**: Required
- **Trailing Commas**: ES5 style
- **Import Organization**: Auto-sorted by ESLint

### Backend (Rust)

- **Indentation**: Tabs (4 spaces width)
- **Line Length**: 100 characters
- **Import Organization**: Grouped (std → external → crate)
- **Trailing Commas**: Vertical style
- **Comments**: Wrapped at 100 characters

## Troubleshooting

### Frontend Issues

**`yarn lint` fails:**
- Run `yarn lint --fix` to auto-fix issues
- Check `frontend/eslint.config.mjs` for rule overrides

**`yarn format:check` fails:**
- Run `make format-web` to auto-format
- Check `.prettierrc.json` for configuration

**`yarn typecheck` fails:**
- Fix TypeScript errors manually
- Check `tsconfig.json` for compiler options

### Backend Issues

**`cargo fmt --check` fails:**
- Run `make format-server` to auto-format
- Check `rustfmt.toml` for configuration

**`cargo clippy` fails:**
- Fix warnings manually or use `cargo fix`
- Some warnings may require code changes

**`cargo test` fails:**
- Check test output for failing tests
- Ensure database migrations are up to date

## Editor Integration

### VS Code

Recommended extensions:
- **Frontend**: ESLint, Prettier, TypeScript
- **Backend**: rust-analyzer, Even Better TOML

Settings (`.vscode/settings.json`):
```json
{
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

### Other Editors

- Configure your editor to run formatters on save
- Set up linter integration for real-time feedback
- Use the Makefile commands for manual checks

## Best Practices

1. **Run `make precommit` before every commit**
2. **Fix linting/formatting issues immediately**
3. **Don't commit code that doesn't pass checks**
4. **Keep commits focused and atomic**
5. **Write descriptive commit messages**
6. **Run tests after significant changes**

## Getting Help

If you encounter issues:

1. Check this guide for configuration details
2. Review the error messages carefully
3. Run individual checks to isolate the problem
4. Check CI logs for additional context
5. Ask for help in the team chat

---

**Remember:** Clean code is happy code! 🎯
