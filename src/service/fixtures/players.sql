-- Test players fixture (requires countries from migrations)
-- Using subqueries to resolve country IDs dynamically from migrations
INSERT INTO player (id, name, country_id, photo_path, birth_date, birth_place, height_cm, weight_kg, position, shoots)
VALUES
    -- Canadian players
    (1, 'Connor McDavid', (SELECT id FROM country WHERE name = 'Canada'), NULL, '1997-01-13', 'Richmond Hill, ON', 185, 88, 'C', 'L'),
    (2, 'Sidney Crosby', (SELECT id FROM country WHERE name = 'Canada'), NULL, '1987-08-07', 'Halifax, NS', 180, 91, 'C', 'L'),
    (3, 'Nathan MacKinnon', (SELECT id FROM country WHERE name = 'Canada'), NULL, '1995-09-01', 'Halifax, NS', 183, 92, 'C', 'R'),

    -- American players
    (4, 'Auston Matthews', (SELECT id FROM country WHERE name = 'United States'), NULL, '1997-09-17', 'San Ramon, CA', 191, 100, 'C', 'L'),
    (5, 'Patrick Kane', (SELECT id FROM country WHERE name = 'United States'), NULL, '1988-11-19', 'Buffalo, NY', 178, 80, 'RW', 'L'),

    -- Russian players
    (6, 'Alexander Ovechkin', (SELECT id FROM country WHERE name = 'Russia'), NULL, '1985-09-17', 'Moscow, Russia', 191, 108, 'LW', 'R'),
    (7, 'Nikita Kucherov', (SELECT id FROM country WHERE name = 'Russia'), NULL, '1993-06-17', 'Maykop, Russia', 180, 82, 'RW', 'L'),

    -- Finnish players
    (8, 'Aleksander Barkov', (SELECT id FROM country WHERE name = 'Finland'), NULL, '1995-09-02', 'Tampere, Finland', 191, 97, 'C', 'L'),

    -- Swedish players
    (9, 'Erik Karlsson', (SELECT id FROM country WHERE name = 'Sweden'), NULL, '1990-05-31', 'Landsbro, Sweden', 183, 84, 'D', 'R'),
    (10, 'Victor Hedman', (SELECT id FROM country WHERE name = 'Sweden'), NULL, '1990-12-18', 'Örnsköldsvik, Sweden', 198, 109, 'D', 'L');
