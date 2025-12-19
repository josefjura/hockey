use maud::{html, Markup, PreEscaped};

/// Form input types
pub enum InputType {
    Text,
    Email,
    Password,
    Number,
    Date,
    Url,
    Tel,
    Search,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Date => "date",
            InputType::Url => "url",
            InputType::Tel => "tel",
            InputType::Search => "search",
        }
    }
}

/// Form field with label, input, and optional error/help text
///
/// # Arguments
/// - `name`: Field name attribute
/// - `label`: Label text
/// - `input_type`: Type of input
/// - `value`: Current value
/// - `placeholder`: Placeholder text
/// - `required`: Whether field is required
/// - `error`: Optional validation error message
/// - `help`: Optional help text
pub fn form_field(
    name: &str,
    label: &str,
    input_type: InputType,
    value: Option<&str>,
    placeholder: Option<&str>,
    required: bool,
    error: Option<&str>,
    help: Option<&str>,
) -> Markup {
    let has_error = error.is_some();
    let input_class = if has_error { "input-error" } else { "" };

    html! {
        div class="form-group" {
            label for=(name) class="form-label" {
                (label)
                @if required {
                    span class="required-indicator" { " *" }
                }
            }
            input
                type=(input_type.as_str())
                id=(name)
                name=(name)
                value=[value]
                placeholder=[placeholder]
                required[required]
                class=(input_class)
                aria-invalid=[has_error.then_some("true")]
                aria-describedby=[error.map(|_| format!("{}-error", name))]
            {}
            @if let Some(err) = error {
                span id=(format!("{}-error", name)) class="field-error" {
                    (err)
                }
            }
            @if let Some(h) = help {
                span class="field-help" {
                    (h)
                }
            }
        }
    }
}

/// Textarea field with label and optional error/help
pub fn form_textarea(
    name: &str,
    label: &str,
    value: Option<&str>,
    placeholder: Option<&str>,
    rows: u8,
    required: bool,
    error: Option<&str>,
    help: Option<&str>,
) -> Markup {
    let has_error = error.is_some();
    let input_class = if has_error { "input-error" } else { "" };

    html! {
        div class="form-group" {
            label for=(name) class="form-label" {
                (label)
                @if required {
                    span class="required-indicator" { " *" }
                }
            }
            textarea
                id=(name)
                name=(name)
                placeholder=[placeholder]
                rows=(rows)
                required[required]
                class=(input_class)
                aria-invalid=[has_error.then_some("true")]
                aria-describedby=[error.map(|_| format!("{}-error", name))]
            {
                @if let Some(v) = value {
                    (v)
                }
            }
            @if let Some(err) = error {
                span id=(format!("{}-error", name)) class="field-error" {
                    (err)
                }
            }
            @if let Some(h) = help {
                span class="field-help" {
                    (h)
                }
            }
        }
    }
}

/// Select field with options
pub fn form_select<T: std::fmt::Display>(
    name: &str,
    label: &str,
    options: &[(T, &str)], // (value, label)
    selected: Option<&str>,
    placeholder: Option<&str>,
    required: bool,
    error: Option<&str>,
) -> Markup {
    let has_error = error.is_some();
    let input_class = if has_error { "input-error" } else { "" };

    html! {
        div class="form-group" {
            label for=(name) class="form-label" {
                (label)
                @if required {
                    span class="required-indicator" { " *" }
                }
            }
            select
                id=(name)
                name=(name)
                required[required]
                class=(input_class)
                aria-invalid=[has_error.then_some("true")]
            {
                @if let Some(ph) = placeholder {
                    option value="" disabled selected[selected.is_none()] {
                        (ph)
                    }
                }
                @for (value, option_label) in options {
                    option
                        value=(value)
                        selected[selected == Some(&value.to_string())]
                    {
                        (option_label)
                    }
                }
            }
            @if let Some(err) = error {
                span id=(format!("{}-error", name)) class="field-error" {
                    (err)
                }
            }
        }
    }
}

/// Checkbox field
pub fn form_checkbox(
    name: &str,
    label: &str,
    checked: bool,
    help: Option<&str>,
) -> Markup {
    html! {
        div class="form-group form-checkbox" {
            label class="checkbox-label" {
                input
                    type="checkbox"
                    id=(name)
                    name=(name)
                    checked[checked]
                {}
                span class="checkbox-text" { (label) }
            }
            @if let Some(h) = help {
                span class="field-help" {
                    (h)
                }
            }
        }
    }
}

/// Form actions row (buttons)
pub fn form_actions(primary_label: &str, cancel_onclick: Option<&str>) -> Markup {
    html! {
        div class="form-actions" {
            @if let Some(onclick) = cancel_onclick {
                button
                    type="button"
                    class="btn btn-secondary"
                    onclick=(onclick)
                {
                    "Cancel"
                }
            }
            button type="submit" class="btn btn-primary" {
                (primary_label)
            }
        }
    }
}

/// Form row for horizontal layout (multiple fields in a row)
pub fn form_row(children: Markup) -> Markup {
    html! {
        div class="form-row" {
            (children)
        }
    }
}

/// Form section with optional title
pub fn form_section(title: Option<&str>, children: Markup) -> Markup {
    html! {
        div class="form-section" {
            @if let Some(t) = title {
                h3 class="form-section-title" { (t) }
            }
            (children)
        }
    }
}

