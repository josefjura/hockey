#!/bin/bash

# Generate RSA key pair for JWT signing (RS256 algorithm)
# This script creates a 4096-bit RSA key pair for secure JWT token signing

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BACKEND_DIR="$(dirname "$SCRIPT_DIR")"
PRIVATE_KEY="$BACKEND_DIR/jwt_private.pem"
PUBLIC_KEY="$BACKEND_DIR/jwt_public.pem"

echo "Generating RSA key pair for JWT signing..."

# Generate private key (4096-bit)
openssl genrsa -out "$PRIVATE_KEY" 4096

# Extract public key from private key
openssl rsa -in "$PRIVATE_KEY" -pubout -out "$PUBLIC_KEY"

# Set appropriate permissions
chmod 600 "$PRIVATE_KEY"
chmod 644 "$PUBLIC_KEY"

echo "✓ Private key generated: $PRIVATE_KEY (permissions: 600)"
echo "✓ Public key generated: $PUBLIC_KEY (permissions: 644)"
echo ""
echo "⚠️  IMPORTANT: These keys are gitignored. Never commit them to version control!"
echo "⚠️  Backup these keys securely for production environments."
