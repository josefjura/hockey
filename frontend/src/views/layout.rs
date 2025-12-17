use maud::{html, Markup, DOCTYPE};

use super::components::sidebar;
use crate::auth::Session;
use crate::i18n::{I18n, Locale};

pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) " - Hockey Management" }
                (base_styles())
                // HTMX library for dynamic HTML updates
                script src="https://unpkg.com/htmx.org@2.0.4" {}
                // Web Components
                script type="module" src="/static/js/components/country-selector.js" {}
            }
            body {
                (content)
            }
        }
    }
}

/// Admin layout with sidebar navigation
pub fn admin_layout(
    title: &str,
    session: &Session,
    current_path: &str,
    i18n: &I18n,
    locale: Locale,
    content: Markup,
) -> Markup {
    base_layout(
        title,
        html! {
            div class="app-layout" {
                (sidebar(session, current_path, i18n, locale))
                main class="main-content" {
                    div class="content-wrapper" {
                        (content)
                    }
                }
            }
        },
    )
}

fn base_styles() -> Markup {
    html! {
        style {
            r#"
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }

            :root {
                --sidebar-width: 260px;
                --primary-color: #3b82f6;
                --primary-hover: #2563eb;
                --primary-dark: #1d4ed8;
                --danger-color: #ef4444;
                --danger-hover: #dc2626;
                --gray-50: #f9fafb;
                --gray-100: #f3f4f6;
                --gray-200: #e5e7eb;
                --gray-300: #d1d5db;
                --gray-400: #9ca3af;
                --gray-500: #6b7280;
                --gray-600: #4b5563;
                --gray-700: #374151;
                --gray-800: #1f2937;
                --gray-900: #111827;
                --sidebar-bg: #1e293b;
                --sidebar-text: #cbd5e1;
                --sidebar-active: #3b82f6;
            }

            body {
                font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                line-height: 1.5;
                color: var(--gray-800);
                background: var(--gray-100);
            }

            /* App Layout */
            .app-layout {
                display: flex;
                min-height: 100vh;
            }

            /* Sidebar Styles */
            .sidebar {
                width: var(--sidebar-width);
                background: var(--sidebar-bg);
                color: var(--sidebar-text);
                display: flex;
                flex-direction: column;
                position: fixed;
                height: 100vh;
                overflow-y: auto;
                box-shadow: 2px 0 8px rgba(0, 0, 0, 0.1);
            }

            .sidebar-brand {
                padding: 1.5rem 1rem;
                border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            }

            .sidebar-brand h1 {
                font-size: 1.25rem;
                font-weight: 700;
                color: white;
            }

            .sidebar-nav {
                flex: 1;
                padding: 1rem 0;
            }

            .nav-link {
                display: flex;
                align-items: center;
                padding: 0.75rem 1rem;
                color: var(--sidebar-text);
                text-decoration: none;
                transition: all 0.2s;
                border-left: 3px solid transparent;
            }

            .nav-link:hover {
                background: rgba(255, 255, 255, 0.05);
                color: white;
            }

            .nav-link.active {
                background: rgba(59, 130, 246, 0.1);
                color: white;
                border-left-color: var(--sidebar-active);
            }

            .nav-icon {
                font-size: 1.25rem;
                margin-right: 0.75rem;
                width: 1.5rem;
                text-align: center;
            }

            .nav-label {
                font-weight: 500;
            }

            /* Sidebar Footer */
            .sidebar-footer {
                border-top: 1px solid rgba(255, 255, 255, 0.1);
                padding: 1rem;
            }

            .user-info {
                display: flex;
                align-items: center;
                gap: 0.75rem;
                padding: 0.75rem;
                background: rgba(255, 255, 255, 0.05);
                border-radius: 8px;
                margin-bottom: 1rem;
            }

            .user-avatar {
                width: 2.5rem;
                height: 2.5rem;
                background: var(--primary-color);
                color: white;
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                font-weight: 700;
                font-size: 1rem;
                flex-shrink: 0;
            }

            .user-details {
                flex: 1;
                min-width: 0;
            }

            .user-name {
                font-weight: 600;
                color: white;
                font-size: 0.875rem;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }

            .user-email {
                font-size: 0.75rem;
                color: var(--gray-400);
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }

            .locale-switcher {
                margin-bottom: 0.75rem;
            }

            .locale-label {
                display: block;
                font-size: 0.75rem;
                font-weight: 600;
                text-transform: uppercase;
                letter-spacing: 0.05em;
                margin-bottom: 0.5rem;
                color: var(--gray-400);
            }

            .locale-select {
                width: 100%;
                padding: 0.5rem;
                background: rgba(255, 255, 255, 0.05);
                border: 1px solid rgba(255, 255, 255, 0.1);
                border-radius: 6px;
                color: white;
                font-size: 0.875rem;
                cursor: pointer;
                transition: all 0.2s;
            }

            .locale-select:hover {
                background: rgba(255, 255, 255, 0.1);
            }

            .locale-select:focus {
                outline: none;
                border-color: var(--primary-color);
                box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
            }

            .locale-select option {
                background: var(--gray-800);
                color: white;
            }

            .logout-form {
                margin: 0;
            }

            .logout-button {
                width: 100%;
                display: flex;
                align-items: center;
                padding: 0.75rem;
                background: rgba(239, 68, 68, 0.1);
                color: #fca5a5;
                border: 1px solid rgba(239, 68, 68, 0.2);
                border-radius: 6px;
                font-size: 0.875rem;
                font-weight: 600;
                cursor: pointer;
                transition: all 0.2s;
            }

            .logout-button:hover {
                background: rgba(239, 68, 68, 0.2);
                color: #fecaca;
            }

            .logout-button .nav-icon {
                margin-right: 0.5rem;
            }

            /* Main Content */
            .main-content {
                flex: 1;
                margin-left: var(--sidebar-width);
                min-height: 100vh;
            }

            .content-wrapper {
                padding: 2rem;
                max-width: 1400px;
            }

            /* Common Elements */
            .container {
                max-width: 1200px;
                margin: 0 auto;
                padding: 2rem;
            }

            .card {
                background: white;
                border-radius: 8px;
                padding: 2rem;
                box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
            }

            .form-group {
                margin-bottom: 1rem;
            }

            label {
                display: block;
                margin-bottom: 0.5rem;
                font-weight: 500;
                color: var(--gray-700);
            }

            input[type="text"],
            input[type="email"],
            input[type="password"],
            input[type="number"],
            input[type="date"],
            select,
            textarea {
                width: 100%;
                padding: 0.5rem 0.75rem;
                border: 1px solid var(--gray-300);
                border-radius: 6px;
                font-size: 1rem;
                transition: all 0.2s;
                font-family: inherit;
            }

            input:focus,
            select:focus,
            textarea:focus {
                outline: none;
                border-color: var(--primary-color);
                box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
            }

            button {
                background: var(--primary-color);
                color: white;
                padding: 0.5rem 1rem;
                border: none;
                border-radius: 6px;
                font-size: 1rem;
                cursor: pointer;
                font-weight: 500;
                transition: all 0.2s;
                font-family: inherit;
            }

            button:hover {
                background: var(--primary-hover);
            }

            button:active {
                background: var(--primary-dark);
            }

            button:disabled {
                background: var(--gray-300);
                cursor: not-allowed;
            }

            .error {
                background: #fef2f2;
                border: 1px solid #fecaca;
                color: #991b1b;
                padding: 0.75rem 1rem;
                border-radius: 6px;
                margin-bottom: 1rem;
            }

            .success {
                background: #f0fdf4;
                border: 1px solid #bbf7d0;
                color: #166534;
                padding: 0.75rem 1rem;
                border-radius: 6px;
                margin-bottom: 1rem;
            }

            .warning {
                background: #fffbeb;
                border: 1px solid #fde68a;
                color: #92400e;
                padding: 0.75rem 1rem;
                border-radius: 6px;
                margin-bottom: 1rem;
            }

            .info {
                background: #eff6ff;
                border: 1px solid #bfdbfe;
                color: #1e40af;
                padding: 0.75rem 1rem;
                border-radius: 6px;
                margin-bottom: 1rem;
            }

            /* Responsive */
            @media (max-width: 768px) {
                .sidebar {
                    width: 70px;
                }

                .main-content {
                    margin-left: 70px;
                }

                .nav-label,
                .user-details,
                .locale-label,
                .logout-button span:not(.nav-icon) {
                    display: none;
                }

                .sidebar-brand h1 {
                    font-size: 1.5rem;
                    text-align: center;
                }

                .user-info {
                    justify-content: center;
                    padding: 0.5rem;
                }

                .nav-link {
                    justify-content: center;
                }
            }
            "#
        }
    }
}

pub fn auth_layout(title: &str, content: Markup) -> Markup {
    base_layout(
        title,
        html! {
            div.container style="max-width: 480px; margin-top: 4rem;" {
                (content)
            }
        },
    )
}
