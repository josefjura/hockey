# Installation & Setup

## Local Development

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

## Create Admin User

```bash
cargo run --bin create_admin
```

The tool will prompt for:
- Email address
- Password (min 8 characters)
- Name (optional)