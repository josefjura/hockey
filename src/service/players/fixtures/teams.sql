-- Test teams fixture (uses country IDs from migration)
INSERT INTO team (id, name, country_id)
VALUES
    (1, 'Team Canada', 34),
    (2, 'Team USA', 187),
    (3, 'Team Russia', 153),
    (4, 'Team Finland', 65),
    (5, 'Team Sweden', 168);
