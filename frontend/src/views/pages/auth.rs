use maud::{html, Markup};

use crate::views::layout::auth_layout;

pub fn login_page(error: Option<String>, csrf_token: &str) -> Markup {
    auth_layout("Sign In", html! {
        div.card {
            h1 style="margin-bottom: 1.5rem; font-size: 1.875rem; font-weight: 700;" {
                "Sign In"
            }

            @if let Some(err) = error {
                div.error {
                    (err)
                }
            }

            form method="POST" action="/auth/login" {
                input type="hidden" name="csrf_token" value=(csrf_token);

                div.form-group {
                    label for="email" { "Email" }
                    input
                        type="email"
                        id="email"
                        name="email"
                        required
                        autofocus
                        placeholder="your@email.com";
                }

                div.form-group {
                    label for="password" { "Password" }
                    input
                        type="password"
                        id="password"
                        name="password"
                        required
                        placeholder="Enter your password";
                }

                button type="submit" style="width: 100%; margin-top: 0.5rem;" {
                    "Sign In"
                }
            }
        }
    })
}
