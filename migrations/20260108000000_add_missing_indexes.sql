-- Add missing database indexes for foreign keys and common query patterns
-- This migration addresses performance issues as the dataset grows
-- Issue #177: Missing database indexes on foreign keys and common queries

-- Foreign key indexes for JOIN operations
-- These dramatically improve query performance when joining tables

-- player_contract indexes (used heavily in player roster queries)
CREATE INDEX IF NOT EXISTS idx_player_contract_player_id
    ON player_contract(player_id);

CREATE INDEX IF NOT EXISTS idx_player_contract_team_participation_id
    ON player_contract(team_participation_id);

-- team_participation indexes (used in team roster and season queries)
CREATE INDEX IF NOT EXISTS idx_team_participation_team_id
    ON team_participation(team_id);

CREATE INDEX IF NOT EXISTS idx_team_participation_season_id
    ON team_participation(season_id);

-- season indexes (used in event/competition hierarchy queries)
CREATE INDEX IF NOT EXISTS idx_season_event_id
    ON season(event_id);

-- event indexes (used in event listing and filtering)
CREATE INDEX IF NOT EXISTS idx_event_country_id
    ON event(country_id);

-- player indexes (used in nationality filtering and lookups)
CREATE INDEX IF NOT EXISTS idx_player_country_id
    ON player(country_id);

-- team indexes (used in team filtering by nationality)
CREATE INDEX IF NOT EXISTS idx_team_country_id
    ON team(country_id);

-- Timestamp indexes for sorting and chronological queries
-- These improve ORDER BY performance on list pages

CREATE INDEX IF NOT EXISTS idx_team_created_at
    ON team(created_at);

CREATE INDEX IF NOT EXISTS idx_player_created_at
    ON player(created_at);

CREATE INDEX IF NOT EXISTS idx_event_created_at
    ON event(created_at);

CREATE INDEX IF NOT EXISTS idx_season_created_at
    ON season(created_at);

-- Composite indexes for common query patterns
-- These optimize queries that filter and sort together

-- Team participation lookup by team, ordered by season
CREATE INDEX IF NOT EXISTS idx_team_participation_team_season
    ON team_participation(team_id, season_id);

-- Player contracts for a team participation
CREATE INDEX IF NOT EXISTS idx_player_contract_team_player
    ON player_contract(team_participation_id, player_id);

-- Seasons by event, ordered by year
CREATE INDEX IF NOT EXISTS idx_season_event_year
    ON season(event_id, year);
