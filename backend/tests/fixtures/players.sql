-- Insert test players (referencing existing countries from migration)
INSERT INTO player (name, country_id, photo_path, created_at, updated_at) VALUES 
    ('Jan Novak', 1, 'http://localhost:9000/hockey-uploads/novak-photo.jpg', '2024-01-01 12:00:00', '2024-01-01 12:00:00'),
    ('Peter Dvorak', 2, 'http://localhost:9000/hockey-uploads/dvorak-photo.jpg', '2024-01-02 12:00:00', '2024-01-02 12:00:00'),
    ('Test Player', 1, NULL, '2024-01-03 12:00:00', '2024-01-03 12:00:00');
