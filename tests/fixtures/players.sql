-- Test players fixture (requires countries fixture)
INSERT INTO player (id, name, country_id, birth_date, position)
VALUES
    (1, 'Wayne Gretzky', 1, '1961-01-26', 'Forward'),
    (2, 'Mario Lemieux', 1, '1965-10-05', 'Forward'),
    (3, 'Bobby Orr', 1, '1948-03-20', 'Defense'),
    (4, 'Gordie Howe', 1, '1928-03-31', 'Forward'),
    (5, 'Pavel Datsyuk', 3, '1978-07-20', 'Forward');
