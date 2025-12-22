use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::events::{EventDetailEntity, EventEntity, SeasonEntity};
use crate::views::components::confirm::{confirm_attrs, ConfirmVariant};

/// Event detail page with seasons list
pub fn event_detail_page(t: &TranslationContext, detail: &EventDetailEntity) -> Markup {
    let event = &detail.event_info;

    html! {
        div class="card" {
            // Header with back button and action buttons
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                div style="display: flex; align-items: center; gap: 1rem;" {
                    a
                        href="/events"
                        class="btn btn-secondary"
                    {
                        "← Back to Events"
                    }
                    h1 style="font-size: 2rem; font-weight: 700; margin: 0;" {
                        (event.name)
                    }
                }
                div style="display: flex; gap: 0.5rem;" {
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/events/{}/edit", event.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        (t.messages.events_edit())
                    }
                    button
                        class="btn btn-danger"
                        hx-post=(format!("/events/{}/delete", event.id))
                        hx-confirm-custom=(confirm_attrs(
                            &t.messages.events_delete().to_string(),
                            &t.messages.events_confirm_delete().to_string(),
                            ConfirmVariant::Danger,
                            Some(&t.messages.common_delete().to_string()),
                            Some(&t.messages.common_cancel().to_string())
                        ))
                    {
                        (t.messages.events_delete())
                    }
                }
            }

            // Event Info Card
            (event_info_card(t, event))

            // Seasons Section
            div style="margin-top: 2rem;" {
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        (t.messages.seasons_title())
                    }
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/events/{}/seasons/new", event.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        (t.messages.seasons_create())
                    }
                }

                @if detail.seasons.is_empty() {
                    (empty_seasons_state(t))
                } @else {
                    (seasons_grid(t, &detail.seasons))
                }
            }

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Event info card with country
fn event_info_card(t: &TranslationContext, event: &EventEntity) -> Markup {
    html! {
        div style="padding: 1.5rem; background: var(--gray-50); border-radius: 8px;" {
            div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;" {
                div {
                    div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                        (t.messages.events_name_label())
                    }
                    div style="font-weight: 600;" {
                        (event.name)
                    }
                }
                div {
                    div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                        (t.messages.events_host_country())
                    }
                    @if let Some(country_name) = &event.country_name {
                        div style="display: flex; align-items: center; gap: 0.5rem; font-weight: 600;" {
                            @if let Some(iso2) = &event.country_iso2_code {
                                img
                                    src=(format!("https://flagcdn.com/w40/{}.png", iso2.to_lowercase()))
                                    alt=(country_name)
                                    style="width: 24px; height: 18px; object-fit: cover; border: 1px solid var(--gray-300); border-radius: 2px;"
                                    onerror="this.style.display='none'";
                            }
                            (country_name)
                        }
                    } @else {
                        div style="color: var(--gray-400); font-style: italic; font-weight: 600;" {
                            (t.messages.common_no_country())
                        }
                    }
                }
            }
        }
    }
}

/// Empty state when no seasons exist
fn empty_seasons_state(t: &TranslationContext) -> Markup {
    html! {
        div style="padding: 3rem; text-align: center; color: var(--gray-500); background: var(--gray-50); border-radius: 8px;" {
            p style="font-size: 1.125rem; font-weight: 600; margin-bottom: 0.5rem;" {
                (t.messages.seasons_empty_title())
            }
            p {
                "Create the first season for this event to get started."
            }
        }
    }
}

/// Grid of seasons
fn seasons_grid(t: &TranslationContext, seasons: &[SeasonEntity]) -> Markup {
    html! {
        div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 1.5rem;" {
            @for season in seasons {
                (season_card(t, season))
            }
        }
    }
}

/// Individual season card
fn season_card(_t: &TranslationContext, season: &SeasonEntity) -> Markup {
    let _display_name = season
        .display_name
        .as_ref()
        .map(|s| format!("{} - {}", season.year, s))
        .unwrap_or_else(|| season.year.to_string());

    html! {
        div style="padding: 1.5rem; background: white; border: 1px solid var(--gray-200); border-radius: 8px; transition: box-shadow 0.2s; cursor: pointer;" {
            div style="display: flex; justify-content: space-between; align-items: start; margin-bottom: 1rem;" {
                div {
                    div style="font-size: 1.25rem; font-weight: 700; margin-bottom: 0.25rem;" {
                        (season.year)
                    }
                    @if let Some(display) = &season.display_name {
                        div style="color: var(--gray-600); font-size: 0.875rem;" {
                            (display)
                        }
                    }
                }
            }

            div style="display: flex; gap: 0.5rem; margin-top: 1rem;" {
                a
                    href=(format!("/seasons/{}", season.id))
                    class="btn btn-sm btn-primary"
                    style="flex: 1;"
                {
                    "View Details"
                }
            }
        }
    }
}
