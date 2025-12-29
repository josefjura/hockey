-- Test teams fixture (requires countries fixture)
INSERT INTO team (id, name, country_id)
VALUES
    (1, 'Team Canada', 1),
    (2, 'Team USA', 2),
    (3, 'Team Russia', 3),
    (4, 'Team Finland', 4),
    (5, 'Team Sweden', 5);
