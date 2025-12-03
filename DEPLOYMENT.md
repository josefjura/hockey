# Deployment Guide

This guide covers deploying the Hockey Management System to production.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Initial Setup](#initial-setup)
- [Key Generation](#key-generation)
- [Environment Configuration](#environment-configuration)
- [Docker Deployment](#docker-deployment)
- [Manual Deployment](#manual-deployment)
- [Post-Deployment Setup](#post-deployment-setup)
- [Security Checklist](#security-checklist)
- [Backup and Recovery](#backup-and-recovery)
- [Monitoring and Maintenance](#monitoring-and-maintenance)

## Prerequisites

### Server Requirements

- **OS**: Linux (Ubuntu 22.04+ recommended)
- **CPU**: 2+ cores
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 20GB minimum (more depending on data volume)
- **Network**: HTTPS-capable (port 443)

### Software Requirements

- **Docker**: 24.0+ and Docker Compose 2.0+
- **OR Manual**: Rust 1.81+, Node.js 18+, SQLite
- **SSL/TLS**: Certificate authority (Let's Encrypt recommended)
- **Reverse Proxy**: Nginx or Traefik (for SSL termination)

## Initial Setup

### 1. Clone Repository

```bash
# Clone the repository
git clone https://github.com/josefjura/hockey.git
cd hockey

# Checkout the latest stable tag
git checkout $(git describe --tags --abbrev=0)
```

### 2. Directory Structure

```bash
# Create data and key directories
mkdir -p data keys

# Set appropriate permissions
chmod 700 keys
chmod 755 data
```

## Key Generation

### Generate RSA Keys for JWT

**Important**: These keys are critical for authentication security. Generate them on your production server and never commit them to version control.

```bash
cd backend

# Generate 4096-bit RSA key pair
./scripts/generate_keys.sh

# Move keys to secure location
mv jwt_private.pem ../keys/
mv jwt_public.pem ../keys/

# Verify key generation
ls -la ../keys/
# Should show:
# -rw------- jwt_private.pem (600 permissions)
# -rw-r--r-- jwt_public.pem (644 permissions)
```

### Generate Secret Keys

Generate secure random keys for HMAC and NextAuth:

```bash
# Generate HMAC key (64 characters recommended)
openssl rand -hex 32

# Generate NextAuth secret (64 characters recommended)
openssl rand -hex 32

# Save these in your .env file
```

## Environment Configuration

### Production Environment File

Create `.env` in the project root:

```bash
# Backend Configuration
DATABASE_URL=sqlite:./data/hockey.db
HMAC_KEY=<your-64-char-hex-key-from-openssl>
JWT_PRIVATE_KEY_PATH=/app/keys/jwt_private.pem
JWT_PUBLIC_KEY_PATH=/app/keys/jwt_public.pem
JWT_ACCESS_TOKEN_DURATION_MINUTES=15
JWT_REFRESH_TOKEN_DURATION_DAYS=7
HOST=0.0.0.0
PORT=8080
ENVIRONMENT=production
RUST_LOG=info

# CORS Configuration (CRITICAL: Set your actual domain)
CORS_ORIGINS=https://yourdomain.com,https://www.yourdomain.com
CORS_METHODS=GET,POST,PUT,DELETE,OPTIONS
CORS_HEADERS=content-type,authorization,x-requested-with

# Frontend Configuration
NEXTAUTH_SECRET=<your-64-char-hex-key-from-openssl>
NEXTAUTH_URL=https://yourdomain.com
AUTH_TRUST_HOST=true
HOCKEY_BACKEND_URL=http://hockey-backend:8080

# Production Build Variables
HOCKEY_DOMAIN=yourdomain.com
NEXT_PUBLIC_ENV=production
```

### Important Notes

- Replace `yourdomain.com` with your actual domain
- Never use `*` for CORS_ORIGINS in production
- Keep `.env` file permissions restrictive: `chmod 600 .env`
- Use strong random keys (minimum 32 characters)
- Backup your `.env` file securely

## Docker Deployment

### Using Docker Compose

```bash
# Build and start services
docker-compose -f docker-compose.prod.yaml up -d

# View logs
docker-compose -f docker-compose.prod.yaml logs -f

# Stop services
docker-compose -f docker-compose.prod.yaml down
```

### Docker Compose Production Configuration

Example `docker-compose.prod.yaml`:

```yaml
version: '3.8'

services:
  hockey-backend:
    image: ghcr.io/josefjura/hockey/backend:latest
    container_name: hockey-backend-prod
    restart: unless-stopped
    environment:
      - DATABASE_URL=${DATABASE_URL}
      - HMAC_KEY=${HMAC_KEY}
      - JWT_PRIVATE_KEY_PATH=${JWT_PRIVATE_KEY_PATH}
      - JWT_PUBLIC_KEY_PATH=${JWT_PUBLIC_KEY_PATH}
      - JWT_ACCESS_TOKEN_DURATION_MINUTES=${JWT_ACCESS_TOKEN_DURATION_MINUTES}
      - JWT_REFRESH_TOKEN_DURATION_DAYS=${JWT_REFRESH_TOKEN_DURATION_DAYS}
      - HOST=${HOST}
      - PORT=${PORT}
      - ENVIRONMENT=${ENVIRONMENT}
      - CORS_ORIGINS=${CORS_ORIGINS}
      - CORS_METHODS=${CORS_METHODS}
      - CORS_HEADERS=${CORS_HEADERS}
      - RUST_LOG=${RUST_LOG}
    volumes:
      - ./data:/app/data
      - ./keys:/app/keys:ro
    networks:
      - hockey-network
    healthcheck:
      test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  hockey-frontend:
    image: ghcr.io/josefjura/hockey/frontend:latest
    container_name: hockey-frontend-prod
    restart: unless-stopped
    environment:
      - NEXTAUTH_SECRET=${NEXTAUTH_SECRET}
      - NEXTAUTH_URL=${NEXTAUTH_URL}
      - AUTH_TRUST_HOST=${AUTH_TRUST_HOST}
      - HOCKEY_BACKEND_URL=${HOCKEY_BACKEND_URL}
    depends_on:
      - hockey-backend
    networks:
      - hockey-network
    healthcheck:
      test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://localhost:3000/api/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  nginx:
    image: nginx:alpine
    container_name: hockey-nginx
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/nginx/ssl:ro
    depends_on:
      - hockey-frontend
    networks:
      - hockey-network

networks:
  hockey-network:
    driver: bridge
```

### Nginx Configuration

Example `nginx.conf` for SSL termination:

```nginx
events {
    worker_connections 1024;
}

http {
    # Redirect HTTP to HTTPS
    server {
        listen 80;
        server_name yourdomain.com www.yourdomain.com;
        return 301 https://$server_name$request_uri;
    }

    # HTTPS server
    server {
        listen 443 ssl http2;
        server_name yourdomain.com www.yourdomain.com;

        # SSL Configuration
        ssl_certificate /etc/nginx/ssl/fullchain.pem;
        ssl_certificate_key /etc/nginx/ssl/privkey.pem;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers HIGH:!aNULL:!MD5;
        ssl_prefer_server_ciphers on;

        # Security Headers
        add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
        add_header X-Frame-Options "SAMEORIGIN" always;
        add_header X-Content-Type-Options "nosniff" always;
        add_header X-XSS-Protection "1; mode=block" always;

        # Frontend
        location / {
            proxy_pass http://hockey-frontend-prod:3000;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection 'upgrade';
            proxy_set_header Host $host;
            proxy_cache_bypass $http_upgrade;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Backend API
        location /api {
            proxy_pass http://hockey-backend-prod:8080;
            proxy_http_version 1.1;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
}
```

## Manual Deployment

### Backend Deployment

```bash
cd backend

# Build release binary
cargo build --release

# Copy binary to deployment location
sudo cp target/release/jura_hockey /usr/local/bin/

# Copy keys to secure location
sudo mkdir -p /etc/hockey/keys
sudo cp jwt_private.pem jwt_public.pem /etc/hockey/keys/
sudo chmod 600 /etc/hockey/keys/jwt_private.pem
sudo chmod 644 /etc/hockey/keys/jwt_public.pem

# Create systemd service
sudo nano /etc/systemd/system/hockey-backend.service
```

Backend systemd service file:

```ini
[Unit]
Description=Hockey Management Backend
After=network.target

[Service]
Type=simple
User=hockey
Group=hockey
WorkingDirectory=/var/lib/hockey
EnvironmentFile=/etc/hockey/backend.env
ExecStart=/usr/local/bin/jura_hockey
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

### Frontend Deployment

```bash
cd frontend

# Build production bundle
yarn build

# Copy build to deployment location
sudo mkdir -p /var/www/hockey
sudo cp -r .next public /var/www/hockey/
sudo cp package.json yarn.lock /var/www/hockey/

# Install production dependencies
cd /var/www/hockey
sudo yarn install --production

# Create systemd service
sudo nano /etc/systemd/system/hockey-frontend.service
```

Frontend systemd service file:

```ini
[Unit]
Description=Hockey Management Frontend
After=network.target

[Service]
Type=simple
User=hockey
Group=hockey
WorkingDirectory=/var/www/hockey
EnvironmentFile=/etc/hockey/frontend.env
ExecStart=/usr/bin/yarn start
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

### Start Services

```bash
# Enable and start backend
sudo systemctl enable hockey-backend
sudo systemctl start hockey-backend

# Enable and start frontend
sudo systemctl enable hockey-frontend
sudo systemctl start hockey-frontend

# Check status
sudo systemctl status hockey-backend
sudo systemctl status hockey-frontend
```

## Post-Deployment Setup

### 1. Create Admin User

```bash
cd backend
cargo run --bin create_admin

# Or if using Docker:
docker exec -it hockey-backend-prod cargo run --bin create_admin
```

Follow the prompts to create your first admin user.

### 2. Verify Deployment

```bash
# Test backend health
curl https://yourdomain.com/api/health

# Test frontend
curl https://yourdomain.com/

# Test authentication
curl -X POST https://yourdomain.com/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@example.com","password":"your-password"}'
```

### 3. Initial Data Setup

1. Log in to the admin interface
2. Add countries, teams, players as needed
3. Configure events and seasons

## Security Checklist

Before going live, verify:

- [ ] RSA keys generated and stored securely
- [ ] Strong random secrets generated (min 32 chars)
- [ ] `.env` file has restrictive permissions (600)
- [ ] CORS configured with explicit origins (no wildcards)
- [ ] ENVIRONMENT set to `production`
- [ ] SSL/TLS certificates installed and valid
- [ ] Firewall configured (only 80, 443 open)
- [ ] Admin user created with strong password
- [ ] Database file has appropriate permissions
- [ ] Keys directory has restrictive permissions (700)
- [ ] No secrets committed to git
- [ ] Security headers configured in reverse proxy
- [ ] HSTS enabled with long max-age
- [ ] Regular backup strategy implemented

## Backup and Recovery

### Database Backup

```bash
# Create backup directory
mkdir -p backups

# Backup database
sqlite3 data/hockey.db ".backup 'backups/hockey-$(date +%Y%m%d-%H%M%S).db'"

# Or using SQLite CLI
cp data/hockey.db backups/hockey-$(date +%Y%m%d-%H%M%S).db

# Compress backup
gzip backups/hockey-$(date +%Y%m%d-%H%M%S).db
```

### Automated Backup Script

```bash
#!/bin/bash
# /usr/local/bin/backup-hockey.sh

BACKUP_DIR="/var/backups/hockey"
DB_PATH="/var/lib/hockey/hockey.db"
RETENTION_DAYS=30

# Create backup
mkdir -p "$BACKUP_DIR"
sqlite3 "$DB_PATH" ".backup '$BACKUP_DIR/hockey-$(date +%Y%m%d-%H%M%S).db'"
gzip "$BACKUP_DIR"/hockey-*.db

# Remove old backups
find "$BACKUP_DIR" -name "hockey-*.db.gz" -mtime +$RETENTION_DAYS -delete

# Upload to remote storage (optional)
# aws s3 sync "$BACKUP_DIR" s3://your-bucket/hockey-backups/
```

Add to crontab:

```bash
# Daily backup at 2 AM
0 2 * * * /usr/local/bin/backup-hockey.sh
```

### Recovery

```bash
# Stop services
docker-compose -f docker-compose.prod.yaml down
# Or: sudo systemctl stop hockey-backend hockey-frontend

# Restore database
gunzip -c backups/hockey-20240101-020000.db.gz > data/hockey.db

# Restart services
docker-compose -f docker-compose.prod.yaml up -d
# Or: sudo systemctl start hockey-backend hockey-frontend
```

## Monitoring and Maintenance

### Health Checks

```bash
# Backend health
curl https://yourdomain.com/api/health

# Frontend health
curl https://yourdomain.com/api/health
```

### Logs

```bash
# Docker logs
docker-compose -f docker-compose.prod.yaml logs -f

# Systemd logs
sudo journalctl -u hockey-backend -f
sudo journalctl -u hockey-frontend -f
```

### Updates

```bash
# Pull latest images
docker-compose -f docker-compose.prod.yaml pull

# Restart with new images
docker-compose -f docker-compose.prod.yaml up -d

# Verify update
docker-compose -f docker-compose.prod.yaml logs -f
```

### Token Cleanup

Clean up expired refresh tokens periodically:

```bash
# Add to crontab (daily at 3 AM)
0 3 * * * sqlite3 /var/lib/hockey/hockey.db "DELETE FROM refresh_tokens WHERE expires_at <= datetime('now')"
```

### Performance Monitoring

Monitor key metrics:

- CPU and memory usage
- Disk space
- Database size
- Request latency
- Error rates

Use tools like:
- Prometheus + Grafana
- Docker stats
- htop/top
- Application logs

## Troubleshooting

### Cannot Connect to Backend

1. Check backend is running: `docker ps` or `systemctl status`
2. Check firewall rules: `sudo ufw status`
3. Check logs: `docker logs hockey-backend-prod`
4. Verify CORS configuration
5. Check SSL certificates

### Authentication Failures

1. Verify JWT keys are mounted correctly
2. Check key file permissions
3. Verify HMAC_KEY is set
4. Check token expiration settings
5. Review auth logs

### Database Issues

1. Check database file exists and is readable
2. Verify migrations ran successfully
3. Check disk space
4. Review database logs
5. Restore from backup if corrupted

### Performance Issues

1. Check database size and indexes
2. Monitor memory usage
3. Review slow query logs
4. Check for resource constraints
5. Scale horizontally if needed

## Support

For issues or questions:

- **GitHub Issues**: https://github.com/josefjura/hockey/issues
- **Documentation**: https://github.com/josefjura/hockey
- **Security Issues**: Report privately to maintainers

## License

This project is licensed under the MIT License. See LICENSE file for details.
