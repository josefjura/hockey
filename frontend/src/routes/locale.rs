use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};

use crate::i18n::Locale;

pub const LOCALE_COOKIE_NAME: &str = "locale";

/// GET /locale/:code - Set locale cookie and redirect back
pub async fn set_locale(
    jar: CookieJar,
    Path(code): Path<String>,
) -> impl IntoResponse {
    // Validate the locale code (defaults to English if invalid)
    let locale = Locale::from_code(&code);
    
    // Build the locale cookie
    let locale_cookie = Cookie::build((LOCALE_COOKIE_NAME, locale.code().to_string()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(365))
        .build();
    
    let jar = jar.add(locale_cookie);
    
    // Redirect to root (the JavaScript will handle redirecting to the current page)
    (jar, Redirect::to("/"))
}

/// Helper function to get locale from cookie jar
pub fn get_locale_from_cookies(jar: &CookieJar) -> Locale {
    jar.get(LOCALE_COOKIE_NAME)
        .map(|cookie| Locale::from_code(cookie.value()))
        .unwrap_or_default()
}
