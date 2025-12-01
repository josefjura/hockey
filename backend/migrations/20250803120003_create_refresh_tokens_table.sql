-- Add migration script here

DROP TABLE IF EXISTS refresh_tokens;

CREATE TABLE refresh_tokens (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  token TEXT NOT NULL UNIQUE,
  user_id INTEGER NOT NULL,
  expires_at DATETIME NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  revoked_at DATETIME,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create index on token for fast lookups during validation
CREATE INDEX idx_refresh_tokens_token ON refresh_tokens(token);

-- Create index on user_id for fast lookups when retrieving user's tokens
CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);

-- Create index on expires_at for fast cleanup of expired tokens
CREATE INDEX idx_refresh_tokens_expires_at ON refresh_tokens(expires_at);

-- Create composite index on user_id and revoked_at for active token lookups
CREATE INDEX idx_refresh_tokens_user_revoked ON refresh_tokens(user_id, revoked_at);
