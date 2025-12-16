use maud::{html, Markup, DOCTYPE};

pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) " - Hockey Management" }
                // TODO: Add Tailwind CSS when we set it up
                style {
                    r#"
                    * {
                        margin: 0;
                        padding: 0;
                        box-sizing: border-box;
                    }
                    body {
                        font-family: system-ui, -apple-system, sans-serif;
                        line-height: 1.5;
                        color: #1f2937;
                        background: #f3f4f6;
                    }
                    .container {
                        max-width: 1200px;
                        margin: 0 auto;
                        padding: 2rem;
                    }
                    .card {
                        background: white;
                        border-radius: 8px;
                        padding: 2rem;
                        box-shadow: 0 1px 3px rgba(0,0,0,0.1);
                    }
                    .form-group {
                        margin-bottom: 1rem;
                    }
                    label {
                        display: block;
                        margin-bottom: 0.5rem;
                        font-weight: 500;
                    }
                    input[type="text"],
                    input[type="email"],
                    input[type="password"] {
                        width: 100%;
                        padding: 0.5rem;
                        border: 1px solid #d1d5db;
                        border-radius: 4px;
                        font-size: 1rem;
                    }
                    input[type="text"]:focus,
                    input[type="email"]:focus,
                    input[type="password"]:focus {
                        outline: none;
                        border-color: #3b82f6;
                        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
                    }
                    button {
                        background: #3b82f6;
                        color: white;
                        padding: 0.5rem 1rem;
                        border: none;
                        border-radius: 4px;
                        font-size: 1rem;
                        cursor: pointer;
                        font-weight: 500;
                    }
                    button:hover {
                        background: #2563eb;
                    }
                    .error {
                        background: #fef2f2;
                        border: 1px solid #fecaca;
                        color: #991b1b;
                        padding: 0.75rem;
                        border-radius: 4px;
                        margin-bottom: 1rem;
                    }
                    .success {
                        background: #f0fdf4;
                        border: 1px solid #bbf7d0;
                        color: #166534;
                        padding: 0.75rem;
                        border-radius: 4px;
                        margin-bottom: 1rem;
                    }
                    "#
                }
            }
            body {
                (content)
            }
        }
    }
}

pub fn auth_layout(title: &str, content: Markup) -> Markup {
    base_layout(title, html! {
        div.container style="max-width: 480px; margin-top: 4rem;" {
            (content)
        }
    })
}
