# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability, please email the maintainers directly. Do not open a public issue.

## Secure Production Deployment

### Critical Security Requirements

1. **Rotate All Secrets**: Never use example/default secrets in production
2. **Environment Variables**: Use `.env.prod` file, never commit it to version control
3. **CORS Configuration**: Restrict to specific domains, never use wildcards in production
4. **HTTPS Only**: Always use HTTPS in production (handled by Traefik)
5. **Database Backups**: Regular automated backups of production database

### Secret Management

#### Required Production Secrets

All secrets must be strong, randomly generated, and unique:

```bash
# Generate HMAC key (64 characters recommended)
openssl rand -hex 32

# Generate NextAuth secret (44 characters recommended)
openssl rand -base64 32
```

#### Secret Rotation Schedule

- **Initial Deployment**: Generate all new secrets
- **Regular Rotation**: Every 90 days (recommended)
- **After Incident**: Immediately if compromise suspected
- **Team Changes**: When team members with access leave

#### Secret Rotation Process

1. **Generate new secrets** using commands above
2. **Update `.env.prod`** with new values
3. **Deploy changes**: `docker-compose -f docker-compose.prod.yaml up -d`
4. **Verify services restart successfully**
5. **Monitor logs** for any authentication issues
6. **Store new secrets securely** in password manager/vault

### Exposed Secrets in Git History

This repository previously contained exposed secrets in:
- `docker-compose.prod.yaml` (lines 18, 46 in commit history)

**Actions taken**:
- Secrets removed from current version (Task #16)
- `.env.prod` added to `.gitignore`
- `.env.prod.example` created as template
- Documentation updated with security best practices

**Required actions if deployed**:
1. Rotate all production secrets immediately
2. Review access logs for unauthorized access
3. Monitor for suspicious activity
4. Consider additional security audit

### Docker Compose Security

#### Production Configuration

The production docker-compose file (`docker-compose.prod.yaml`) now:
- Uses environment variables for ALL secrets
- Requires `.env.prod` file (not committed to git)
- Includes secure defaults for optional settings
- Enforces CORS restrictions

#### Development Configuration

The development docker-compose file (`docker-compose.yaml`):
- Uses example secrets with clear warnings
- NOT suitable for production use
- Includes additional debugging features

### CORS Configuration

Proper CORS configuration is critical for security:

```bash
# GOOD: Specific domain(s)
CORS_ORIGINS=https://yourdomain.com

# GOOD: Multiple specific domains
CORS_ORIGINS=https://app.example.com,https://admin.example.com

# BAD: Wildcard (only for development!)
CORS_ORIGINS=*
```

### Database Security

- SQLite database stored in persistent volume
- Regular backups recommended
- No direct external access
- Access only through authenticated API

### Authentication Security

- JWT tokens with RSA signing
- Refresh tokens stored securely
- Access tokens with short expiration (15 minutes)
- Refresh tokens with longer expiration (7 days)
- Automatic token refresh on frontend

## Security Checklist for Production Deployment

Before deploying to production, verify:

- [ ] All secrets rotated and randomly generated
- [ ] `.env.prod` file created with production values
- [ ] `.env.prod` NOT committed to version control
- [ ] CORS_ORIGINS set to specific domain(s)
- [ ] HTTPS enabled (via Traefik or other reverse proxy)
- [ ] Database backup strategy implemented
- [ ] Monitoring and logging configured
- [ ] Security headers configured in reverse proxy
- [ ] Rate limiting configured (if needed)
- [ ] Firewall rules configured
- [ ] SSH access secured with keys only
- [ ] System updates automated
- [ ] Secrets stored in secure vault/password manager

## Dependency Security

Keep dependencies up to date:

```bash
# Backend (Rust)
cd backend
cargo update
cargo audit

# Frontend (Node.js)
cd frontend
yarn upgrade-interactive
yarn audit
```

## Monitoring and Incident Response

### Monitoring

Monitor for:
- Failed authentication attempts
- Unusual API access patterns
- High error rates
- Unauthorized access attempts

### Incident Response

If security incident detected:
1. Rotate all secrets immediately
2. Review access logs
3. Identify scope of compromise
4. Update affected systems
5. Document incident and response
6. Update security procedures if needed

## Security Updates

This document is maintained as part of the repository and should be updated when:
- New security features are added
- Security procedures change
- Vulnerabilities are discovered and fixed
- Deployment configuration changes

Last updated: 2025-12-03
