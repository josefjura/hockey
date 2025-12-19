# Multi-stage Dockerfile for Hockey Management System
# Builds a production-ready single binary with embedded assets

# Stage 1: Build the Rust application
# Rust 1.85+ required for edition2024 support
FROM rust:1.85-slim-bookworm AS builder

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

# Build the actual application
RUN cargo build --release

# Stage 2: Runtime image
FROM gcr.io/distroless/cc-debian12

# Copy binary from builder
COPY --from=builder /app/target/release/hockey /app/hockey
COPY --from=builder /app/target/release/create_admin /app/create_admin

# Copy migrations (needed for runtime schema checks)
COPY migrations /app/migrations

# Copy static files (CSS, JS, images)
COPY static /app/static

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
