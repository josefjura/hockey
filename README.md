# Hockey Management System

A modern hockey tournament management application built with Rust, HTMX, and Maud. Single-binary deployment with server-rendered HTML and progressive enhancement.

## 🏗️ Project Structure

```
hockey/
├── src/               # Rust source code (Axum + HTMX + Maud)
│   ├── main.rs        # Application entry point
│   ├── routes/        # HTTP route handlers
│   ├── views/         # Maud HTML templates
│   ├── business/      # Business logic layer
│   ├── service/       # Data access layer
│   ├── auth/          # Authentication & sessions
│   ├── i18n/          # Internationalization (Czech/English)
│   └── bin/           # Utility binaries (create_admin)
├── migrations/        # SQLx database migrations
├── static/            # Static assets (CSS, JS, images)
│   └── js/components/ # Built Lit web components
├── web_components/    # Lit TypeScript source (built to static/js/)
├── backup/            # Archived previous implementations (gitignored)
└── data/              # SQLite database (gitignored)
```

## 🚀 Features

- **Complete Hockey Management** - Teams, Players, Events, Seasons, Matches
- **Score Tracking** - Individual goals with scorer/assist tracking
- **Multi-language Support** - English & Czech via fluent-rs
- **Responsive Design** - Mobile-first with Tailwind CSS
- **Hybrid Table Strategy**:
  - Small datasets: Client-side Lit Web Components with sorting/filtering
  - Large datasets: Server-side HTMX pagination
- **Session-based Authentication** - Cookie-based auth with bcrypt
- **Progressive Enhancement** - Works without JavaScript, enhanced with it
- **Single Binary Deployment** - All assets embedded in binary with gzip compression
- **Optimized Assets** - Minified JavaScript, embedded CSS/JS/images, automatic compression

## 🛠️ Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Async Rust web framework
- **Templating**: [Maud](https://maud.lambda.xyz/) - Type-safe HTML templates in Rust
- **Interactivity**: [HTMX](https://htmx.org/) - Hypermedia-driven updates
- **Components**: [Lit](https://lit.dev/) - Lightweight web components
- **Database**: SQLite with [SQLx](https://github.com/launchbadge/sqlx) compile-time checked queries
- **Authentication**: Session-based with bcrypt password hashing
- **Internationalization**: [fluent-rs](https://docs.rs/fluent/) for Czech/English
- **Styling**: [Tailwind CSS](https://tailwindcss.com/)

## 📋 Prerequisites

- **Rust 1.81+** (or latest stable)
- **SQLite 3** (for local development)
- **Node.js 20+** (only for web components development)
- **Docker & Docker Compose** (for containerized deployment)

## 🚀 Getting Started

### Local Development

1. **Clone the repository:**
   ```bash
   git clone https://github.com/josefjura/hockey.git
   cd hockey
   ```

2. **Copy environment variables:**
   ```bash
   cp .env.example .env
   ```

3. **Generate session secret:**
   ```bash
   openssl rand -hex 32
   ```
   Add the output to `.env` as `SESSION_SECRET`

4. **Run the application:**
   ```bash
   cargo run
   ```
   The database will be created automatically from migrations on first run.

5. **Visit the application:**
   Open http://localhost:8080 in your browser

### Create Admin User

```bash
cargo run --bin create_admin
```

The tool will prompt for:
- Email address
- Password (min 8 characters)
- Name (optional)

### Using Make Commands

```bash
# Show all available commands
make help

# Run pre-commit checks (format, clippy, tests)
make precommit

# Start development server
make dev

# Run tests
make test

# Build for production
make build

# Create admin user
make create-admin

# Docker commands
make docker-build    # Build Docker image
make docker-up       # Start containers
make docker-down     # Stop containers
make docker-logs     # View logs
```

### Web Components Development

If you need to modify Lit web components:

```bash
cd web_components

# Install dependencies (first time only)
yarn install

# Build components (outputs to ../static/js/components/)
yarn build

# Build with minification for production
yarn build:prod

# Watch mode for development
yarn watch

# Run Storybook for component development
yarn storybook
```

**Note:** Built JS files are committed to `static/js/components/`. After building, commit both TypeScript source and built output. For production builds, use `yarn build:prod` to minify the JavaScript.

## 🐳 Docker Deployment

### Development

```bash
docker compose up -d
```

Access at http://localhost:8080

### Production

1. **Set up environment variables:**
   ```bash
   cp .env.prod.example .env.prod
   # Edit .env.prod with production values
   ```

2. **Deploy:**
   ```bash
   docker compose -f docker-compose.prod.yaml up -d
   ```

## 🔧 Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:./hockey.db` | SQLite database file path |
| `SESSION_SECRET` | ⚠️ development key | Session encryption secret (required for production) |
| `ENVIRONMENT` | `development` | Environment mode (`development` or `production`) |
| `PORT` | `8080` | Server port |
| `RUST_LOG` | `info` | Logging level |

## 📦 CI/CD

The project uses **cargo-dist style** GitHub Actions workflow:

- **On Push to Master**: Runs tests only (format, clippy, cargo test)
- **On Tag (v*.*.*)**: 
  - Runs full test suite
  - Builds Docker image
  - Pushes to GitHub Container Registry (`ghcr.io/josefjura/hockey:latest`)
  - Auto-creates GitHub Release with notes

### Release Workflow

1. **Create and push a tag:**
   ```bash
   git tag v0.1.0
   git push --tags
   ```

2. **GitHub Actions automatically:**
   - Runs tests
   - Builds multi-platform Docker image
   - Pushes to `ghcr.io/josefjura/hockey:latest`
   - Creates GitHub Release with deployment instructions

3. **Deploy to server:**
   ```bash
   # On your production server
   cd ~/hockey
   docker compose -f docker-compose.prod.yaml pull
   docker compose -f docker-compose.prod.yaml up -d
   ```

4. **Or use Watchtower** for automatic updates (see DEPLOYMENT.md)

For detailed deployment instructions, see [DEPLOYMENT.md](DEPLOYMENT.md)

## 📚 Development Commands

```bash
# Start development server
cargo run

# Build project
cargo build --release

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
```

## 🗂️ Project History

This project was rewritten in 2025 from a dual-service architecture (Next.js frontend + Rust API backend) to a unified Rust application with HTMX. The new architecture provides:

- **Simpler Deployment** - Single Docker container instead of two services
- **Better Performance** - No API overhead, direct database queries
- **Type Safety** - End-to-end Rust with compile-time HTML templates
- **Progressive Enhancement** - Server-rendered HTML with HTMX

The old implementations are preserved locally in the `backup/` directory (gitignored).

## 📄 License

This project is licensed under the MIT License.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
