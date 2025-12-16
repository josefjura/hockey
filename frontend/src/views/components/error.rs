use maud::{html, Markup};

pub fn error_message(message: &str) -> Markup {
    html! {
        div class="error" style="padding: 1rem; margin: 1rem 0;" {
            (message)
        }
    }
}
