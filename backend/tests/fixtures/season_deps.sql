-- Insert test events (referencing existing countries from migration)
INSERT INTO event (name, country_id, created_at, updated_at) VALUES 
    ('World Championship', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    ('European Championship', 2, '2024-01-02 12:00:00', '2024-01-02 12:00:00'),
    ('National League', NULL, '2024-01-03 12:00:00', '2024-01-03 12:00:00');
