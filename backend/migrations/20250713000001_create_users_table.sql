-- Add migration script here

DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  email TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  name TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Insert a default admin user (password: "admin123")
INSERT INTO users (email, password_hash, name) VALUES 
('admin@example.com', '$2b$12$EnsOQLrapjuJcOIXJJIXIeyHrXpkmxYbColj9UGRvdvJxPok/2Z2y', 'Admin User');
