-- Test seasons fixture (requires events fixture)
INSERT INTO season (id, year, display_name, event_id, country_id)
VALUES
    (1, 2022, '2022 Winter Olympics', 1, 1),
    (2, 2023, '2023 World Championship', 2, 4),
    (3, 2024, '2024 World Cup', 3, 2);
