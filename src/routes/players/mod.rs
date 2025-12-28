// Players routes module
mod players;
mod scoring;

// Re-export all route handlers from players module
pub use players::{
    player_create, player_create_form, player_delete, player_detail, player_edit_form,
    player_update, players_get, players_list_partial,
};

// Re-export scoring route handlers
pub use scoring::{player_scoring_get, player_scoring_list_partial};
