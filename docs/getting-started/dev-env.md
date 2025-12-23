# Development Environment

## Prerequisites

- **Rust 1.81+** (or latest stable)
- **SQLite 3** (for local development)
- **Node.js 20+** (only for web components development)
- **Docker & Docker Compose** (for containerized deployment)

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:./hockey.db` | SQLite database file path |
| `SESSION_SECRET` | ⚠️ development key | Session encryption secret (required for production) |
| `ENVIRONMENT` | `development` | Environment mode (`development` or `production`) |
| `PORT` | `8080` | Server port |
| `RUST_LOG` | `info` | Logging level |