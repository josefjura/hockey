// Re-export table components for convenience
pub use crate::views::components::table::pagination;

// Sub-modules
mod actions;
mod empty_states;
mod layout;
mod modals;

// Re-export all public functions
#[allow(unused_imports)]
pub use actions::{table_actions, table_actions_i18n};
pub use empty_states::{empty_state, empty_state_i18n};
pub use layout::{page_header, page_header_i18n};
#[allow(unused_imports)]
pub use modals::{modal_form, modal_form_i18n, modal_form_multipart, modal_form_multipart_i18n};
