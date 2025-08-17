-- Insert test events (referencing existing countries from migration)
INSERT INTO event (name, country_id, created_at, updated_at) VALUES 
    ('World Championship', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    ('European Championship', 2, '2024-01-02 12:00:00', '2024-01-02 12:00:00'),
    ('National League', NULL, '2024-01-03 12:00:00', '2024-01-03 12:00:00');

-- Insert test seasons
INSERT INTO season (year, display_name, event_id, created_at, updated_at) VALUES 
    (2023, '2023 World Championship', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    (2024, '2024 European Championship', 2, '2024-01-02 12:00:00', '2024-01-02 12:00:00'),
    (2025, '2025 National League Season', 3, '2024-01-03 12:00:00', '2024-01-03 12:00:00');
