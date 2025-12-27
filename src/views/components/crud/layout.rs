use maud::{html, Markup};

/// Page header with title, description, and create button
pub fn page_header(title: &str, description: &str, create_url: &str, create_label: &str) -> Markup {
    html! {
        div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
            div {
                h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem;" {
                    (title)
                }
                p style="color: var(--gray-600);" {
                    (description)
                }
            }
            button
                class="btn btn-primary"
                hx-get=(create_url)
                hx-target="#modal-container"
                hx-swap="innerHTML"
            {
                (create_label)
            }
        }
    }
}

/// Page header with i18n support
pub fn page_header_i18n(
    title: &str,
    description: &str,
    create_url: &str,
    create_label: &str,
) -> Markup {
    html! {
        div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
            div {
                h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem;" {
                    (title)
                }
                p style="color: var(--gray-600);" {
                    (description)
                }
            }
            button
                class="btn btn-primary"
                hx-get=(create_url)
                hx-target="#modal-container"
                hx-swap="innerHTML"
            {
                (create_label)
            }
        }
    }
}
