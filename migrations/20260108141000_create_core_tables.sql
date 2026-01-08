-- Core schema: event, season, team, player, team_participation, player_contract
-- Using STRICT for proper type enforcement and sqlx type inference

CREATE TABLE event (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  country_id INTEGER,
  start_date TEXT,
  end_date TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (country_id) REFERENCES country(id)
) STRICT;

CREATE TABLE season (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  year INTEGER NOT NULL,
  display_name TEXT,
  event_id INTEGER NOT NULL,
  country_id INTEGER,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (event_id) REFERENCES event(id),
  FOREIGN KEY (country_id) REFERENCES country(id)
) STRICT;

CREATE TABLE team (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  short_name TEXT,
  country_id INTEGER,
  logo_path TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (country_id) REFERENCES country(id)
) STRICT;

CREATE TABLE player (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  country_id INTEGER NOT NULL,
  photo_path TEXT,
  birth_date TEXT,
  birth_place TEXT,
  height_cm INTEGER,
  weight_kg INTEGER,
  position TEXT,
  shoots TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (country_id) REFERENCES country(id)
) STRICT;

CREATE TABLE team_participation (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  team_id INTEGER NOT NULL,
  season_id INTEGER NOT NULL,
  event_id INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (team_id) REFERENCES team(id),
  FOREIGN KEY (season_id) REFERENCES season(id),
  FOREIGN KEY (event_id) REFERENCES event(id),
  UNIQUE (team_id, season_id, event_id)
) STRICT;

CREATE TABLE player_contract (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player_id INTEGER NOT NULL,
  team_participation_id INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (player_id) REFERENCES player(id),
  FOREIGN KEY (team_participation_id) REFERENCES team_participation(id)
) STRICT;
