use maud::{html, Markup};

/// Loading spinner for inline use
///
/// # Arguments
/// - `size`: sm, md, lg, xl
/// - `label`: Optional label text
pub fn loading_spinner(size: &str, label: Option<&str>) -> Markup {
    html! {
        hockey-loading-spinner
            size=(size)
            label=[label]
        {}
    }
}

/// Full loading state for content areas
///
/// # Arguments
/// - `label`: Loading message to display
/// - `size`: Spinner size (sm, md, lg, xl)
pub fn loading_state(label: &str, size: &str) -> Markup {
    html! {
        hockey-loading-state
            label=(label)
            size=(size)
        {}
    }
}

/// Skeleton loading state for tables
///
/// # Arguments
/// - `rows`: Number of skeleton rows to display
pub fn loading_skeleton(rows: usize) -> Markup {
    html! {
        hockey-loading-state
            variant="skeleton"
            skeletonRows=(rows)
        {}
    }
}

/// Inline loading indicator
pub fn loading_inline(label: Option<&str>) -> Markup {
    html! {
        hockey-loading-state
            variant="inline"
            size="sm"
            label=[label]
        {}
    }
}

/// HTMX loading indicator that shows during requests
///
/// This creates a loading spinner that is hidden by default
/// and shown during HTMX requests using the `htmx-indicator` class.
///
/// # Arguments
/// - `id`: Unique ID for the indicator
/// - `label`: Optional loading message
pub fn htmx_loading_indicator(id: &str, label: Option<&str>) -> Markup {
    html! {
        div
            id=(id)
            class="htmx-indicator"
            style="display: none;"
        {
            hockey-loading-spinner
                size="sm"
                layout="horizontal"
                label=[label]
            {}
        }
    }
}

/// Button with loading state support
///
/// Creates a button that shows a spinner when clicked and waiting for response.
/// Works with HTMX by using the `htmx-request` class.
///
/// # Arguments
/// - `text`: Button text
/// - `loading_text`: Text to show while loading
/// - `button_class`: CSS classes for the button
pub fn loading_button(text: &str, loading_text: &str, button_class: &str) -> Markup {
    html! {
        button class=(button_class) {
            span class="btn-text" { (text) }
            span class="btn-loading" style="display: none;" {
                hockey-loading-spinner size="sm" layout="horizontal" label=(loading_text) {}
            }
        }
    }
}

/// CSS for HTMX loading indicators
///
/// Include this once in your layout to enable loading indicator support
pub fn htmx_loading_styles() -> Markup {
    html! {
        style {
            r#"
            /* HTMX Loading Indicator Styles */
            .htmx-indicator {
                display: none;
            }
            
            .htmx-request .htmx-indicator {
                display: inline-flex !important;
            }
            
            .htmx-request.htmx-indicator {
                display: inline-flex !important;
            }
            
            /* Button loading states */
            .htmx-request .btn-text {
                display: none;
            }
            
            .htmx-request .btn-loading {
                display: inline-flex !important;
            }
            
            /* Disabled appearance during request */
            .htmx-request {
                pointer-events: none;
                opacity: 0.7;
            }
            
            /* Loading overlay for content areas */
            .loading-overlay {
                position: relative;
            }
            
            .loading-overlay::after {
                content: '';
                position: absolute;
                inset: 0;
                background: rgba(255, 255, 255, 0.8);
                display: none;
                z-index: 10;
            }
            
            .loading-overlay.htmx-request::after {
                display: block;
            }
            
            .loading-overlay .loading-spinner-overlay {
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                z-index: 11;
                display: none;
            }
            
            .loading-overlay.htmx-request .loading-spinner-overlay {
                display: block;
            }
            "#
        }
    }
}
