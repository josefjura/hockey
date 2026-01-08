-- Performance indexes for foreign keys and timestamps
-- These improve query performance for joins and sorted queries

-- Foreign key indexes
CREATE INDEX idx_event_country_id ON event(country_id);
CREATE INDEX idx_season_event_id ON season(event_id);
CREATE INDEX idx_season_country_id ON season(country_id);
CREATE INDEX idx_team_country_id ON team(country_id);
CREATE INDEX idx_player_country_id ON player(country_id);

CREATE INDEX idx_team_participation_team_id ON team_participation(team_id);
CREATE INDEX idx_team_participation_season_id ON team_participation(season_id);
CREATE INDEX idx_team_participation_event_id ON team_participation(event_id);

CREATE INDEX idx_player_contract_player_id ON player_contract(player_id);
CREATE INDEX idx_player_contract_team_participation_id ON player_contract(team_participation_id);

CREATE INDEX idx_match_season_id ON match(season_id);
CREATE INDEX idx_match_home_team_id ON match(home_team_id);
CREATE INDEX idx_match_away_team_id ON match(away_team_id);
CREATE INDEX idx_match_date ON match(match_date);
CREATE INDEX idx_match_status ON match(status);

CREATE INDEX idx_score_event_match_id ON score_event(match_id);
CREATE INDEX idx_score_event_team_id ON score_event(team_id);
CREATE INDEX idx_score_event_scorer_id ON score_event(scorer_id);

CREATE INDEX idx_player_event_stats_player_id ON player_event_stats(player_id);
CREATE INDEX idx_player_event_stats_event_id ON player_event_stats(event_id);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);

-- Timestamp indexes for sorting and filtering
CREATE INDEX idx_event_created_at ON event(created_at);
CREATE INDEX idx_season_created_at ON season(created_at);
CREATE INDEX idx_team_created_at ON team(created_at);
CREATE INDEX idx_player_created_at ON player(created_at);
CREATE INDEX idx_team_participation_created_at ON team_participation(created_at);
CREATE INDEX idx_player_contract_created_at ON player_contract(created_at);
CREATE INDEX idx_match_created_at ON match(created_at);
CREATE INDEX idx_score_event_created_at ON score_event(created_at);
CREATE INDEX idx_player_event_stats_created_at ON player_event_stats(created_at);
CREATE INDEX idx_player_property_change_created_at ON player_property_change(created_at);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_sessions_created_at ON sessions(created_at);
