-- Test events fixture (requires countries fixture)
INSERT INTO event (id, name, country_id)
VALUES
    (1, 'Winter Olympics', 1),
    (2, 'World Championship', 4),
    (3, 'World Cup', 2);
