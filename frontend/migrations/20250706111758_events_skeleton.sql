-- Add migration script here
CREATE TABLE IF NOT EXISTS event (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name text NOT NULL,
  country_id int,
  FOREIGN KEY (country_id) REFERENCES country (id)
);