/// CSS for enhanced form styling
///
/// Include this in your layout for consistent form styling
pub fn form_styles() -> Markup {
    html! {
        (PreEscaped(r#"
        <style>
            /* Form Group */
            .form-group {
                margin-bottom: 1.25rem;
            }

            /* Form Label */
            .form-label {
                display: block;
                margin-bottom: 0.5rem;
                font-weight: 500;
                font-size: 0.875rem;
                color: var(--gray-700);
            }

            .required-indicator {
                color: #ef4444;
            }

            /* Input Styles */
            .form-group input[type="text"],
            .form-group input[type="email"],
            .form-group input[type="password"],
            .form-group input[type="number"],
            .form-group input[type="date"],
            .form-group input[type="url"],
            .form-group input[type="tel"],
            .form-group input[type="search"],
            .form-group select,
            .form-group textarea {
                width: 100%;
                padding: 0.625rem 0.875rem;
                border: 1px solid var(--gray-300);
                border-radius: 8px;
                font-size: 0.9375rem;
                line-height: 1.5;
                transition: border-color 0.15s, box-shadow 0.15s;
                background-color: white;
            }

            .form-group input:hover,
            .form-group select:hover,
            .form-group textarea:hover {
                border-color: var(--gray-400);
            }

            .form-group input:focus,
            .form-group select:focus,
            .form-group textarea:focus {
                outline: none;
                border-color: var(--primary-color);
                box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
            }

            /* Error State */
            .form-group input.input-error,
            .form-group select.input-error,
            .form-group textarea.input-error {
                border-color: #ef4444;
            }

            .form-group input.input-error:focus,
            .form-group select.input-error:focus,
            .form-group textarea.input-error:focus {
                box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.15);
            }

            .field-error {
                display: block;
                margin-top: 0.375rem;
                font-size: 0.8125rem;
                color: #dc2626;
            }

            .field-help {
                display: block;
                margin-top: 0.375rem;
                font-size: 0.8125rem;
                color: var(--gray-500);
            }

            /* Disabled State */
            .form-group input:disabled,
            .form-group select:disabled,
            .form-group textarea:disabled {
                background-color: var(--gray-100);
                color: var(--gray-500);
                cursor: not-allowed;
            }

            /* Checkbox */
            .form-checkbox {
                display: flex;
                flex-direction: column;
            }

            .checkbox-label {
                display: flex;
                align-items: center;
                gap: 0.5rem;
                cursor: pointer;
                font-weight: 400;
            }

            .checkbox-label input[type="checkbox"] {
                width: 1.125rem;
                height: 1.125rem;
                margin: 0;
                cursor: pointer;
                accent-color: var(--primary-color);
            }

            .checkbox-text {
                font-size: 0.9375rem;
                color: var(--gray-700);
            }

            /* Form Actions */
            .form-actions {
                display: flex;
                gap: 0.75rem;
                justify-content: flex-end;
                margin-top: 1.5rem;
                padding-top: 1.5rem;
                border-top: 1px solid var(--gray-200);
            }

            /* Form Row */
            .form-row {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                gap: 1rem;
            }

            .form-row .form-group {
                margin-bottom: 0;
            }

            /* Form Section */
            .form-section {
                margin-bottom: 2rem;
            }

            .form-section-title {
                font-size: 1rem;
                font-weight: 600;
                color: var(--gray-800);
                margin-bottom: 1rem;
                padding-bottom: 0.5rem;
                border-bottom: 1px solid var(--gray-200);
            }

            /* Button Variants */
            .btn {
                display: inline-flex;
                align-items: center;
                justify-content: center;
                gap: 0.5rem;
                padding: 0.625rem 1.25rem;
                font-size: 0.9375rem;
                font-weight: 500;
                border-radius: 8px;
                cursor: pointer;
                transition: all 0.15s;
                border: none;
            }

            .btn-primary {
                background: var(--primary-color);
                color: white;
            }

            .btn-primary:hover {
                background: var(--primary-hover);
            }

            .btn-primary:active {
                background: var(--primary-dark);
            }

            .btn-secondary {
                background: white;
                color: var(--gray-700);
                border: 1px solid var(--gray-300);
            }

            .btn-secondary:hover {
                background: var(--gray-50);
                border-color: var(--gray-400);
            }

            .btn-danger {
                background: #ef4444;
                color: white;
            }

            .btn-danger:hover {
                background: #dc2626;
            }

            .btn-warning {
                background: #f59e0b;
                color: white;
            }

            .btn-warning:hover {
                background: #d97706;
            }

            .btn-sm {
                padding: 0.375rem 0.75rem;
                font-size: 0.8125rem;
            }

            .btn:disabled {
                opacity: 0.6;
                cursor: not-allowed;
            }

            .btn:focus {
                outline: none;
                box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.3);
            }

            /* File Input */
            .form-group input[type="file"] {
                padding: 0.5rem;
                border: 1px dashed var(--gray-300);
                border-radius: 8px;
                background: var(--gray-50);
                cursor: pointer;
            }

            .form-group input[type="file"]:hover {
                border-color: var(--primary-color);
                background: rgba(59, 130, 246, 0.05);
            }
        </style>
        "#))
    }
}
