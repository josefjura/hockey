-- Create player_event_stats table to track career totals per event/competition
-- This allows tracking a player's career stats within a specific competition
-- (e.g., NHL career, Olympic career, World Championship career)

CREATE TABLE IF NOT EXISTS player_event_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    goals_total INTEGER DEFAULT 0,
    assists_total INTEGER DEFAULT 0,

    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (player_id) REFERENCES player (id) ON DELETE CASCADE,
    FOREIGN KEY (event_id) REFERENCES event (id) ON DELETE CASCADE,

    -- Ensure one stats entry per player per event
    UNIQUE (player_id, event_id)
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_player_event_stats_player_id ON player_event_stats(player_id);
CREATE INDEX IF NOT EXISTS idx_player_event_stats_event_id ON player_event_stats(event_id);
