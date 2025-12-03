# Docker Deployment Guide

This guide explains how to deploy the hockey management application using Docker Compose.

## Quick Start

1. **Copy environment variables**:
   ```bash
   cp .env.example .env
   ```

2. **Edit environment variables**:
   ```bash
   # Required: Change these secret keys
   HOCKEY_HMAC_KEY=your-secret-hmac-key-32-chars-minimum
   NEXTAUTH_SECRET=your-secret-nextauth-key-32-chars-minimum
   
   # For production: Update the URL
   NEXTAUTH_URL=https://your-domain.com
   HOCKEY_DOMAIN=your-domain.com
   ```

3. **Start the application**:
   ```bash
   docker-compose up -d
   ```

4. **Access the application**:
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080
   - API Documentation: http://localhost:8080/docs

## Production Deployment

### Prerequisites
- Traefik reverse proxy running with external network `traefik-net`
- DNS records pointing to your server:
  - `your-domain.com` → your server IP
  - `api.your-domain.com` → your server IP

### Production Environment Setup

1. **Set production environment variables**:
   ```bash
   # Copy the example file
   cp .env.prod.example .env.prod

   # Generate secure secrets
   export HOCKEY_HMAC_KEY=$(openssl rand -hex 32)
   export NEXTAUTH_SECRET=$(openssl rand -base64 32)

   # Edit .env.prod and set:
   # - HOCKEY_HMAC_KEY (use generated value above)
   # - NEXTAUTH_SECRET (use generated value above)
   # - NEXTAUTH_URL (your production domain)
   # - NEXT_PUBLIC_API_URL (your API domain)
   # - CORS_ORIGINS (your frontend domain)
   ```

2. **Deploy with production configuration**:
   ```bash
   docker-compose -f docker-compose.prod.yaml up -d
   ```

   **IMPORTANT**: The production compose file now requires a `.env.prod` file with all secrets.
   Never commit `.env.prod` to version control!

### Production Features
- **Traefik Integration**: Automatic HTTPS with Let's Encrypt
- **Container Images**: Uses pre-built images from GitHub Container Registry
- **Auto-updates**: Watchtower integration for automatic updates
- **WWW Redirect**: Redirects www subdomain to apex domain
- **Internal Networking**: Services communicate over internal Docker network

## Database Setup

The SQLite database is automatically:
- Created in a Docker volume (`hockey-data`)
- Initialized with migrations on first startup
- Persisted between container restarts

### Database Location
- Inside container: `/app/data/hockey.db`
- Docker volume: `hockey-data`

### Backup Database
```bash
# Create backup
docker-compose exec hockey-backend cp /app/data/hockey.db /app/data/hockey-backup-$(date +%Y%m%d).db

# Or copy to host
docker cp hockey-backend:/app/data/hockey.db ./hockey-backup.db
```

## Environment Variables

### Required Variables (Production)
All production secrets must be set in `.env.prod` file:

- `HOCKEY_HMAC_KEY`: JWT signing key for backend authentication (generate with: `openssl rand -hex 32`)
- `NEXTAUTH_SECRET`: NextAuth.js secret for session encryption (generate with: `openssl rand -base64 32`)
- `NEXTAUTH_URL`: Frontend public URL (e.g., https://hockey.example.com)
- `NEXT_PUBLIC_API_URL`: Backend API public URL (e.g., https://api.hockey.example.com)
- `CORS_ORIGINS`: Comma-separated allowed CORS origins (e.g., https://hockey.example.com)

### Optional Variables
- `NEXTAUTH_URL_INTERNAL`: Internal NextAuth URL (default: http://hockey_web:3000)
- `HOCKEY_BACKEND_URL`: Internal backend URL (default: http://hockey-backend:8080)
- `CORS_METHODS`: Allowed HTTP methods (default: GET,POST,PUT,DELETE,OPTIONS)
- `CORS_HEADERS`: Allowed headers (default: Content-Type,Authorization)
- `RUST_LOG`: Logging level (default: info,jura_hockey=debug)
- `NEXT_PUBLIC_ENV`: Environment name (default: production)

### Security Best Practices
1. NEVER commit `.env.prod` to version control (already in .gitignore)
2. Use strong, randomly generated secrets (minimum 32 characters)
3. Rotate secrets after initial deployment
4. Store secrets securely (password manager or secrets vault)
5. Use specific CORS origins, not wildcards
6. Rotate secrets periodically (every 90 days recommended)

## Troubleshooting

### Check logs
```bash
# All services
docker-compose logs

# Specific service
docker-compose logs hockey-backend
docker-compose logs hockey-frontend
```

### Restart services
```bash
# Restart all
docker-compose restart

# Restart specific service
docker-compose restart hockey-backend
```

### Reset database
```bash
# Stop services
docker-compose down

# Remove database volume (WARNING: This deletes all data)
docker volume rm hockey_hockey-data

# Start services (will recreate database)
docker-compose up -d
```

### Development vs Production

**Development** (docker-compose.yaml):
- Uses build contexts for live development
- Includes debug logging
- Exposes both frontend and backend ports

**Production** (docker-compose.prod.yaml):
- Optimized for production deployment
- Includes nginx reverse proxy (optional)
- Production-grade restart policies
- Reduced logging verbosity