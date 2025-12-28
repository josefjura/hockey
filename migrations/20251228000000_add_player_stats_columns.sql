-- Add career statistics columns to player table
-- These columns track total goals and assists (including both identified and unidentified)
ALTER TABLE player ADD COLUMN goals_total INTEGER DEFAULT 0;
ALTER TABLE player ADD COLUMN assists_total INTEGER DEFAULT 0;

-- Create indexes for performance on assist columns
-- These indexes will speed up queries that look up assists for players
CREATE INDEX IF NOT EXISTS idx_score_event_assist1_id ON score_event(assist1_id);
CREATE INDEX IF NOT EXISTS idx_score_event_assist2_id ON score_event(assist2_id);

-- Backfill existing statistics from score_event table
-- Count goals for each player
UPDATE player
SET goals_total = (
    SELECT COUNT(*)
    FROM score_event
    WHERE score_event.scorer_id = player.id
)
WHERE EXISTS (
    SELECT 1 FROM score_event WHERE score_event.scorer_id = player.id
);

-- Count assists for each player (both assist1 and assist2)
UPDATE player
SET assists_total = (
    SELECT COUNT(*)
    FROM score_event
    WHERE score_event.assist1_id = player.id OR score_event.assist2_id = player.id
)
WHERE EXISTS (
    SELECT 1 FROM score_event
    WHERE score_event.assist1_id = player.id OR score_event.assist2_id = player.id
);
