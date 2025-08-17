-- Countries (use high IDs to avoid conflicts with existing countries)
INSERT INTO country (id, name, iso2Code, iihf, isHistorical, years, enabled, created_at, updated_at) VALUES 
    (9001, 'Country A', 'CA', 1, 0, NULL, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00'),
    (9002, 'Country B', 'CB', 1, 0, NULL, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00');

-- Teams  
INSERT INTO team (id, name, country_id, created_at, updated_at) VALUES 
    (1, 'Team A', 9001, '2024-01-01 00:00:00', '2024-01-01 00:00:00'),
    (2, 'Team B', 9002, '2024-01-01 00:00:00', '2024-01-01 00:00:00');

-- Players
INSERT INTO player (id, name, country_id, photo_path, created_at, updated_at) VALUES 
    (1, 'Player A', 9001, NULL, '2024-01-01 00:00:00', '2024-01-01 00:00:00'),
    (2, 'Player B', 9002, NULL, '2024-01-01 00:00:00', '2024-01-01 00:00:00');

-- Team Participations
INSERT INTO team_participation (id, team_id, season_id, created_at, updated_at) VALUES
    (1, 1, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00'),
    (2, 2, 2, '2024-01-01 00:00:00', '2024-01-01 00:00:00');

-- Player Contracts (Players assigned to team participations)
INSERT INTO player_contract (id, team_participation_id, player_id, created_at, updated_at) VALUES
    (1, 1, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00'),
    (2, 2, 2, '2024-01-01 00:00:00', '2024-01-01 00:00:00');