# Hockey Management System

A modern hockey tournament management application built with Rust, HTMX, and Maud. Single-binary deployment with embedded assets, server-rendered HTML, and progressive enhancement.

## 🚧 Active Rewrite

This project is currently being rewritten from Next.js/React to HTMX/Maud for simplified deployment and maintenance.

- **Track progress**: [View GitHub Milestones](https://github.com/josefjura/hockey/milestones)
- **Implementation plan**: See `REWRITE_ANALYSIS.md` for architecture details
- **Getting started**: See `GITHUB_PROJECT_SETUP.md` for task breakdown

## 🏗️ Project Structure

```
hockey/
├── frontend/           # NEW: Rust + HTMX + Maud application
│   ├── migrations/     # SQLx database migrations
│   └── src/           # Rust source code
├── backend/           # Reference: Old Rust API (for SQL queries)
├── backup/            # Reference: Old Next.js frontend (gitignored)
└── docs/              # Analysis and project documentation
```

## 🚀 Features (Planned)

- **Complete Hockey Management** - Teams, Players, Events, Seasons, Matches
- **Score Tracking** - Individual goals with scorer/assist tracking
- **Multi-language Support** (English & Czech) via fluent-rs
- **Responsive Design** - Mobile-first with Tailwind CSS
- **Two Table Strategies**:
  - Small datasets (≤500 rows): Client-side Lit Web Components
  - Large datasets: Server-side HTMX datagrids
- **Session-based Authentication** - Simplified auth with cookies
- **Progressive Enhancement** - Works without JavaScript, enhanced with JS

## 🛠️ Tech Stack

### Backend & Frontend (Single Binary)
- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Async Rust web framework
- **Templating**: [Maud](https://maud.lambda.xyz/) - Type-safe HTML templates
- **Interactivity**: [HTMX](https://htmx.org/) - Server-driven dynamic HTML
- **Components**: [Lit](https://lit.dev/) - Web Components for rich client-side features
- **Database**: SQLite with [SQLx](https://github.com/launchbadge/sqlx)
- **Authentication**: Session-based with bcrypt password hashing
- **Internationalization**: [fluent-rs](https://docs.rs/fluent/) for Czech/English
- **Styling**: [Tailwind CSS](https://tailwindcss.com/)

### Architecture
- **Three-layer**: Routes → Business Logic → Service (preserved from old backend)
- **Server-first**: HTML rendered on server, progressive enhancement with JS
- **Embedded Assets**: CSS, JS, flags, images compiled into binary
- **Type-safe queries**: SQLx compile-time verification

## 📋 Prerequisites

- Rust 1.81+ (or latest stable)
- SQLite (for development)
- Node.js 18+ (for Lit component compilation only)

## 🚀 Getting Started

### Development Setup

```bash
cd frontend

# First run: Database will be created from migrations
cargo run

# Application available at http://localhost:8080
```

### Create Admin User

```bash
cd frontend
cargo run --bin create_admin
```

The tool will prompt for:
- Email address
- Password (min 8 characters)
- Name (optional)

### Environment Variables

Create `.env` in `frontend/`:

```bash
DATABASE_URL=sqlite:./hockey.db
ENVIRONMENT=development
SESSION_SECRET=your-secret-key-change-in-production
```

**Production**: Use secure random keys generated with `openssl rand -hex 32`

## 📡 API Structure

The application uses HTMX for dynamic updates, returning HTML fragments instead of JSON:

### Pages
- `/` - Dashboard with stats and quick actions
- `/teams` - Teams management with search/filters
- `/players` - Players management
- `/events` - Events/tournaments management
- `/seasons` - Seasons management
- `/matches` - Matches list with complex filtering
- `/matches/{id}` - Match detail with score tracking
- `/management` - Countries and system settings

### Authentication
- `POST /auth/login` - Login form submission
- `POST /auth/logout` - Logout and destroy session

### HTMX Endpoints
Most endpoints accept both full page loads and HTMX requests:
- Full page: Returns complete HTML with layout
- HTMX: Returns HTML fragment for target element

Example:
```html
<!-- HTMX request for pagination -->
<div hx-get="/teams/list?page=2" hx-target="#teams-table">
  Load more
</div>
```

## 🌍 Internationalization

The app supports multiple languages using fluent-rs:
- 🇺🇸 English (default)
- 🇨🇿 Czech

Language switching via locale parameter in URL or user preference cookie.

## 📊 Data Management

### Database Schema
- **Authentication**: users, sessions (cookie-based)
- **Core entities**: country, event, season, team, player
- **Relationships**: team_participation, player_contract
- **Matches**: match, score_event (goals with scorer/assists)

### Historical Data
Supports historical countries (Soviet Union, East Germany, etc.) with year ranges.

## 🎨 Design System

- **Server-rendered**: Maud templates for type-safe HTML
- **Styling**: Tailwind CSS utility classes
- **Components**:
  - Reusable Maud templates for tables, forms, modals
  - Lit Web Components for rich interactions (small tables, selectors)
- **Icons**: Inline SVG or icon font
- **Responsive**: Mobile-first design with breakpoints

## 🔐 Authentication

Session-based authentication with cookies:

1. **Login**: User submits email/password
   - Backend validates against bcrypt hash
   - Creates session, sets HttpOnly cookie

2. **Requests**: Session cookie sent automatically
   - Middleware validates session
   - Extracts user from session store

3. **Logout**: Destroy session and clear cookie

**Security Features**:
- Bcrypt password hashing
- HttpOnly, Secure, SameSite cookies
- CSRF protection
- Session expiration

## 📁 Project Documentation

- **REWRITE_ANALYSIS.md** - Complete architecture analysis and implementation plan
- **GITHUB_PROJECT_SETUP.md** - Milestones, issues, and task breakdown
- **CLAUDE.md** - AI assistant instructions and project patterns
- **SECURITY.md** - Security guidelines and best practices

## 🏗️ Implementation Status

**Current Phase**: Phase 1 - Foundation

Track progress at: https://github.com/josefjura/hockey/milestones

### Timeline (9 weeks total)
- ✅ Week 0: Analysis and planning
- 🔄 Weeks 1-2: Phase 1 - Foundation (auth, layout, Teams CRUD)
- ⏳ Weeks 3-4: Phase 2 - Core entities
- ⏳ Week 5: Phase 3 - Table components
- ⏳ Weeks 6-7: Phase 4 - Matches & scoring
- ⏳ Week 8: Phase 5 - Dashboard & polish
- ⏳ Week 9: Phase 6 - Deployment

## 🚢 Deployment

### Single Binary

Build for production:

```bash
cd frontend
cargo build --release

# Binary with embedded assets:
./target/release/hockey
```

### Systemd Service

Example service file:

```ini
[Unit]
Description=Hockey Management System
After=network.target

[Service]
Type=simple
User=hockey
WorkingDirectory=/opt/hockey
ExecStart=/opt/hockey/hockey
Restart=on-failure
Environment="DATABASE_URL=sqlite:/opt/hockey/data/hockey.db"
Environment="ENVIRONMENT=production"
Environment="SESSION_SECRET=your-production-secret"

[Install]
WantedBy=multi-user.target
```

### Reverse Proxy

Use Nginx or Caddy for SSL termination:

```nginx
server {
    listen 443 ssl http2;
    server_name hockey.example.com;

    ssl_certificate /etc/letsencrypt/live/hockey.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/hockey.example.com/privkey.pem;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 🔧 Development

### Project Commands

```bash
# Run application
cargo run

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Create admin user
cargo run --bin create_admin
```

### Hot Reload

Use `cargo-watch` for automatic recompilation:

```bash
cargo install cargo-watch
cargo watch -x run
```

## 📚 Learning Resources

- **Axum**: https://docs.rs/axum/
- **Maud**: https://maud.lambda.xyz/
- **HTMX**: https://htmx.org/docs/
- **Lit**: https://lit.dev/docs/
- **SQLx**: https://docs.rs/sqlx/
- **fluent-rs**: https://docs.rs/fluent/

## 🤝 Contributing

See `GITHUB_PROJECT_SETUP.md` for current issues and milestones.

## 📄 License

MIT License - See LICENSE file for details

---

**Old Architecture Reference**: See `backup/` directory for previous Next.js/React implementation
