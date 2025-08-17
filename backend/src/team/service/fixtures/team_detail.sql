-- Events
INSERT INTO event (id, name, country_id, created_at, updated_at) VALUES 
    (1, 'Test Tournament', 34, '2024-01-01 12:00:00', '2024-01-01 12:00:00');

-- Seasons
INSERT INTO season (id, year, display_name, event_id, created_at, updated_at) VALUES 
    (1, 2023, '2023 Season', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    (2, 2024, '2024 Season', 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00');

-- Teams (using real country IDs: 34=Canada, 183=Sweden)
INSERT INTO team (id, name, country_id, logo_path, created_at, updated_at) VALUES 
    (1, 'Test Team A', 34, 'test-logo-a.jpg', '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    (2, 'Test Team B', 183, 'test-logo-b.jpg', '2024-01-01 12:00:00', '2024-01-01 12:00:00');

-- Team Participations
INSERT INTO team_participation (id, team_id, season_id) VALUES 
    (1, 1, 1), -- Team A in 2023 Season
    (2, 1, 2), -- Team A in 2024 Season
    (3, 2, 1); -- Team B in 2023 Season

-- Players (using real country IDs: 34=Canada, 183=Sweden, 65=Finland)
INSERT INTO player (id, name, country_id, created_at, updated_at) VALUES 
    (1, 'John Doe', 34, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    (2, 'Jane Smith', 183, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    (3, 'Mike Johnson', 65, '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    (4, 'Erik Karlsson', 183, '2024-01-01 12:00:00', '2024-01-01 12:00:00');

-- Player Contracts
INSERT INTO player_contract (id, team_participation_id, player_id) VALUES 
    (1, 1, 1), -- John Doe on Team A in 2023
    (2, 1, 2), -- Jane Smith on Team A in 2023
    (3, 2, 1), -- John Doe on Team A in 2024
    (4, 2, 3), -- Mike Johnson on Team A in 2024
    (5, 3, 4); -- Erik Karlsson on Team B in 2023
