use dioxus::prelude::*;

use crate::root::routes::Route;
use crate::IO::repos::list_repos;
use app::prelude::Pagination;

#[component]
pub fn Home() -> Element {
    let repos = use_server_future(move || {
        list_repos(Pagination {
            limit: Some(200),
            offset: Some(0),
        })
    })?;

    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-6 space-y-6",
            div { class: "space-y-1",
                h1 { class: "text-2xl font-semibold tracking-tight", "Home" }
                p { class: "text-sm text-[color:var(--secondary-color-5)]", "Repo list" }
            }

            match repos() {
                Some(Ok(page)) => {
                    let total = page.meta.total;
                    let items = page.items;
                    rsx! {
                    div { class: "text-sm text-[color:var(--secondary-color-5)]", "total: {total}" }
                    div { class: "grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3",
                        for r in items {
                            Link {
                                key: "{r.id}",
                                class: "block rounded-xl border border-[color:var(--primary-color-6)] bg-[color:var(--primary-color-3)] p-4 hover:bg-[color:var(--primary-color-4)]",
                                to: match r.id.split_once('/') {
                                    Some((owner, name)) => Route::RepoDetail {
                                        owner: owner.to_string(),
                                        name: name.to_string(),
                                    },
                                    None => Route::Home {},
                                },
                                div { class: "space-y-2",
                                    div { class: "font-semibold truncate", "{r.id}" }
                                    div { class: "grid grid-cols-2 gap-2 text-sm",
                                        div { class: "text-[color:var(--secondary-color-5)]", "stars" }
                                        div { class: "text-right font-medium", "{r.stars}" }
                                        div { class: "text-[color:var(--secondary-color-5)]", "forks" }
                                        div { class: "text-right font-medium", "{r.forks}" }
                                    }
                                    if let Some(last) = r.last_fetched_at {
                                        div { class: "text-xs text-[color:var(--secondary-color-5)] truncate", "last: {last}" }
                                    }
                                }
                            }
                        }
                    }
                }
                },
                Some(Err(e)) => rsx! { div { class: "text-sm", "{e}" } },
                None => rsx! { div { class: "text-sm text-[color:var(--secondary-color-5)]", "Loading..." } },
            }
        }
    }
}
