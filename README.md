# Hockey Management System

A modern hockey tournament management application built with Rust, HTMX, and Maud. Single-binary deployment with server-rendered HTML and progressive enhancement.

## 📖 Documentation

The full documentation is available in the `docs/` directory:

- **[Getting Started](docs/getting-started/installation.md)**
  - [Installation & Setup](docs/getting-started/installation.md)
  - [Development Environment](docs/getting-started/dev-env.md)
- **[Architecture](docs/architecture/overview.md)**
  - [Overview](docs/architecture/overview.md)
  - [Tech Stack](docs/architecture/tech-stack.md)
- **[Development](docs/development/guide.md)**
  - [Development Guide](docs/development/guide.md)
  - [Web Components](docs/development/guide.md#web-components-development)
- **[Deployment](docs/deployment/guide.md)**
  - [Docker](docs/deployment/docker.md)
  - [Production Guide](docs/deployment/guide.md)

## 🚀 Quick Start

1. **Clone & Setup:**
   ```bash
   git clone https://github.com/josefjura/hockey.git
   cd hockey
   cp .env.example .env
   # Set SESSION_SECRET in .env
   ```

2. **Run:**
   ```bash
   cargo run --bin hockey
   ```

3. **Visit:** http://localhost:8080

For detailed instructions, see the [Installation Guide](docs/getting-started/installation.md).

## 🗂️ Project History

This project was rewritten in 2025 from a dual-service architecture (Next.js frontend + Rust API backend) to a unified Rust application with HTMX. The new architecture provides:

- **Simpler Deployment** - Single Docker container instead of two services
- **Better Performance** - No API overhead, direct database queries
- **Type Safety** - End-to-end Rust with compile-time HTML templates
- **Progressive Enhancement** - Server-rendered HTML with HTMX

## 📄 License

This project is licensed under the MIT License.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
