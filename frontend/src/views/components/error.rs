use maud::{html, Markup, PreEscaped};

/// Simple inline error message
pub fn error_message(message: &str) -> Markup {
    html! {
        div class="error" style="padding: 1rem; margin: 1rem 0;" {
            (message)
        }
    }
}

/// Error state variants
pub enum ErrorVariant {
    /// General error
    Error,
    /// Not found (404)
    NotFound,
    /// Access denied (403)
    Forbidden,
    /// Server error (500)
    ServerError,
    /// Network/connection error
    NetworkError,
    /// Validation error
    ValidationError,
}

impl ErrorVariant {
    fn icon(&self) -> &'static str {
        match self {
            ErrorVariant::Error | ErrorVariant::ValidationError => {
                r#"<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="10"/>
                    <line x1="12" y1="8" x2="12" y2="12"/>
                    <line x1="12" y1="16" x2="12.01" y2="16"/>
                </svg>"#
            }
            ErrorVariant::NotFound => {
                r#"<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="11" cy="11" r="8"/>
                    <line x1="21" y1="21" x2="16.65" y2="16.65"/>
                    <line x1="8" y1="11" x2="14" y2="11"/>
                </svg>"#
            }
            ErrorVariant::Forbidden => {
                r#"<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                    <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                </svg>"#
            }
            ErrorVariant::ServerError => {
                r#"<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="2" y="2" width="20" height="8" rx="2" ry="2"/>
                    <rect x="2" y="14" width="20" height="8" rx="2" ry="2"/>
                    <line x1="6" y1="6" x2="6.01" y2="6"/>
                    <line x1="6" y1="18" x2="6.01" y2="18"/>
                </svg>"#
            }
            ErrorVariant::NetworkError => {
                r#"<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="1" y1="1" x2="23" y2="23"/>
                    <path d="M16.72 11.06A10.94 10.94 0 0 1 19 12.55"/>
                    <path d="M5 12.55a10.94 10.94 0 0 1 5.17-2.39"/>
                    <path d="M10.71 5.05A16 16 0 0 1 22.58 9"/>
                    <path d="M1.42 9a15.91 15.91 0 0 1 4.7-2.88"/>
                    <path d="M8.53 16.11a6 6 0 0 1 6.95 0"/>
                    <line x1="12" y1="20" x2="12.01" y2="20"/>
                </svg>"#
            }
        }
    }

    fn color(&self) -> &'static str {
        match self {
            ErrorVariant::Error | ErrorVariant::ValidationError => "#ef4444",
            ErrorVariant::NotFound => "#6b7280",
            ErrorVariant::Forbidden => "#f59e0b",
            ErrorVariant::ServerError => "#ef4444",
            ErrorVariant::NetworkError => "#6366f1",
        }
    }
}

/// Full-page or section error state with icon, message, and optional actions
///
/// # Arguments
/// - `variant`: Type of error to display
/// - `title`: Error title
/// - `message`: Detailed error message
/// - `retry_url`: Optional URL for retry button (HTMX GET)
/// - `retry_target`: Target element for retry (e.g., "#content")
pub fn error_state(
    variant: ErrorVariant,
    title: &str,
    message: &str,
    retry_url: Option<&str>,
    retry_target: Option<&str>,
) -> Markup {
    let color = variant.color();
    let icon = variant.icon();

    html! {
        div class="error-state" style="
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 3rem 2rem;
            text-align: center;
            min-height: 300px;
        " {
            // Icon
            div style=(format!("color: {}; margin-bottom: 1.5rem; opacity: 0.8;", color)) {
                (PreEscaped(icon))
            }

            // Title
            h2 style="
                font-size: 1.5rem;
                font-weight: 700;
                color: var(--gray-800);
                margin-bottom: 0.75rem;
            " {
                (title)
            }

            // Message
            p style="
                color: var(--gray-600);
                max-width: 400px;
                line-height: 1.6;
                margin-bottom: 1.5rem;
            " {
                (message)
            }

            // Actions
            @if let Some(url) = retry_url {
                div style="display: flex; gap: 0.75rem;" {
                    button
                        class="btn btn-primary"
                        hx-get=(url)
                        hx-target=(retry_target.unwrap_or("#content"))
                        hx-swap="innerHTML"
                    {
                        "Try Again"
                    }
                    a href="/" class="btn" style="background: white; border: 1px solid var(--gray-300);" {
                        "Go Home"
                    }
                }
            }
        }
    }
}

/// Inline error alert with optional dismiss
pub fn error_alert(message: &str, dismissible: bool) -> Markup {
    html! {
        div
            class="error-alert"
            style="
                display: flex;
                align-items: flex-start;
                gap: 0.75rem;
                padding: 1rem;
                background: #fef2f2;
                border: 1px solid #fecaca;
                border-radius: 8px;
                color: #991b1b;
            "
        {
            // Icon
            svg
                width="20"
                height="20"
                viewBox="0 0 20 20"
                fill="currentColor"
                style="flex-shrink: 0; margin-top: 0.125rem;"
            {
                path
                    fill-rule="evenodd"
                    d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                    clip-rule="evenodd"
                {}
            }

            // Message
            span style="flex: 1;" { (message) }

            // Dismiss button
            @if dismissible {
                button
                    type="button"
                    onclick="this.closest('.error-alert').remove()"
                    style="
                        background: none;
                        border: none;
                        color: #991b1b;
                        cursor: pointer;
                        padding: 0;
                        opacity: 0.7;
                    "
                    aria-label="Dismiss"
                {
                    svg width="16" height="16" viewBox="0 0 20 20" fill="currentColor" {
                        path
                            fill-rule="evenodd"
                            d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                            clip-rule="evenodd"
                        {}
                    }
                }
            }
        }
    }
}

/// Form field validation error
pub fn field_error(message: &str) -> Markup {
    html! {
        span class="field-error" style="
            display: block;
            color: #dc2626;
            font-size: 0.875rem;
            margin-top: 0.25rem;
        " {
            (message)
        }
    }
}

/// Not found (404) error state
pub fn not_found_state(entity: &str) -> Markup {
    error_state(
        ErrorVariant::NotFound,
        &format!("{} Not Found", entity),
        &format!(
            "The {} you're looking for doesn't exist or has been removed.",
            entity.to_lowercase()
        ),
        None,
        None,
    )
}

/// Server error state with retry
pub fn server_error_state(retry_url: &str, retry_target: &str) -> Markup {
    error_state(
        ErrorVariant::ServerError,
        "Something Went Wrong",
        "We're having trouble processing your request. Please try again in a moment.",
        Some(retry_url),
        Some(retry_target),
    )
}

/// Network error state with retry
pub fn network_error_state(retry_url: &str, retry_target: &str) -> Markup {
    error_state(
        ErrorVariant::NetworkError,
        "Connection Error",
        "Unable to connect to the server. Please check your internet connection and try again.",
        Some(retry_url),
        Some(retry_target),
    )
}
