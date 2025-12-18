use maud::{html, Markup};

/// Countries management page
pub fn countries_page() -> Markup {
    html! {
        div class="card" {
            // Header
            div style="margin-bottom: 1.5rem;" {
                h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem;" {
                    "Countries"
                }
                p style="color: var(--gray-600);" {
                    "Manage country data, IIHF membership, and availability. Use the table below to search, sort, and enable/disable countries."
                }
            }

            // Countries table web component
            countries-table
                api-endpoint="/api/countries"
                page-size="20" {}

            // No-JS fallback
            noscript {
                div style="padding: 2rem; text-align: center; background-color: #fef3c7; border: 1px solid #f59e0b; border-radius: 8px; margin-top: 1rem;" {
                    p style="font-weight: 600; color: #92400e;" {
                        "JavaScript Required"
                    }
                    p style="color: #92400e; margin-top: 0.5rem;" {
                        "The countries management table requires JavaScript to be enabled. Please enable JavaScript in your browser to view and manage countries."
                    }
                }
            }
        }
    }
}
