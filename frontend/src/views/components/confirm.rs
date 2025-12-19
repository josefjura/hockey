use maud::{html, Markup};

/// Confirmation dialog variants
pub enum ConfirmVariant {
    Danger,
    Warning,
    Info,
}

impl ConfirmVariant {
    fn as_str(&self) -> &'static str {
        match self {
            ConfirmVariant::Danger => "danger",
            ConfirmVariant::Warning => "warning",
            ConfirmVariant::Info => "info",
        }
    }
}

/// Generate the hx-confirm-custom attribute value for HTMX integration
///
/// Use this to create the JSON string needed for the custom confirmation dialog.
///
/// # Arguments
/// - `title`: Dialog title
/// - `message`: Confirmation message
/// - `variant`: Dialog variant (danger, warning, info)
/// - `confirm_text`: Optional confirm button text (default: "Confirm")
/// - `cancel_text`: Optional cancel button text (default: "Cancel")
///
/// # Example
/// ```rust
/// button
///     hx-post="/delete/123"
///     hx-confirm-custom=(confirm_attrs("Delete Item", "Are you sure?", ConfirmVariant::Danger, None, None))
/// { "Delete" }
/// ```
pub fn confirm_attrs(
    title: &str,
    message: &str,
    variant: ConfirmVariant,
    confirm_text: Option<&str>,
    cancel_text: Option<&str>,
) -> String {
    let mut parts = vec![
        format!(r#""title":"{}""#, escape_json(title)),
        format!(r#""message":"{}""#, escape_json(message)),
        format!(r#""variant":"{}""#, variant.as_str()),
    ];

    if let Some(text) = confirm_text {
        parts.push(format!(r#""confirmText":"{}""#, escape_json(text)));
    }

    if let Some(text) = cancel_text {
        parts.push(format!(r#""cancelText":"{}""#, escape_json(text)));
    }

    format!("{{{}}}", parts.join(","))
}

/// Helper to escape JSON string values
fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Delete confirmation button with standard styling
///
/// Creates a delete button with HTMX attributes and custom confirmation dialog
pub fn delete_button(
    url: &str,
    target: &str,
    entity_name: &str,
    label: &str,
) -> Markup {
    let confirm = confirm_attrs(
        &format!("Delete {}", entity_name),
        &format!("Are you sure you want to delete this {}? This action cannot be undone.", entity_name.to_lowercase()),
        ConfirmVariant::Danger,
        Some("Delete"),
        Some("Cancel"),
    );

    html! {
        button
            class="btn btn-sm btn-danger"
            hx-post=(url)
            hx-target=(target)
            hx-swap="outerHTML"
            hx-confirm-custom=(confirm)
        {
            (label)
        }
    }
}

/// Danger action button with confirmation
pub fn danger_action_button(
    url: &str,
    target: &str,
    button_label: &str,
    confirm_title: &str,
    confirm_message: &str,
) -> Markup {
    let confirm = confirm_attrs(
        confirm_title,
        confirm_message,
        ConfirmVariant::Danger,
        Some("Confirm"),
        Some("Cancel"),
    );

    html! {
        button
            class="btn btn-danger"
            hx-post=(url)
            hx-target=(target)
            hx-swap="outerHTML"
            hx-confirm-custom=(confirm)
        {
            (button_label)
        }
    }
}

/// Warning action button with confirmation
pub fn warning_action_button(
    url: &str,
    target: &str,
    button_label: &str,
    confirm_title: &str,
    confirm_message: &str,
) -> Markup {
    let confirm = confirm_attrs(
        confirm_title,
        confirm_message,
        ConfirmVariant::Warning,
        Some("Proceed"),
        Some("Cancel"),
    );

    html! {
        button
            class="btn btn-warning"
            hx-post=(url)
            hx-target=(target)
            hx-swap="outerHTML"
            hx-confirm-custom=(confirm)
        {
            (button_label)
        }
    }
}
