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
    ('HC Slovan Bratislava', 2, 'http://localhost:9000/hockey-uploads/slovan-logo.jpg', '2024-01-02 12:00:00', '2024-01-02 12:00:00');

-- Insert test players
INSERT INTO player (name, country_id, photo_path, created_at, updated_at) VALUES 
    ('Jan Novak', 1, 'http://localhost:9000/hockey-uploads/novak-photo.jpg', '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    ('Peter Dvorak', 2, 'http://localhost:9000/hockey-uploads/dvorak-photo.jpg', '2024-01-02 12:00:00', '2024-01-02 12:00:00');

-- Insert test team participations
INSERT INTO team_participation (team_id, season_id, created_at, updated_at) VALUES 
    (1, 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    (2, 2, '2024-01-02 12:00:00', '2024-01-02 12:00:00');

-- Insert test player contracts
INSERT INTO player_contract (team_participation_id, player_id, created_at, updated_at) VALUES 
    (1, 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    (2, 2, '2024-01-02 12:00:00', '2024-01-02 12:00:00');
