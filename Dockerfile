# Multi-stage Dockerfile for Hockey Management System
# Builds a production-ready single binary with embedded assets

# Stage 1: Build and minify web components
FROM node:20-slim AS node-builder

WORKDIR /app

# Copy web components files
COPY web_components/package.json web_components/yarn.lock ./web_components/
WORKDIR /app/web_components
RUN yarn install --frozen-lockfile

# Copy source and build with minification
COPY web_components/ ./
RUN yarn build:prod

# Stage 2: Build the Rust application
# Rust 1.85+ required for edition2024 support
FROM rust:1.85-slim-bookworm AS rust-builder

# Install build dependencies
RUN apt-get update && \
	apt-get install -y \
	pkg-config \
	libssl-dev \
	&& rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src/bin && \
	echo "fn main() {}" > src/main.rs && \
	echo "fn main() {}" > src/bin/create_admin.rs && \
	cargo build --release && \
	rm -rf src target/release/hockey target/release/create_admin target/release/deps/hockey-* target/release/deps/create_admin-*

# Copy source code and migrations
COPY src ./src
COPY migrations ./migrations
COPY static ./static

# Copy minified web components from node-builder
COPY --from=node-builder /app/static/js/components ./static/js/components

# Build the actual application (with embedded assets)
RUN cargo build --release

# Stage 3: Runtime image
FROM gcr.io/distroless/cc-debian12

# Copy binary from rust-builder (assets are embedded in the binary)
COPY --from=rust-builder /app/target/release/hockey /app/hockey
COPY --from=rust-builder /app/target/release/create_admin /app/create_admin

# Copy migrations (needed for runtime schema checks)
COPY migrations /app/migrations

# Note: Static assets are now embedded in the binary
# Only uploads directory needs to be available at runtime

# Create data directory (distroless runs as nonroot by default)
WORKDIR /app

# Note: /app/data directory will be created by Docker when volume is mounted
# Expose port
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:///app/data/hockey.db?mode=rwc
ENV ENVIRONMENT=production

# Note: Distroless doesn't support healthcheck commands with shell
# Health checks should be configured at the orchestration level (Docker Compose, Kubernetes)

# Run the application
CMD ["/app/hockey"]
