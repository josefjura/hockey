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
   # Copy and edit for production
   cp .env.example .env.prod
   
   # Required production variables
   HOCKEY_DOMAIN=your-domain.com
   HOCKEY_GITHUB_REPOSITORY=your-org/hockey
   HOCKEY_HMAC_KEY=your-secure-32-char-key
   NEXTAUTH_SECRET=your-secure-32-char-secret
   DATABASE_URL=sqlite:///app/data/hockey.db
   ```

2. **Deploy with Traefik integration**:
   ```bash
   docker-compose -f docker-compose.yaml -f docker-compose.prod.yaml up -d
   ```

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

### Required Variables
- `HOCKEY_HMAC_KEY`: JWT signing key for backend authentication (min 32 chars)
- `NEXTAUTH_SECRET`: NextAuth.js secret for session encryption (min 32 chars) - Standard NextAuth variable

### Optional Variables
- `NEXTAUTH_URL`: Frontend URL (default: http://localhost:3000) - Standard NextAuth variable
- `DATABASE_URL`: SQLite database path (default: sqlite:///app/data/hockey.db) - Standard SQLx variable
- `HOCKEY_DOMAIN`: Production domain name (default: hockey.dev)
- `HOCKEY_GITHUB_REPOSITORY`: GitHub repository for container images

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