-- Test team participations fixture (requires seasons and teams fixtures)
INSERT INTO team_participation (id, season_id, team_id, event_id)
VALUES
    (1, 1, 1, 1),  -- Team Canada in 2022 Winter Olympics
    (2, 1, 2, 1),  -- Team USA in 2022 Winter Olympics
    (3, 2, 3, 2),  -- Team Russia in 2023 World Championship
    (4, 2, 4, 2);  -- Team Finland in 2023 World Championship
