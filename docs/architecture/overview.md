# Architecture Overview

## Project Structure

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

## Features

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