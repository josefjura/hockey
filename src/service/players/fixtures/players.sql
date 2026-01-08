-- Test players fixture (uses country IDs from migration)
INSERT INTO player (id, name, country_id, birth_date, position, shoots)
VALUES
    (1, 'Connor McDavid', 34, '1997-01-13', 'C', 'L'),
    (2, 'Wayne Gretzky', 34, '1961-01-26', 'C', 'L'),
    (3, 'Mario Lemieux', 34, '1965-10-05', 'C', 'R'),
    (4, 'Bobby Orr', 34, '1948-03-20', 'D', 'L'),
    (5, 'Gordie Howe', 34, '1928-03-31', 'RW', 'R'),
    (6, 'Pavel Datsyuk', 153, '1978-07-20', 'C', 'L'),
    (7, 'Alexander Ovechkin', 153, '1985-09-17', 'LW', 'R'),
    (8, 'Sidney Crosby', 34, '1987-08-07', 'C', 'L'),
    (9, 'Patrick Kane', 187, '1988-11-19', 'RW', 'L'),
    (10, 'Auston Matthews', 187, '1997-09-17', 'C', 'L');
