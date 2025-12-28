// Players service module
mod players;
mod scoring_entities;
mod scoring_queries;

// Re-export everything from players module
pub use players::{
    create_player, delete_player, get_countries, get_player_by_id, get_player_detail,
    get_players, update_player, CreatePlayerEntity, PagedResult, PlayerContractWithTeamEntity,
    PlayerDetailEntity, PlayerEntity, PlayerFilters, SortField, SortOrder, UpdatePlayerEntity,
};

// Re-export scoring entities and queries
pub use scoring_entities::{
    PlayerSeasonStats, PlayerScoringEventEntity, PlayerScoringFilters, ScoringEventSortField,
};
pub use scoring_queries::{
    get_player_season_stats, get_player_scoring_events, get_player_seasons, get_player_teams,
};
