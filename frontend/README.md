# Hockey Management Application - Frontend

Single-binary Rust application using Axum, Maud, HTMX, and Lit Web Components.

## Project Structure

```
frontend/
├── src/
│   ├── main.rs              # Entry point
│   ├── config.rs            # Configuration management
│   ├── routes/              # HTTP route handlers
│   ├── views/               # Maud templates
│   │   ├── components/      # Reusable components
│   │   └── pages/           # Page templates
│   ├── business/            # Business logic layer
│   ├── service/             # Data access layer
│   ├── auth/                # Authentication & sessions
│   └── i18n/                # Internationalization
│       └── messages/        # Translation files
├── web_components/          # Lit Web Components (TypeScript)
├── static/                  # Static assets (CSS, JS, flags)
├── migrations/              # SQLx database migrations
└── Cargo.toml
```

## Prerequisites

- Rust 1.75+ (with cargo)
- SQLite 3
- OpenSSL (for generating secrets)

## Setup

1. **Copy environment variables:**
   ```bash
   cp .env.example .env
   ```

2. **Generate session secret:**
   ```bash
   openssl rand -hex 32
   ```
   Add the output to `.env` as `SESSION_SECRET`

3. **Build the project:**
   ```bash
   cargo build
   ```

4. **Run the application:**
   ```bash
   cargo run
   ```
   The database will be created automatically from migrations on first run.

5. **Visit the application:**
   Open http://localhost:8080 in your browser

## Development Commands

```bash
# Start development server
cargo run

# Build project
cargo build

# Run tests
cargo test

# Check code (faster than build)
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Hot reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Create admin user (after auth is implemented)
cargo run --bin create_admin
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:./hockey.db` | SQLite database file path |
| `SESSION_SECRET` | ⚠️ development key | Session encryption secret (required for production) |
| `ENVIRONMENT` | `development` | Environment mode (`development` or `production`) |
| `PORT` | `8080` | Server port |

## Tech Stack

- **Backend**: Axum (web framework)
- **Templating**: Maud (type-safe HTML)
- **Interactivity**: HTMX (dynamic updates)
- **Components**: Lit (Web Components)
- **Database**: SQLite + SQLx
- **Auth**: Session-based with cookies
- **i18n**: fluent-rs (Czech/English)
- **Styling**: Tailwind CSS

## Project Status

This is an active rewrite. See [REWRITE_ANALYSIS.md](../REWRITE_ANALYSIS.md) and [GITHUB_PROJECT_SETUP.md](../GITHUB_PROJECT_SETUP.md) for details.

**Current Phase**: Phase 1 - Foundation
**Milestone**: https://github.com/josefjura/hockey/milestone/1

## License

MIT
