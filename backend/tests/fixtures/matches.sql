-- Insert test events (referencing existing countries from migration)
INSERT INTO event (name, country_id, created_at, updated_at) VALUES 
    ('Czech Hockey League', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    ('Slovak Hockey League', 2, '2024-01-02 12:00:00', '2024-01-02 12:00:00');

-- Insert test seasons
INSERT INTO season (year, display_name, event_id, created_at, updated_at) VALUES 
    (2023, '2023 Czech Season', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    (2024, '2024 Slovak Season', 2, '2024-01-02 12:00:00', '2024-01-02 12:00:00');

-- Insert test teams
INSERT INTO team (name, country_id, logo_path, created_at, updated_at) VALUES 
    ('HC Sparta Praha', 1, 'http://localhost:9000/hockey-uploads/sparta-logo.jpg', '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    ('HC Slovan Bratislava', 2, 'http://localhost:9000/hockey-uploads/slovan-logo.jpg', '2024-01-02 12:00:00', '2024-01-02 12:00:00'),
    ('Home Team', 1, NULL, '2024-01-03 12:00:00', '2024-01-03 12:00:00'),
    ('Away Team', 1, NULL, '2024-01-04 12:00:00', '2024-01-04 12:00:00');

-- Insert test matches
INSERT INTO "match" (season_id, home_team_id, away_team_id, home_score, away_score, match_date, venue, status, created_at, updated_at) VALUES 
    (1, 1, 2, 3, 2, '2024-01-15 19:00:00', 'O2 Arena', 'completed', '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    (1, 3, 4, 1, 0, '2024-01-16 20:00:00', 'Ice Palace', 'in_progress', '2024-01-02 12:00:00', '2024-01-02 12:00:00');
