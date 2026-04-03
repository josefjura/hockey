use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::views::layout::auth_layout;

pub fn login_page(t: &TranslationContext, error: Option<String>) -> Markup {
    auth_layout(
        &t.messages.signin_title().to_string(),
        html! {
            div.card {
                h1 style="margin-bottom: 1.5rem; font-size: 1.875rem; font-weight: 700;" {
                    (t.messages.signin_title())
                }

                @if let Some(err) = error {
                    div.error {
                        (err)
                    }
                }

                form method="POST" action="/auth/login" {

                    div.form-group {
                        label for="email" { (t.messages.signin_email()) }
                        input
                            type="email"
                            id="email"
                            name="email"
                            required
                            autofocus
                            placeholder="your@email.com";
                    }

                    div.form-group {
                        label for="password" { (t.messages.signin_password()) }
                        input
                            type="password"
                            id="password"
                            name="password"
                            required;
                    }

                    button type="submit" style="width: 100%; margin-top: 0.5rem;" {
                        (t.messages.signin_button())
                    }
                }
            }
        },
    )
}
