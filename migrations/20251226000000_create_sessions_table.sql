-- Create sessions table for production-ready session storage
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    user_email TEXT NOT NULL,
    user_name TEXT NOT NULL,
    csrf_token TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Index for faster lookups by expiration time (for cleanup)
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at);

-- Index for faster lookups by user_id
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id);
