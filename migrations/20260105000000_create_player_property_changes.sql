-- Create player_property_change table for tracking career milestones and property changes
CREATE TABLE IF NOT EXISTS player_property_change (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    change_date TEXT NOT NULL,          -- ISO 8601 date format (YYYY-MM-DD)
    property_type TEXT NOT NULL,        -- Position|Trade|Role|JerseyNumber|Status|Retirement|Other
    old_value TEXT,                     -- Optional: previous value
    new_value TEXT,                     -- Optional: new value
    description TEXT NOT NULL,          -- User-provided description (max 500 chars)
    season_id INTEGER,                  -- Optional: link to specific season

    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (player_id) REFERENCES player (id) ON DELETE CASCADE,
    FOREIGN KEY (season_id) REFERENCES season (id) ON DELETE SET NULL,

    -- Business rule: prevent exact duplicates
    UNIQUE (player_id, change_date, property_type, description)
);

-- Performance indexes
CREATE INDEX IF NOT EXISTS idx_player_property_change_player_id
    ON player_property_change(player_id);

CREATE INDEX IF NOT EXISTS idx_player_property_change_season_id
    ON player_property_change(season_id);

CREATE INDEX IF NOT EXISTS idx_player_property_change_date
    ON player_property_change(change_date DESC);
