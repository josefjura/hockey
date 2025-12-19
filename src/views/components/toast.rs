#![allow(dead_code)]

use maud::{html, Markup, PreEscaped};

/// Toast notification variants
pub enum ToastVariant {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastVariant {
    fn as_str(&self) -> &'static str {
        match self {
            ToastVariant::Success => "success",
            ToastVariant::Error => "error",
            ToastVariant::Warning => "warning",
            ToastVariant::Info => "info",
        }
    }
}

/// JavaScript to show a toast notification
///
/// Returns a script tag that will show a toast when executed.
/// Use this in HTMX responses to show feedback after actions.
///
/// # Arguments
/// - `message`: The message to display
/// - `variant`: Toast type (success, error, warning, info)
pub fn show_toast(message: &str, variant: ToastVariant) -> Markup {
    let escaped_message = message.replace('\'', "\\'").replace('\n', "\\n");
    html! {
        (PreEscaped(format!(
            r#"<script>
                (function() {{
                    const container = document.querySelector('hockey-toast-container');
                    if (container) {{
                        container.{}('{}');
                    }}
                }})();
            </script>"#,
            variant.as_str(),
            escaped_message
        )))
    }
}

/// Show a success toast
pub fn toast_success(message: &str) -> Markup {
    show_toast(message, ToastVariant::Success)
}

/// Show an error toast
#[allow(dead_code)]
pub fn toast_error(message: &str) -> Markup {
    show_toast(message, ToastVariant::Error)
}

/// Show a warning toast
#[allow(dead_code)]
pub fn toast_warning(message: &str) -> Markup {
    show_toast(message, ToastVariant::Warning)
}

/// Show an info toast
#[allow(dead_code)]
pub fn toast_info(message: &str) -> Markup {
    show_toast(message, ToastVariant::Info)
}

/// JavaScript for HTMX event-based toast notifications
///
/// This adds event listeners to automatically show toasts based on
/// custom HTMX response headers. Include this once in your layout.
///
/// Headers supported:
/// - `HX-Toast-Success`: Success message
/// - `HX-Toast-Error`: Error message
/// - `HX-Toast-Warning`: Warning message
/// - `HX-Toast-Info`: Info message
pub fn htmx_toast_event_handler() -> Markup {
    html! {
        (PreEscaped(r#"
        <script>
            document.addEventListener('htmx:afterRequest', function(evt) {
                const container = document.querySelector('hockey-toast-container');
                if (!container) return;

                const xhr = evt.detail.xhr;
                if (!xhr) return;

                const successMsg = xhr.getResponseHeader('HX-Toast-Success');
                const errorMsg = xhr.getResponseHeader('HX-Toast-Error');
                const warningMsg = xhr.getResponseHeader('HX-Toast-Warning');
                const infoMsg = xhr.getResponseHeader('HX-Toast-Info');

                if (successMsg) container.success(successMsg);
                if (errorMsg) container.error(errorMsg);
                if (warningMsg) container.warning(warningMsg);
                if (infoMsg) container.info(infoMsg);
            });

            // Also handle request errors
            document.addEventListener('htmx:responseError', function(evt) {
                const container = document.querySelector('hockey-toast-container');
                if (container) {
                    container.error('An error occurred. Please try again.');
                }
            });
        </script>
        "#))
    }
}
