-- Stats and career tracking: player_event_stats, player_property_change
-- Using STRICT for proper type enforcement and sqlx type inference

CREATE TABLE player_event_stats (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player_id INTEGER NOT NULL,
  event_id INTEGER NOT NULL,
  goals_total INTEGER NOT NULL DEFAULT 0,
  assists_total INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (player_id) REFERENCES player(id),
  FOREIGN KEY (event_id) REFERENCES event(id),
  UNIQUE (player_id, event_id)
) STRICT;

CREATE TABLE player_property_change (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player_id INTEGER NOT NULL,
  season_id INTEGER,
  property_type TEXT NOT NULL,
  change_date TEXT,
  description TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (player_id) REFERENCES player(id),
  FOREIGN KEY (season_id) REFERENCES season(id)
) STRICT;

CREATE INDEX idx_player_property_change_player_id ON player_property_change(player_id);
CREATE INDEX idx_player_property_change_season_id ON player_property_change(season_id);
CREATE INDEX idx_player_property_change_date ON player_property_change(change_date);
