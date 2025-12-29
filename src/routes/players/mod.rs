// Players routes module
mod event_stats_handlers;
mod handlers;
mod scoring;

// Re-export all route handlers from handlers module
pub use handlers::{
    player_create, player_create_form, player_delete, player_detail, player_edit_form,
    player_update, players_get, players_list_partial,
};

// Re-export scoring route handlers
pub use scoring::{player_scoring_get, player_scoring_list_partial};

// Re-export event stats handlers
pub use event_stats_handlers::{
    event_stats_create, event_stats_create_form, event_stats_delete, event_stats_edit_form,
    event_stats_update,
};
