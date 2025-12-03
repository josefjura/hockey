-- Auth Test Fixtures
-- Provides test users with known passwords for authentication testing

-- Test user: testuser@example.com
-- Password: testpassword123
-- Password hash generated with: bcrypt("testpassword123", DEFAULT_COST)
INSERT INTO users (email, name, password_hash, created_at, updated_at) VALUES
    ('testuser@example.com', 'Test User', '$2b$12$aPiC/B333PM75K78Fe5cOuLnL.xOfLXRtvl4tY/T2WlKnOwYbKWaK', '2024-01-01 12:00:00', '2024-01-01 12:00:00');

-- Additional test user: admin@example.com
-- Password: adminpass456
-- Password hash generated with: bcrypt("adminpass456", DEFAULT_COST)
INSERT INTO users (email, name, password_hash, created_at, updated_at) VALUES
    ('admin@example.com', 'Admin User', '$2b$12$fQ7l2.zcjsRYg.z9QgV.VuVL/ZotDgnbFrYe8HSR/iORGiUOzZT36', '2024-01-01 12:00:00', '2024-01-01 12:00:00');
