# Deployment Guide

This guide explains how to deploy the Hockey Management System to production using Docker and GitHub Container Registry.

## Overview

The deployment follows a **cargo-dist style** workflow:
1. Push a version tag (e.g., `v0.1.0`) to trigger the build
2. GitHub Actions builds and pushes Docker image to `ghcr.io`
3. Pull the image to your server manually or use Watchtower for auto-updates
4. No SSH deployment - simple pull and restart workflow

## Prerequisites

- Linux server with Docker and Docker Compose installed
- Domain name pointed to your server (optional, but recommended)
- SSH access to the server
- GitHub account with access to the repository

## Server Setup

### 1. Install Docker

```bash
# Update package index
sudo apt update

# Install prerequisites
sudo apt install -y apt-transport-https ca-certificates curl software-properties-common

# Add Docker's GPG key
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

# Add Docker repository
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install Docker
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin

# Add your user to docker group
sudo usermod -aG docker $USER
```

### 2. Set Up Traefik (Optional)

If you're using Traefik as a reverse proxy:

```bash
# Create Traefik network
docker network create traefik-net

# Create Traefik directory
mkdir -p ~/traefik
cd ~/traefik

# Create docker-compose.yml for Traefik
# (Add your Traefik configuration here)
```

### 3. Prepare Deployment Directory

```bash
# Create deployment directory
mkdir -p ~/hockey
cd ~/hockey

# Create data directory with proper permissions
mkdir -p data
chmod 755 data
```

## GitHub Setup

### 1. Container Registry Permissions

The GitHub Actions workflow uses the built-in `GITHUB_TOKEN` to push images to GitHub Container Registry. **No manual secrets needed.**

### 2. Verify Workflow Permissions

1. Go to your GitHub repository → **Settings** → **Actions** → **General**
2. Under "Workflow permissions", ensure:
   - ✅ **Read and write permissions** is selected
   - ✅ "Allow GitHub Actions to create and approve pull requests" (optional)

### 3. Container Visibility (First Time Only)

After the first image is pushed, make it publicly accessible:

1. Go to your GitHub profile → **Packages**
2. Find the `hockey` package
3. Click **Package settings**
4. Under "Danger Zone" → **Change visibility** → **Public**

This allows pulling the image without authentication.

## Production Configuration

### 1. Create Production Environment File

On your server, create `.env.prod`:

```bash
cd ~/hockey
nano .env.prod
```

Add the following (replace with actual values):

```bash
# Session Secret (Generate with: openssl rand -hex 32)
SESSION_SECRET=your-actual-secret-here-64-characters-minimum

# Application URL
HOCKEY_URL=https://hockey.yourdomain.com

# Logging
RUST_LOG=info,hockey=debug
```

### 2. Download docker-compose.prod.yaml

```bash
# Download from repository
curl -O https://raw.githubusercontent.com/josefjura/hockey/master/docker-compose.prod.yaml

# Or copy manually
nano docker-compose.prod.yaml
```

Update the Traefik labels with your domain:

```yaml
labels:
  - "traefik.http.routers.hockey.rule=Host(`hockey.yourdomain.com`)"
```

### 3. Verify Configuration

```bash
# Check environment variables
cat .env.prod

# Check docker-compose configuration
docker compose -f docker-compose.prod.yaml config
```

## Deployment Workflow

### Cargo-Dist Style Release Process

This project follows the cargo-dist pattern where **tags trigger releases**:

1. **Create a version tag:**
   ```bash
   git tag v0.1.0
   git push --tags
   ```

2. **GitHub Actions automatically:**
   - Runs full test suite (format, clippy, cargo test)
   - Builds Docker image for linux/amd64 and linux/arm64
   - Pushes to `ghcr.io/josefjura/hockey:latest`
   - Creates GitHub Release with auto-generated notes
   - Adds deployment instructions to release notes

3. **Monitor the build:**
   - Go to your repository → **Actions** tab
   - Watch the "Build and Push Docker Image" workflow
   - Build typically takes 5-10 minutes

### Manual Deployment on Server

Once the image is built and pushed to ghcr.io, deploy to your server:

```bash
# SSH to your server
ssh user@your-server.com
cd ~/hockey

# Pull latest image from GitHub Container Registry
docker compose -f docker-compose.prod.yaml pull

# Restart the application
docker compose -f docker-compose.prod.yaml up -d

# Check logs
docker compose -f docker-compose.prod.yaml logs -f hockey

# Verify health
curl http://localhost:8080/health
```

### Automatic Updates with Watchtower

Set up Watchtower to automatically pull and restart when new images are pushed:

**Option 1: Add to docker-compose.prod.yaml**

```yaml
services:
  hockey:
    # ... existing configuration ...
    labels:
      - "com.centurylinklabs.watchtower.enable=true"
  
  watchtower:
    image: containrrr/watchtower
    container_name: watchtower
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
    command: --interval 300 --cleanup
    restart: unless-stopped
```

**Option 2: Separate Watchtower Deployment**

Create `~/watchtower/docker-compose.yml`:

