use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

use super::{TranslationContext, Locale};

/// Middleware that extracts locale from cookies and adds TranslationContext to request extensions
///
/// This eliminates the need to pass i18n and locale through every function.
/// Use `Extension(t): Extension<TranslationContext>` in your handlers to access translations.
pub async fn translation_context_middleware(
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    // Get locale from cookie (defaults to English if not set)
    let locale = jar
        .get(crate::routes::locale::LOCALE_COOKIE_NAME)
        .map(|cookie| Locale::from_code(cookie.value()))
        .unwrap_or_default();

    // Create translation context and add to request extensions
    let t = TranslationContext::new(locale);
    request.extensions_mut().insert(t);

    // Also insert the locale itself for cases where just the locale is needed
    request.extensions_mut().insert(locale);

    next.run(request).await
}
