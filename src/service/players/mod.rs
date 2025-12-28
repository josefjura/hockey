// Players service module
mod player_ops;
mod scoring_entities;
mod scoring_queries;

// Re-export everything from player_ops module
pub use player_ops::{
    create_player, delete_player, get_countries, get_player_by_id, get_player_detail, get_players,
    update_player, CreatePlayerEntity, PagedResult, PlayerContractWithTeamEntity,
    PlayerDetailEntity, PlayerEntity, PlayerFilters, SortField, SortOrder, UpdatePlayerEntity,
};

// Re-export scoring entities and queries
pub use scoring_entities::{
    PlayerScoringEventEntity, PlayerScoringFilters, PlayerSeasonStats, ScoringEventSortField,
};
pub use scoring_queries::{
    get_player_scoring_events, get_player_season_stats, get_player_seasons, get_player_teams,
};
