use dioxus::prelude::*;

use crate::root::Route;
use crate::types::repos::RepoDto;

#[component]
pub fn RepoRow(repo: RepoDto) -> Element {
    let RepoDto {
        id,
        stars,
        last_fetched_at,
        tags,
        full_name,
        ..
    } = repo;

    let (owner, name) = id.split_once('/').unwrap_or(("", &id));
    let display_name = full_name.as_deref().unwrap_or(&id);
    let github_url = if owner.is_empty() {
        format!("https://github.com/{id}")
    } else {
        format!("https://github.com/{owner}/{name}")
    };

    let route = if owner.is_empty() {
        Route::Home {}
    } else {
        Route::RepoDetail {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    };

    rsx! {
        article { class: "flex gap-4 rounded-xl border border-primary-6 bg-primary-2 px-5 py-4",
            div { class: "shrink-0",
                div { class: "flex h-12 w-12 items-center justify-center rounded-full border border-primary-6 bg-primary-1 text-lg font-semibold text-secondary-4",
                    {name.chars().next().unwrap_or('R').to_ascii_uppercase().to_string()}
                }
            }

            div { class: "min-w-0 flex-1 space-y-2",
                div { class: "flex items-center gap-2",
                    h3 { class: "truncate text-base font-semibold text-secondary-4", "{display_name}" }
                    if !tags.is_empty() {
                        div { class: "hidden items-center gap-1 sm:flex",
                            for tag in tags.iter().take(3) {
                                span { class: "rounded-md border border-primary-6 bg-primary-1 px-2 py-0.5 text-xs text-secondary-5",
                                    "{tag.label}"
                                }
                            }
                        }
                    }
                }

                div { class: "text-sm text-secondary-5", "Source: {github_url}" }
                div { class: "text-xs text-secondary-5",
                    if let Some(last) = last_fetched_at {
                        "Updated: {last}"
                    } else {
                        "Updated: -"
                    }
                }
            }

            div { class: "shrink-0 space-y-2 text-right",
                div { class: "text-sm font-semibold text-secondary-4", "{stars} stars" }
                Link {
                    class: "text-sm text-secondary-5 hover:text-secondary-4 hover:underline",
                    to: route,
                    "View"
                }
            }
        }
    }
}
