-- Match tracking system: match and score_event tables
-- Using STRICT for proper type enforcement and sqlx type inference

CREATE TABLE match (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  season_id INTEGER NOT NULL,
  home_team_id INTEGER NOT NULL,
  away_team_id INTEGER NOT NULL,
  home_score_unidentified INTEGER NOT NULL DEFAULT 0,
  away_score_unidentified INTEGER NOT NULL DEFAULT 0,
  match_date TEXT,
  status TEXT NOT NULL DEFAULT 'scheduled',
  venue TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (season_id) REFERENCES season(id),
  FOREIGN KEY (home_team_id) REFERENCES team(id),
  FOREIGN KEY (away_team_id) REFERENCES team(id),
  CHECK (home_team_id != away_team_id),
  CHECK (home_score_unidentified >= 0 AND away_score_unidentified >= 0)
) STRICT;

CREATE TABLE score_event (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  match_id INTEGER NOT NULL,
  team_id INTEGER NOT NULL,
  scorer_id INTEGER,
  assist1_id INTEGER,
  assist2_id INTEGER,
  period INTEGER,
  time_minutes INTEGER,
  time_seconds INTEGER,
  goal_type TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (match_id) REFERENCES match(id),
  FOREIGN KEY (team_id) REFERENCES team(id),
  FOREIGN KEY (scorer_id) REFERENCES player(id),
  FOREIGN KEY (assist1_id) REFERENCES player(id),
  FOREIGN KEY (assist2_id) REFERENCES player(id),
  CHECK (time_minutes >= 0 AND time_minutes <= 60),
  CHECK (time_seconds >= 0 AND time_seconds <= 59),
  CHECK (period >= 1 AND period <= 5)
) STRICT;
