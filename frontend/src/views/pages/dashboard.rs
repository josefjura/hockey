use maud::{html, Markup};

pub fn dashboard_page() -> Markup {
    html! {
        div class="card" {
            h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 1rem;" {
                "Dashboard"
            }
            p style="color: var(--gray-600); margin-bottom: 2rem;" {
                "Welcome to the Hockey Management Application. This is the main dashboard where you can see an overview of your hockey management data."
            }

            div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1.5rem; margin-top: 2rem;" {
                (stat_card("Teams", "0", "🏒"))
                (stat_card("Players", "0", "👤"))
                (stat_card("Events", "0", "🏆"))
                (stat_card("Seasons", "0", "📅"))
            }

            div style="margin-top: 3rem;" {
                div class="info" {
                    strong { "Getting Started: " }
                    "Use the sidebar navigation to manage teams, players, events, seasons, and matches."
                }
            }
        }
    }
}

fn stat_card(title: &str, value: &str, icon: &str) -> Markup {
    html! {
        div style="
            background: linear-gradient(135deg, var(--primary-color) 0%, var(--primary-dark) 100%);
            padding: 1.5rem;
            border-radius: 12px;
            color: white;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        " {
            div style="display: flex; justify-content: space-between; align-items: start; margin-bottom: 1rem;" {
                div style="font-size: 2rem;" { (icon) }
                div style="font-size: 2.5rem; font-weight: 700; line-height: 1;" { (value) }
            }
            div style="font-size: 0.875rem; opacity: 0.9; font-weight: 500;" { (title) }
        }
    }
}
