pub mod confirm;
pub mod crud;
pub mod empty_state;
pub mod error;
pub mod forms;
pub mod htmx;
pub mod loading;
pub mod sidebar;
pub mod table;
pub mod toast;

pub use sidebar::sidebar;

/// URL-encoded SVG data URI used as a fallback avatar when a player photo fails to load.
///
/// Renders a grey circle with a "?" character. Use this in `onerror` attributes:
/// ```html
/// onerror="this.src='{AVATAR_FALLBACK_SVG}'"
/// ```
pub const AVATAR_FALLBACK_SVG: &str = "data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22%3E%3Ccircle cx=%2250%22 cy=%2250%22 r=%2250%22 fill=%22%23e5e7eb%22/%3E%3Ctext x=%2250%25%22 y=%2250%25%22 text-anchor=%22middle%22 dy=%22.3em%22 font-size=%2240%22 fill=%22%23666%22%3E%3F%3C/text%3E%3C/svg%3E";