```yaml
version: '3.8'

services:
  watchtower:
    image: containrrr/watchtower
    container_name: watchtower
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
    environment:
      - WATCHTOWER_CLEANUP=true
      - WATCHTOWER_POLL_INTERVAL=300  # Check every 5 minutes
      - WATCHTOWER_INCLUDE_STOPPED=false
      - WATCHTOWER_REVIVE_STOPPED=false
    restart: unless-stopped
```

With Watchtower running:
1. Push a new tag → GitHub Actions builds image
2. Watchtower detects new image within 5 minutes
3. Automatically pulls and restarts the container
4. Zero manual intervention needed! 🚀

## First Deployment

### 1. Commit and Push Changes

Ensure all changes are committed to master:

```bash
git add .
git commit -m "Repository reorganization and cargo-dist CI/CD"
git push origin master
```

### 2. Create and Push First Tag

Trigger the first build by creating a version tag:

```bash
# Create tag
git tag v0.1.0

# Push tag (triggers GitHub Actions)
git push --tags
```

### 3. Monitor the Build

1. Go to GitHub → Your Repository → **Actions**
2. You'll see two workflows running:
   - **Test** - Runs on master push (tests only)
   - **Build and Push Docker Image** - Runs on tag (builds + release)
3. Wait for "Build and Push Docker Image" to complete (~5-10 min)
4. Check the **Releases** page for the auto-created release

### 4. Deploy on Server

Once the build completes, SSH to your server:

```bash
ssh user@your-server.com
cd ~/hockey

# Pull the newly built image
docker compose -f docker-compose.prod.yaml pull

# Start the application
docker compose -f docker-compose.prod.yaml up -d

# Watch logs
docker compose -f docker-compose.prod.yaml logs -f
```

## Post-Deployment

### 1. Create Admin User

```bash
# On your server (or via SSH)

# Run create_admin in the container
docker exec -it hockey /app/create_admin
```

### 2. Verify Deployment

- **Visit your application:** https://hockey.yourdomain.com (or http://your-ip:8080)
- **Check health endpoint:** `curl http://localhost:8080/health`
- **View logs:** `docker compose -f docker-compose.prod.yaml logs -f`
- **Check container status:** `docker ps`

### 3. Set Up Monitoring (Optional)

Consider these monitoring solutions:

- **Watchtower** - Auto-update containers (see section above)
- **Uptime Monitoring** - UptimeRobot, StatusCake, or Healthchecks.io
- **Log Aggregation** - Loki + Grafana for centralized logs
- **Metrics** - Prometheus + Grafana for performance metrics
- **Backup Automation** - Cron job for daily database backups

## Subsequent Deployments

After the initial setup, deployments are simple:

1. **Make your changes and commit:**
   ```bash
   git add .
   git commit -m "Add new feature"
   git push origin master
   ```

2. **Create a new tag:**
   ```bash
   git tag v0.2.0
   git push --tags
   ```

3. **GitHub Actions builds automatically**

4. **Deploy (choose one):**
   
   **Option A - Manual:**
   ```bash
   ssh user@server "cd ~/hockey && docker compose -f docker-compose.prod.yaml pull && docker compose -f docker-compose.prod.yaml up -d"
   ```
   
   **Option B - Watchtower:**
   Just wait 5 minutes - Watchtower handles it automatically! 🎉

## Maintenance

### View Logs

```bash
docker compose -f docker-compose.prod.yaml logs -f
```

### Restart Application

```bash
docker compose -f docker-compose.prod.yaml restart
```

### Update Application

```bash
docker compose -f docker-compose.prod.yaml pull
docker compose -f docker-compose.prod.yaml up -d
```

### Backup Database

```bash
# Create backup
cp data/hockey.db data/hockey.db.backup-$(date +%Y%m%d)

# Or use sqlite3
sqlite3 data/hockey.db ".backup data/hockey.db.backup-$(date +%Y%m%d)"
```

### Restore Database

```bash
# Stop the application
docker compose -f docker-compose.prod.yaml down

# Restore backup
cp data/hockey.db.backup-20231219 data/hockey.db

# Start the application
docker compose -f docker-compose.prod.yaml up -d
```

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker compose -f docker-compose.prod.yaml logs

# Check if port is already in use
sudo netstat -tulpn | grep 8080

# Check disk space
df -h
```

### Database Issues

```bash
# Check database file permissions
ls -la data/

# Verify database integrity
sqlite3 data/hockey.db "PRAGMA integrity_check;"
```

### Network Issues

```bash
# Check if Traefik network exists
docker network ls | grep traefik-net

# Create if missing
docker network create traefik-net
```

## Security Best Practices

1. **Use Strong Secrets**: Generate with `openssl rand -hex 32`
2. **Keep Docker Updated**: Regularly update Docker and images
3. **Use Watchtower**: Auto-update containers
4. **Enable Firewall**: Only expose necessary ports
5. **Regular Backups**: Automate database backups
6. **Monitor Logs**: Set up log aggregation
7. **HTTPS Only**: Always use TLS in production
8. **Limit SSH Access**: Use key-based auth, disable password auth

## Support

For issues or questions:
- GitHub Issues: https://github.com/josefjura/hockey/issues
- Documentation: https://github.com/josefjura/hockey/blob/master/README.md
