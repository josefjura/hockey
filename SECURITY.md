# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability, please email the maintainers directly. **Do not open a public issue.**

## Secure Production Deployment

### Critical Security Requirements

1. **Rotate All Secrets**: Never use example/default secrets in production
2. **Environment Variables**: Use `.env` file with secure values, never commit it
3. **HTTPS Only**: Always use HTTPS in production (via reverse proxy)
4. **Database Backups**: Regular automated backups of production database
5. **Session Security**: HttpOnly, Secure, SameSite cookies

### Secret Management

#### Required Production Secrets

All secrets must be strong, randomly generated, and unique:

```bash
# Generate session secret (recommended 32 bytes)
openssl rand -hex 32
```

#### Secret Rotation Schedule

- **Initial Deployment**: Generate all new secrets
- **Regular Rotation**: Every 90 days (recommended)
- **After Incident**: Immediately if compromise suspected
- **Team Changes**: When team members with access leave

#### Secret Rotation Process

1. **Generate new secret** using command above
2. **Update `.env`** with new SESSION_SECRET value
3. **Restart service**: `systemctl restart hockey` or rebuild/redeploy
4. **Existing sessions invalidated**: Users must re-login
5. **Store new secret securely** in password manager/vault

### Authentication Security

The application uses session-based authentication with these security features:

#### Session Security
- **HttpOnly cookies**: Prevents JavaScript access
- **Secure flag**: HTTPS-only transmission (production)
- **SameSite**: CSRF protection
- **Expiration**: Automatic session timeout

#### Password Security
- **Bcrypt hashing**: Strong password hashing (cost factor 12)
- **Minimum length**: 8 characters enforced
- **No plaintext storage**: Passwords never stored in plaintext

### Database Security

#### SQLite Security
- **File permissions**: `chmod 600 hockey.db` (owner read/write only)
- **Backup encryption**: Encrypt database backups at rest
- **Connection limits**: Single process access (SQLite default)

#### Migration Security
- **Review migrations**: Inspect all migration files before running
- **Backup before migrate**: Always backup before applying new migrations
- **One-way operations**: Migrations are not automatically reversible

### Network Security

#### Reverse Proxy Requirements
Always use a reverse proxy (Nginx, Caddy) for:
- **SSL/TLS termination**: HTTPS with valid certificates
- **Rate limiting**: Prevent brute force attacks
- **Request filtering**: Block malicious requests
- **Header security**: Add security headers (HSTS, CSP, etc.)

#### Example Nginx Configuration

```nginx
server {
    listen 443 ssl http2;
    server_name hockey.example.com;

    # SSL Configuration
    ssl_certificate /etc/letsencrypt/live/hockey.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/hockey.example.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    # Security Headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline';" always;

    # Rate Limiting
    limit_req_zone $binary_remote_addr zone=login:10m rate=5r/m;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /auth/login {
        limit_req zone=login burst=10 nodelay;
        proxy_pass http://localhost:8080;
    }
}
```

### Application Security

#### Input Validation
- **Server-side validation**: All user input validated on server
- **Type safety**: Rust's type system prevents many vulnerabilities
- **SQL injection prevention**: SQLx parameterized queries
- **XSS prevention**: Maud escapes HTML by default

#### CSRF Protection
- **Token validation**: CSRF tokens in forms
- **SameSite cookies**: Additional CSRF protection
- **State-changing operations**: POST/PUT/DELETE only

#### File Upload Security
(If/when file uploads are implemented)
- **File type validation**: Whitelist allowed types
- **Size limits**: Enforce maximum file size
- **Virus scanning**: Scan uploaded files
- **Separate storage**: Store outside web root

### Monitoring and Logging

#### Security Logging
Log these security events:
- Failed login attempts
- Session creation/destruction
- Access to sensitive operations
- Database errors
- Configuration changes

#### Log Management
- **Secure storage**: Restrict access to log files
- **No sensitive data**: Don't log passwords, tokens, or PII
- **Retention policy**: Rotate and archive logs regularly
- **Monitoring**: Alert on suspicious patterns

### Deployment Checklist

Before deploying to production:

- [ ] All secrets generated and securely stored
- [ ] `.env` file created with production secrets
- [ ] Database file permissions set to 600
- [ ] HTTPS configured with valid certificate
- [ ] Reverse proxy configured with security headers
- [ ] Rate limiting enabled for login endpoints
- [ ] Database backups configured
- [ ] Monitoring and alerting set up
- [ ] Firewall rules configured (only HTTPS port open)
- [ ] Service user created (don't run as root)
- [ ] Systemd service configured with security options

### Security Headers

Recommended security headers (configured in reverse proxy):

```
Strict-Transport-Security: max-age=31536000; includeSubDomains
X-Frame-Options: SAMEORIGIN
X-Content-Type-Options: nosniff
X-XSS-Protection: 1; mode=block
Content-Security-Policy: default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline';
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: geolocation=(), microphone=(), camera=()
```

### Incident Response

If a security incident occurs:

1. **Isolate**: Take affected systems offline if necessary
2. **Assess**: Determine scope and severity
3. **Contain**: Stop ongoing attack/breach
4. **Rotate**: Change all secrets immediately
5. **Notify**: Inform affected users if data compromised
6. **Investigate**: Review logs and determine root cause
7. **Remediate**: Fix vulnerability
8. **Document**: Record incident and response
9. **Improve**: Update security measures

### Regular Security Maintenance

#### Weekly
- Review access logs for anomalies
- Check for failed login attempts
- Verify backup completion

#### Monthly
- Update dependencies (`cargo update`)
- Review security advisories
- Test backup restoration

#### Quarterly
- Rotate secrets
- Security audit
- Penetration testing (if resources available)

### Security Best Practices

1. **Principle of Least Privilege**: Grant minimum necessary access
2. **Defense in Depth**: Multiple layers of security
3. **Fail Securely**: Errors should not expose sensitive information
4. **Keep Updated**: Regularly update dependencies
5. **Security by Design**: Consider security from the start
6. **Secure Defaults**: Default configuration should be secure
7. **Audit Trail**: Log security-relevant events

### Dependency Security

#### Rust Dependencies
```bash
# Check for known vulnerabilities
cargo audit

# Update dependencies
cargo update

# Review dependency tree
cargo tree
```

#### Security Advisories
Monitor:
- [RustSec Advisory Database](https://rustsec.org/)
- [GitHub Security Advisories](https://github.com/advisories)
- Dependency GitHub repositories

### Contact

For security concerns, contact the maintainers at:
- **Email**: [Your security contact email]
- **Response Time**: Within 48 hours for critical issues

---

**Last Updated**: 2025-12-16
**Version**: 2.0 (HTMX rewrite)
