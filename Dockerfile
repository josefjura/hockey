# Multi-stage Dockerfile for Hockey Management System
# Builds a production-ready single binary with embedded assets

# Stage 1: Build the Rust application
FROM rust:1.81-slim-bookworm AS builder

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
	rm -rf src

# Copy source code and migrations
COPY src ./src
COPY migrations ./migrations
COPY static ./static

# Build the actual application
RUN cargo build --release

# Stage 2: Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
	apt-get install -y \
	ca-certificates \
	libssl3 \
	sqlite3 \
	&& rm -rf /var/lib/apt/lists/*

# Create app user for security
RUN useradd -m -u 1000 hockey

# Create app directory and data directory
WORKDIR /app
RUN mkdir -p /app/data && chown -R hockey:hockey /app

# Copy binary from builder
COPY --from=builder /app/target/release/hockey /app/hockey
COPY --from=builder /app/target/release/create_admin /app/create_admin

# Copy migrations (needed for runtime schema checks)
COPY --chown=hockey:hockey migrations ./migrations

# Switch to non-root user
USER hockey

# Expose port
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:///app/data/hockey.db
ENV ENVIRONMENT=production

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
	CMD ["sh", "-c", "wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1"]

# Run the application
CMD ["/app/hockey"]
