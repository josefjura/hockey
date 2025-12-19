-- Add migration script here

CREATE TABLE IF NOT EXISTS season (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  year int NOT NULL,
  display_name text,
  event_id int NOT NULL,
  FOREIGN KEY (event_id) REFERENCES event (id)
);

CREATE TABLE IF NOT EXISTS player (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name text NOT NULL,
  country_id int NOT NULL,
  FOREIGN KEY (country_id) REFERENCES country (id)
);

CREATE TABLE IF NOT EXISTS country (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name text NOT NULL
);

CREATE TABLE IF NOT EXISTS event (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name text NOT NULL,
  country_id int,
  FOREIGN KEY (country_id) REFERENCES country (id)
);

CREATE TABLE IF NOT EXISTS team (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name text,
  country_id int,
  FOREIGN KEY (country_id) REFERENCES country (id)
);

CREATE TABLE IF NOT EXISTS team_participation (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  team_id int NOT NULL,
  season_id int NOT NULL,
  FOREIGN KEY (team_id) REFERENCES team (id),
  FOREIGN KEY (season_id) REFERENCES season (id)
);

CREATE TABLE IF NOT EXISTS player_contract (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  team_participation_id int NOT NULL,
  player_id int NOT NULL,
  FOREIGN KEY (team_participation_id) REFERENCES team_participation (id),
  FOREIGN KEY (player_id) REFERENCES player (id)
);