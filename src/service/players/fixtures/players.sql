-- Test players fixture (uses country IDs from migration)
INSERT INTO player (id, name, country_id, birth_date, position)
VALUES
    (1, 'Wayne Gretzky', 34, '1961-01-26', 'Forward'),
    (2, 'Mario Lemieux', 34, '1965-10-05', 'Forward'),
    (3, 'Bobby Orr', 34, '1948-03-20', 'Defense'),
    (4, 'Gordie Howe', 34, '1928-03-31', 'Forward'),
    (5, 'Pavel Datsyuk', 153, '1978-07-20', 'Forward'),
    (6, 'Connor McDavid', 34, '1997-01-13', 'Forward'),
    (7, 'Alexander Ovechkin', 153, '1985-09-17', 'Forward'),
    (8, 'Sidney Crosby', 34, '1987-08-07', 'Forward'),
    (9, 'Patrick Kane', 187, '1988-11-19', 'Forward'),
    (10, 'Auston Matthews', 187, '1997-09-17', 'Forward');
