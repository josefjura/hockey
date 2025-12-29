-- Test countries fixture
INSERT INTO country (id, name, iihf, iocCode, iso2Code, isHistorical, years, enabled)
VALUES
    (1, 'Canada', 1, 'CAN', 'ca', 0, NULL, 1),
    (2, 'United States', 1, 'USA', 'us', 0, NULL, 1),
    (3, 'Russia', 1, 'RUS', 'ru', 0, NULL, 1),
    (4, 'Finland', 1, 'FIN', 'fi', 0, NULL, 1),
    (5, 'Sweden', 1, 'SWE', 'se', 0, NULL, 1),
    (6, 'Czech Republic', 1, 'CZE', 'cz', 0, NULL, 1),
    (7, 'Slovakia', 1, 'SVK', 'sk', 0, NULL, 1),
    (8, 'Switzerland', 1, 'SUI', 'ch', 0, NULL, 1),
    (9, 'Germany', 1, 'GER', 'de', 0, NULL, 1),
    (10, 'Austria', 1, 'AUT', 'at', 0, NULL, 1),
    (11, 'Latvia', 1, 'LAT', 'lv', 0, NULL, 1),
    (12, 'Norway', 1, 'NOR', 'no', 0, NULL, 1),
    (13, 'Denmark', 1, 'DEN', 'dk', 0, NULL, 1),
    (14, 'France', 1, 'FRA', 'fr', 0, NULL, 1),
    (15, 'Belarus', 1, 'BLR', 'by', 0, NULL, 1),
    (16, 'Soviet Union', 1, 'URS', 'su', 1, '1922-1991', 0),
    (17, 'East Germany', 1, 'GDR', NULL, 1, '1949-1990', 0),
    (18, 'Czechoslovakia', 1, 'TCH', NULL, 1, '1920-1992', 0),
    (19, 'Japan', 1, 'JPN', 'jp', 0, NULL, 1),
    (20, 'South Korea', 1, 'KOR', 'kr', 0, NULL, 1);
