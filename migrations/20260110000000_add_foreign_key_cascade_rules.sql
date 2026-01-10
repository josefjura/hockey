-- Add proper CASCADE/RESTRICT rules to foreign key constraints
-- SQLite doesn't support ALTER TABLE for foreign keys, so we need to recreate tables

-- Disable foreign key constraints during migration
PRAGMA foreign_keys = OFF;

-- ============================================================================
-- event: Add RESTRICT for country_id
-- ============================================================================
ALTER TABLE event RENAME TO event_old;

CREATE TABLE event (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  country_id INTEGER,
  start_date TEXT,
  end_date TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (country_id) REFERENCES country(id) ON DELETE RESTRICT
) STRICT;

INSERT INTO event SELECT * FROM event_old;
DROP TABLE event_old;

-- ============================================================================
-- season: Add CASCADE for event_id, RESTRICT for country_id
-- ============================================================================
ALTER TABLE season RENAME TO season_old;

CREATE TABLE season (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  year INTEGER NOT NULL,
  display_name TEXT,
  event_id INTEGER NOT NULL,
  country_id INTEGER,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (event_id) REFERENCES event(id) ON DELETE CASCADE,
  FOREIGN KEY (country_id) REFERENCES country(id) ON DELETE RESTRICT
) STRICT;

INSERT INTO season SELECT * FROM season_old;
DROP TABLE season_old;

-- ============================================================================
-- team: Add RESTRICT for country_id
-- ============================================================================
ALTER TABLE team RENAME TO team_old;

CREATE TABLE team (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  short_name TEXT,
  country_id INTEGER,
  logo_path TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (country_id) REFERENCES country(id) ON DELETE RESTRICT
) STRICT;

INSERT INTO team SELECT * FROM team_old;
DROP TABLE team_old;

-- ============================================================================
-- player: Add RESTRICT for country_id
-- ============================================================================
ALTER TABLE player RENAME TO player_old;

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
  FOREIGN KEY (country_id) REFERENCES country(id) ON DELETE RESTRICT
) STRICT;

INSERT INTO player SELECT * FROM player_old;
DROP TABLE player_old;

-- ============================================================================
-- team_participation: Add CASCADE for all foreign keys
-- ============================================================================
ALTER TABLE team_participation RENAME TO team_participation_old;

CREATE TABLE team_participation (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  team_id INTEGER NOT NULL,
  season_id INTEGER NOT NULL,
  event_id INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (team_id) REFERENCES team(id) ON DELETE CASCADE,
  FOREIGN KEY (season_id) REFERENCES season(id) ON DELETE CASCADE,
  FOREIGN KEY (event_id) REFERENCES event(id) ON DELETE CASCADE,
  UNIQUE (team_id, season_id, event_id)
) STRICT;

INSERT INTO team_participation SELECT * FROM team_participation_old;
DROP TABLE team_participation_old;

-- ============================================================================
-- player_contract: Add CASCADE for all foreign keys
-- ============================================================================
ALTER TABLE player_contract RENAME TO player_contract_old;

CREATE TABLE player_contract (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player_id INTEGER NOT NULL,
  team_participation_id INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (player_id) REFERENCES player(id) ON DELETE CASCADE,
  FOREIGN KEY (team_participation_id) REFERENCES team_participation(id) ON DELETE CASCADE
) STRICT;

INSERT INTO player_contract SELECT * FROM player_contract_old;
DROP TABLE player_contract_old;

-- ============================================================================
-- match: CASCADE for season_id, RESTRICT for team references (preserve history)
-- ============================================================================
ALTER TABLE match RENAME TO match_old;

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
  FOREIGN KEY (season_id) REFERENCES season(id) ON DELETE CASCADE,
  FOREIGN KEY (home_team_id) REFERENCES team(id) ON DELETE RESTRICT,
  FOREIGN KEY (away_team_id) REFERENCES team(id) ON DELETE RESTRICT,
  CHECK (home_team_id != away_team_id),
  CHECK (home_score_unidentified >= 0 AND away_score_unidentified >= 0)
) STRICT;

INSERT INTO match SELECT * FROM match_old;
DROP TABLE match_old;

-- ============================================================================
-- score_event: CASCADE for match_id, RESTRICT for team/player references
-- ============================================================================
ALTER TABLE score_event RENAME TO score_event_old;

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
  FOREIGN KEY (match_id) REFERENCES match(id) ON DELETE CASCADE,
  FOREIGN KEY (team_id) REFERENCES team(id) ON DELETE RESTRICT,
  FOREIGN KEY (scorer_id) REFERENCES player(id) ON DELETE RESTRICT,
  FOREIGN KEY (assist1_id) REFERENCES player(id) ON DELETE RESTRICT,
  FOREIGN KEY (assist2_id) REFERENCES player(id) ON DELETE RESTRICT,
  CHECK (time_minutes >= 0 AND time_minutes <= 60),
  CHECK (time_seconds >= 0 AND time_seconds <= 59),
  CHECK (period >= 1 AND period <= 5)
) STRICT;

INSERT INTO score_event SELECT * FROM score_event_old;
DROP TABLE score_event_old;

-- ============================================================================
-- player_event_stats: CASCADE for both foreign keys
-- ============================================================================
ALTER TABLE player_event_stats RENAME TO player_event_stats_old;

CREATE TABLE player_event_stats (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player_id INTEGER NOT NULL,
  event_id INTEGER NOT NULL,
  goals_total INTEGER NOT NULL DEFAULT 0,
  assists_total INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (player_id) REFERENCES player(id) ON DELETE CASCADE,
  FOREIGN KEY (event_id) REFERENCES event(id) ON DELETE CASCADE,
  UNIQUE (player_id, event_id)
) STRICT;

INSERT INTO player_event_stats SELECT * FROM player_event_stats_old;
DROP TABLE player_event_stats_old;

-- ============================================================================
-- player_property_change: CASCADE for player_id, SET NULL for season_id
-- ============================================================================
ALTER TABLE player_property_change RENAME TO player_property_change_old;

CREATE TABLE player_property_change (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player_id INTEGER NOT NULL,
  season_id INTEGER,
  property_type TEXT NOT NULL,
  old_value TEXT,
  new_value TEXT,
  change_date TEXT,
  description TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (player_id) REFERENCES player(id) ON DELETE CASCADE,
  FOREIGN KEY (season_id) REFERENCES season(id) ON DELETE SET NULL,
  UNIQUE (player_id, property_type, change_date)
) STRICT;

INSERT INTO player_property_change SELECT * FROM player_property_change_old;
DROP TABLE player_property_change_old;

-- Recreate indexes that were on player_property_change
CREATE INDEX idx_player_property_change_player_id ON player_property_change(player_id);
CREATE INDEX idx_player_property_change_season_id ON player_property_change(season_id);
CREATE INDEX idx_player_property_change_date ON player_property_change(change_date);

-- Re-enable foreign key constraints
PRAGMA foreign_keys = ON;
