use crate::i18n::TranslationContext;
use maud::{html, Markup};

pub fn management_page(t: &TranslationContext) -> Markup {
    html! {
        div class="container mx-auto px-4 py-8" {
            // Page header
            div class="mb-8" {
                h1 class="text-3xl font-bold text-gray-900 mb-2" {
                    (t.messages.management_title())
                }
                p class="text-gray-600" {
                    (t.messages.management_description())
                }
            }

            // Management sections grid
            div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6" {
                // Countries card - active
                a href="/countries" class="block p-6 bg-white rounded-lg border-2 border-gray-200 hover:border-blue-500 hover:shadow-lg transition-all duration-200" {
                    div class="flex items-center mb-4" {
                        div class="text-4xl mr-4" { "🌍" }
                        h2 class="text-xl font-semibold text-gray-900" {
                            (t.messages.management_countries_title())
                        }
                    }
                    p class="text-gray-600 mb-4" {
                        "Manage country data, IIHF membership, and availability"
                    }
                    div class="flex items-center text-blue-600 font-medium" {
                        span { "Manage countries" }
                        span class="ml-2" { "→" }
                    }
                }

                // Future: Users card (placeholder)
                div class="p-6 bg-gray-50 rounded-lg border-2 border-gray-200 opacity-60 cursor-not-allowed" {
                    div class="flex items-center mb-4" {
                        div class="text-4xl mr-4" { "👥" }
                        h2 class="text-xl font-semibold text-gray-600" {
                            "Users"
                        }
                    }
                    p class="text-gray-500 mb-4" {
                        "Manage user accounts and permissions"
                    }
                    span class="inline-block px-3 py-1 bg-gray-300 text-gray-600 rounded-full text-sm font-medium" {
                        "Coming soon"
                    }
                }

                // Future: System settings (placeholder)
                div class="p-6 bg-gray-50 rounded-lg border-2 border-gray-200 opacity-60 cursor-not-allowed" {
                    div class="flex items-center mb-4" {
                        div class="text-4xl mr-4" { "⚙️" }
                        h2 class="text-xl font-semibold text-gray-600" {
                            "Settings"
                        }
                    }
                    p class="text-gray-500 mb-4" {
                        "Configure system preferences and options"
                    }
                    span class="inline-block px-3 py-1 bg-gray-300 text-gray-600 rounded-full text-sm font-medium" {
                        "Coming soon"
                    }
                }
            }
        }
    }
}
