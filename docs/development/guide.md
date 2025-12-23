# Development Guide

## Using Make Commands

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

## Development Commands

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

## Web Components Development

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