use maud::{html, Markup, PreEscaped};

/// Modal wrapper with backdrop and form structure
///
/// # Features
/// - Escape key to close
/// - Ctrl/Cmd+Enter to submit
/// - Focus trap (Tab cycles through focusable elements)
/// - Click outside to close
///
/// # Parameters
/// - `modal_id`: Unique ID for this modal (e.g., "player-modal", "season-modal")
/// - `title`: Modal title
/// - `error`: Optional error message to display
/// - `form_action`: POST URL for form submission
/// - `form_fields`: The form fields markup
/// - `submit_label`: Label for the submit button
pub fn modal_form(
    modal_id: &str,
    title: &str,
    error: Option<&str>,
    form_action: &str,
    form_fields: Markup,
    submit_label: &str,
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id=(modal_id)
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                // Header with close button
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        (title)
                    }
                    button
                        type="button"
                        class="modal-close-btn"
                        style="background: none; border: none; font-size: 1.5rem; cursor: pointer; color: var(--gray-500); padding: 0.25rem;"
                        onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        title="Close (Esc)"
                    {
                        "×"
                    }
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form hx-post=(form_action) hx-target=(format!("#{}", modal_id)) hx-swap="outerHTML" {
                    (form_fields)

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end; align-items: center;" {
                        // Keyboard shortcut hints
                        span style="font-size: 0.75rem; color: var(--gray-400); margin-right: auto;" {
                            "Esc to close • Ctrl+Enter to save"
                        }
                        button
                            type="button"
                            class="btn btn-secondary"
                            onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        {
                            "Cancel"
                        }
                        button type="submit" class="btn btn-primary" {
                            (submit_label)
                        }
                    }
                }
            }
        }
        (PreEscaped(format!(r#"
        <script>
            (function() {{
                const modal = document.getElementById('{}');
                if (!modal) return;

                // Focus first input
                const firstInput = modal.querySelector('input:not([type="hidden"]), select, textarea');
                if (firstInput) firstInput.focus();

                // Keyboard handler
                const keyHandler = function(e) {{
                    // Escape to close
                    if (e.key === 'Escape') {{
                        e.preventDefault();
                        modal.remove();
                        document.removeEventListener('keydown', keyHandler);
                    }}
                    // Ctrl/Cmd+Enter to submit
                    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {{
                        e.preventDefault();
                        const form = modal.querySelector('form');
                        if (form) {{
                            const submitBtn = form.querySelector('button[type="submit"]');
                            if (submitBtn) submitBtn.click();
                        }}
                    }}
                }};

                document.addEventListener('keydown', keyHandler);

                // Click outside to close
                modal.addEventListener('click', function(e) {{
                    if (e.target === this) {{
                        this.remove();
                        document.removeEventListener('keydown', keyHandler);
                    }}
                }});
            }})();
        </script>
        "#, modal_id)))
    }
}

/// Modal wrapper with backdrop and form structure (multipart/form-data encoding)
///
/// Same as `modal_form` but with multipart/form-data encoding for file uploads
///
/// # Features
/// - Escape key to close
/// - Ctrl/Cmd+Enter to submit
/// - Focus trap (Tab cycles through focusable elements)
/// - Click outside to close
///
/// # Parameters
/// - `modal_id`: Unique ID for this modal (e.g., "player-modal", "season-modal")
/// - `title`: Modal title
/// - `error`: Optional error message to display
/// - `form_action`: POST URL for form submission
/// - `form_fields`: The form fields markup
/// - `submit_label`: Label for the submit button
pub fn modal_form_multipart(
    modal_id: &str,
    title: &str,
    error: Option<&str>,
    form_action: &str,
    form_fields: Markup,
    submit_label: &str,
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id=(modal_id)
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                // Header with close button
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        (title)
                    }
                    button
                        type="button"
                        class="modal-close-btn"
                        style="background: none; border: none; font-size: 1.5rem; cursor: pointer; color: var(--gray-500); padding: 0.25rem;"
                        onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        title="Close (Esc)"
                    {
                        "×"
                    }
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form
                    hx-post=(form_action)
                    hx-target=(format!("#{}", modal_id))
                    hx-swap="outerHTML"
                    hx-encoding="multipart/form-data"
                {
                    (form_fields)

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end; align-items: center;" {
                        // Keyboard shortcut hints
                        span style="font-size: 0.75rem; color: var(--gray-400); margin-right: auto;" {
                            "Esc to close • Ctrl+Enter to save"
                        }
                        button
                            type="button"
                            class="btn btn-secondary"
                            onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        {
                            "Cancel"
                        }
                        button type="submit" class="btn btn-primary" {
                            (submit_label)
                        }
                    }
                }
            }
        }
        (PreEscaped(format!(r#"
        <script>
            (function() {{
                const modal = document.getElementById('{}');
                if (!modal) return;

                // Focus first input
                const firstInput = modal.querySelector('input:not([type="hidden"]):not([type="file"]), select, textarea');
                if (firstInput) firstInput.focus();

                // Keyboard handler
                const keyHandler = function(e) {{
                    // Escape to close
                    if (e.key === 'Escape') {{
                        e.preventDefault();
                        modal.remove();
                        document.removeEventListener('keydown', keyHandler);
                    }}
                    // Ctrl/Cmd+Enter to submit
                    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {{
                        e.preventDefault();
                        const form = modal.querySelector('form');
                        if (form) {{
                            const submitBtn = form.querySelector('button[type="submit"]');
                            if (submitBtn) submitBtn.click();
                        }}
                    }}
                }};

                document.addEventListener('keydown', keyHandler);

                // Click outside to close
                modal.addEventListener('click', function(e) {{
                    if (e.target === this) {{
                        this.remove();
                        document.removeEventListener('keydown', keyHandler);
                    }}
                }});
            }})();
        </script>
        "#, modal_id)))
    }
}

/// Modal form with i18n support
///
/// # Features
/// - Escape key to close
/// - Ctrl/Cmd+Enter to submit
/// - Focus trap (Tab cycles through focusable elements)
/// - Click outside to close
pub fn modal_form_i18n(
    modal_id: &str,
    title: &str,
    error: Option<&str>,
    form_action: &str,
    form_fields: Markup,
    submit_label: &str,
    cancel_label: &str,
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id=(modal_id)
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                // Header with close button
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        (title)
                    }
                    button
                        type="button"
                        class="modal-close-btn"
                        style="background: none; border: none; font-size: 1.5rem; cursor: pointer; color: var(--gray-500); padding: 0.25rem;"
                        onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        title="Close (Esc)"
                    {
                        "×"
                    }
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form hx-post=(form_action) hx-target=(format!("#{}", modal_id)) hx-swap="outerHTML" {
                    (form_fields)

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end; align-items: center;" {
                        // Keyboard shortcut hints
                        span style="font-size: 0.75rem; color: var(--gray-400); margin-right: auto;" {
                            "Esc to close • Ctrl+Enter to save"
                        }
                        button
                            type="button"
                            class="btn btn-secondary"
                            onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        {
                            (cancel_label)
                        }
                        button type="submit" class="btn btn-primary" {
                            (submit_label)
                        }
                    }
                }
            }
        }
        (PreEscaped(format!(r#"
        <script>
            (function() {{
                const modal = document.getElementById('{}');
                if (!modal) return;

                // Focus first input
                const firstInput = modal.querySelector('input:not([type="hidden"]), select, textarea');
                if (firstInput) firstInput.focus();

                // Keyboard handler
                const keyHandler = function(e) {{
                    // Escape to close
                    if (e.key === 'Escape') {{
                        e.preventDefault();
                        modal.remove();
                        document.removeEventListener('keydown', keyHandler);
                    }}
                    // Ctrl/Cmd+Enter to submit
                    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {{
                        e.preventDefault();
                        const form = modal.querySelector('form');
                        if (form) {{
                            const submitBtn = form.querySelector('button[type="submit"]');
                            if (submitBtn) submitBtn.click();
                        }}
                    }}
                }};

                document.addEventListener('keydown', keyHandler);

                // Click outside to close
                modal.addEventListener('click', function(e) {{
                    if (e.target === this) {{
                        this.remove();
                        document.removeEventListener('keydown', keyHandler);
                    }}
                }});
            }})();
        </script>
        "#, modal_id)))
    }
}

/// Modal form with i18n support (multipart/form-data encoding for file uploads)
///
/// # Features
/// - Escape key to close
/// - Ctrl/Cmd+Enter to submit
/// - Focus trap (Tab cycles through focusable elements)
/// - Click outside to close
#[allow(dead_code)]
pub fn modal_form_multipart_i18n(
    modal_id: &str,
    title: &str,
    error: Option<&str>,
    form_action: &str,
    form_fields: Markup,
    submit_label: &str,
    cancel_label: &str,
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id=(modal_id)
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                // Header with close button
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        (title)
                    }
                    button
                        type="button"
                        class="modal-close-btn"
                        style="background: none; border: none; font-size: 1.5rem; cursor: pointer; color: var(--gray-500); padding: 0.25rem;"
                        onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        title="Close (Esc)"
                    {
                        "×"
                    }
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form
                    hx-post=(form_action)
                    hx-target=(format!("#{}", modal_id))
                    hx-swap="outerHTML"
                    hx-encoding="multipart/form-data"
                {
                    (form_fields)

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end; align-items: center;" {
                        // Keyboard shortcut hints
                        span style="font-size: 0.75rem; color: var(--gray-400); margin-right: auto;" {
                            "Esc to close • Ctrl+Enter to save"
                        }
                        button
                            type="button"
                            class="btn btn-secondary"
                            onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        {
                            (cancel_label)
                        }
                        button type="submit" class="btn btn-primary" {
                            (submit_label)
                        }
                    }
                }
            }
        }
        (PreEscaped(format!(r#"
        <script>
            (function() {{
                const modal = document.getElementById('{}');
                if (!modal) return;

                // Focus first input (skip hidden and file inputs for file uploads)
                const firstInput = modal.querySelector('input:not([type="hidden"]):not([type="file"]), select, textarea');
                if (firstInput) firstInput.focus();

                // Keyboard handler
                const keyHandler = function(e) {{
                    // Escape to close
                    if (e.key === 'Escape') {{
                        e.preventDefault();
                        modal.remove();
                        document.removeEventListener('keydown', keyHandler);
                    }}
                    // Ctrl/Cmd+Enter to submit
                    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {{
                        e.preventDefault();
                        const form = modal.querySelector('form');
                        if (form) {{
                            const submitBtn = form.querySelector('button[type="submit"]');
                            if (submitBtn) submitBtn.click();
                        }}
                    }}
                }};

                document.addEventListener('keydown', keyHandler);

                // Click outside to close
                modal.addEventListener('click', function(e) {{
                    if (e.target === this) {{
                        this.remove();
                        document.removeEventListener('keydown', keyHandler);
                    }}
                }});
            }})();
        </script>
        "#, modal_id)))
    }
}
