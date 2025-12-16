-- Add audit columns (created_at, updated_at) to tables that don't have them yet
-- Note: SQLite doesn't support CURRENT_TIMESTAMP as default for existing tables,
-- so we add them as NULL first, then update with current timestamp

-- Add audit columns to country table
ALTER TABLE country ADD created_at TEXT;
ALTER TABLE country ADD updated_at TEXT;
UPDATE country SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;

-- Add audit columns to event table  
ALTER TABLE event ADD created_at TEXT;
ALTER TABLE event ADD updated_at TEXT;
UPDATE event SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;

-- Add audit columns to season table
ALTER TABLE season ADD created_at TEXT;
ALTER TABLE season ADD updated_at TEXT;
UPDATE season SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;

-- Add audit columns to player table
ALTER TABLE player ADD created_at TEXT;
ALTER TABLE player ADD updated_at TEXT;
UPDATE player SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;

-- Add audit columns to team table
ALTER TABLE team ADD created_at TEXT;
ALTER TABLE team ADD updated_at TEXT;
UPDATE team SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;

-- Add audit columns to team_participation table
ALTER TABLE team_participation ADD created_at TEXT;
ALTER TABLE team_participation ADD updated_at TEXT;
UPDATE team_participation SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;

-- Add audit columns to player_contract table
ALTER TABLE player_contract ADD created_at TEXT;
ALTER TABLE player_contract ADD updated_at TEXT;
UPDATE player_contract SET created_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE created_at IS NULL;
