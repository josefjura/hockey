-- Test events fixture (uses country IDs from migration)
INSERT INTO event (id, name, country_id)
VALUES
    (1, 'Winter Olympics', 34),
    (2, 'World Championship', 65),
    (3, 'World Cup', 187);
