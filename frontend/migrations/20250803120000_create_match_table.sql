-- Add match and score_event tables

CREATE TABLE IF NOT EXISTS match (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  season_id INTEGER NOT NULL,
  home_team_id INTEGER NOT NULL,
  away_team_id INTEGER NOT NULL,
  
  -- Unidentified goals (when we know the score but not the details)
  home_score_unidentified INTEGER DEFAULT 0,
  away_score_unidentified INTEGER DEFAULT 0,
  
  -- Match metadata
  match_date TEXT, -- Using TEXT for ISO 8601 date format
  status TEXT DEFAULT 'scheduled', -- scheduled, in_progress, finished, cancelled
  venue TEXT,
  
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
  
  FOREIGN KEY (season_id) REFERENCES season (id),
  FOREIGN KEY (home_team_id) REFERENCES team (id),
  FOREIGN KEY (away_team_id) REFERENCES team (id),
  
  -- Constraint to prevent team playing against itself
  CHECK (home_team_id != away_team_id),
  
  -- Constraint to ensure non-negative scores
  CHECK (home_score_unidentified >= 0 AND away_score_unidentified >= 0)
);

CREATE TABLE IF NOT EXISTS score_event (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  match_id INTEGER NOT NULL,
  team_id INTEGER NOT NULL,
  scorer_id INTEGER, -- can be NULL for unknown scorer
  assist1_id INTEGER, -- can be NULL
  assist2_id INTEGER, -- can be NULL
  period INTEGER, -- 1, 2, 3, OT, SO (overtime=4, shootout=5)
  time_minutes INTEGER,
  time_seconds INTEGER,
  goal_type TEXT, -- even_strength, power_play, short_handed, penalty_shot, etc.
  
  created_at TEXT DEFAULT CURRENT_TIMESTAMP,
  
  FOREIGN KEY (match_id) REFERENCES match (id) ON DELETE CASCADE,
  FOREIGN KEY (team_id) REFERENCES team (id),
  FOREIGN KEY (scorer_id) REFERENCES player (id),
  FOREIGN KEY (assist1_id) REFERENCES player (id),
  FOREIGN KEY (assist2_id) REFERENCES player (id),
  
  -- Constraint to ensure valid time values
  CHECK (time_minutes >= 0 AND time_minutes <= 60),
  CHECK (time_seconds >= 0 AND time_seconds <= 59),
  CHECK (period >= 1 AND period <= 5)
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_match_season_id ON match(season_id);
CREATE INDEX IF NOT EXISTS idx_match_home_team_id ON match(home_team_id);
CREATE INDEX IF NOT EXISTS idx_match_away_team_id ON match(away_team_id);
CREATE INDEX IF NOT EXISTS idx_match_date ON match(match_date);
CREATE INDEX IF NOT EXISTS idx_match_status ON match(status);

CREATE INDEX IF NOT EXISTS idx_score_event_match_id ON score_event(match_id);
CREATE INDEX IF NOT EXISTS idx_score_event_team_id ON score_event(team_id);
CREATE INDEX IF NOT EXISTS idx_score_event_scorer_id ON score_event(scorer_id);