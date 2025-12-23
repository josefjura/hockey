# Docker Deployment

## Development

```bash
docker compose up -d
```

Access at http://localhost:8080

## Production

1. **Set up environment variables:**
   ```bash
   cp .env.prod.example .env.prod
   # Edit .env.prod with production values
   ```

2. **Deploy:**
   ```bash
   docker compose -f docker-compose.prod.yaml up -d
   ```