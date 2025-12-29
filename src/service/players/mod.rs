// Players service module
mod constants;
mod event_stats;
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

// Re-export event stats
pub use event_stats::{
    delete_player_event_stats, get_all_events, get_or_create_player_event_stats,
    get_player_event_stats, update_player_event_stats, PlayerEventStatsEntity,
};

// Re-export constants
pub use constants::{
    EVENT_TYPE_ASSIST_PRIMARY, EVENT_TYPE_ASSIST_SECONDARY, EVENT_TYPE_GOAL,
    FILTER_EVENT_TYPE_ASSISTS, FILTER_EVENT_TYPE_GOALS,
};
