pub mod middleware;
pub mod password;
pub mod session;
pub mod signing;

pub use middleware::{require_auth, SESSION_COOKIE_NAME};
pub use password::verify_password;
pub use signing::{sign_session_id, verify_signed_session_id};

// Re-export these for potential future use
#[allow(unused_imports)]
pub use middleware::{get_session, optional_auth};
#[allow(unused_imports)]
pub use password::hash_password;
pub use session::{Session, SessionStore};
