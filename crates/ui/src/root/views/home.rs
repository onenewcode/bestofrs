use dioxus::prelude::*;

use app::prelude::Pagination as PageQuery;

use crate::components::common::CommonPagination;
use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};
use crate::root::Route;
use crate::types::repos::RepoDto;
use crate::IO::repos::list_repos;

#[component]
pub fn Home() -> Element {
    rsx! { RepoListContent { title: "Home".to_string(), subtitle: "Repo list".to_string() } }
}

#[component]
pub fn RepoList() -> Element {
    rsx! { RepoListContent { title: "Repo".to_string(), subtitle: "All repos".to_string() } }
}

#[component]
fn RepoListContent(title: String, subtitle: String) -> Element {
    let mut page_size = use_signal(|| 50u32);
    let mut current_page = use_signal(|| 1u32);

    let repos = use_server_future(move || {
        let limit = page_size();
        let page = current_page().max(1);
        list_repos(PageQuery {
            limit: Some(limit),
            offset: Some(limit.saturating_mul(page.saturating_sub(1))),
        })
    })?;

    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-6 space-y-6",
            div { class: "bg-primary-1 rounded-3xl shadow-2xl px-8 py-8 space-y-8",
                div { class: "space-y-2",
                    h1 { class: "text-3xl font-bold tracking-tight text-secondary-4", "{title}" }
                    p { class: "text-base text-secondary-5", "{subtitle}" }
                }

            match repos() {
                Some(Ok(page)) => {
                    let meta = page.meta;
                    let total = meta.total;
                    let total_pages = meta.total_pages;
                    let items = page.items;

                    let from = if total == 0 { 0 } else { meta.offset as u64 + 1 };
                    let to = meta.offset as u64 + items.len() as u64;

                    rsx! {
                        div { class: "flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between bg-primary-3 rounded-2xl px-6 py-4 shadow-sm",
                            div { class: "text-sm font-medium text-secondary-4",
                                "total: {total}"
                                if total > 0 {
                                    span { class: "ml-2 text-secondary-5", "({from}-{to})" }
                                }
                            }

                            div { class: "flex items-center gap-3",
                                span { class: "text-sm font-medium text-secondary-5", "page size" }
                                Select::<u32> {
                                    value: Some(page_size()),
                                    placeholder: "page size",
                                    on_value_change: move |v: Option<u32>| {
                                        if let Some(v) = v {
                                            page_size.set(v);
                                            current_page.set(1);
                                        }
                                    },
                                    SelectTrigger { aria_label: "Select page size", style: "min-width: 7rem;",
                                        SelectValue {}
                                    }
                                    SelectList { aria_label: "Page size options",
                                        SelectGroup {
                                            SelectGroupLabel { "Page size" }
                                            SelectOption::<u32> {
                                                index: 0usize,
                                                value: 20u32,
                                                text_value: Some("20".to_string()),
                                                "20"
                                                SelectItemIndicator {}
                                            }
                                            SelectOption::<u32> {
                                                index: 1usize,
                                                value: 50u32,
                                                text_value: Some("50".to_string()),
                                                "50"
                                                SelectItemIndicator {}
                                            }
                                            SelectOption::<u32> {
                                                index: 2usize,
                                                value: 100u32,
                                                text_value: Some("100".to_string()),
                                                "100"
                                                SelectItemIndicator {}
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if items.is_empty() {
                            div { class: "text-center py-12 text-secondary-5", "No repos found" }
                        } else {
                            div { class: "space-y-4",
                                for r in items {
                                    RepoCard { key: "{r.id}", repo: r }
                                }
                            }
                        }

                        if total_pages > 1 {
                            CommonPagination {
                                current_page: current_page(),
                                total_pages,
                                on_page_change: move |p| current_page.set(p),
                            }
                        }
                    }
                }
                Some(Err(e)) => rsx! { div { class: "text-sm text-primary-error bg-secondary-error rounded-lg p-4", "{e}" } },
                None => rsx! { div { class: "text-center py-12 text-secondary-5", "Loading..." } },
            }
            }
        }
    }
}

#[component]
fn RepoCard(repo: RepoDto) -> Element {
    let RepoDto {
        id,
        stars,
        forks,
        last_fetched_at,
        tags,
        full_name,
        ..
    } = repo;

    // 提取 owner 和 name
    let (owner, name) = id.split_once('/').unwrap_or(("", &id));
    let display_name = full_name.as_deref().unwrap_or(&id);

    let nav = use_navigator();
    let route = if !owner.is_empty() {
        Route::RepoDetail {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    } else {
        Route::Home {}
    };

    rsx! {
        div {
            class: "group flex gap-6 p-6 rounded-2xl bg-primary-2 hover:bg-primary-3 hover:shadow-md transition-all duration-200 cursor-pointer",
            onclick: move |_| {
                let _ = nav.push(route.clone());
            },
                // Left: Avatar/Icon
                div { class: "flex-shrink-0",
                    div { class: "w-16 h-16 rounded-lg bg-primary-4 flex items-center justify-center text-2xl font-bold text-secondary-4",
                        {name.chars().next().unwrap_or('R').to_uppercase().to_string()}
                    }
                }

                // Right: Content
                div { class: "flex-1 min-w-0 space-y-3",
                    // Top: Title and description
                    div { class: "space-y-1",
                        div { class: "flex items-center gap-2",
                            h3 { class: "text-lg font-semibold text-secondary-4 group-hover:text-secondary-3 transition-colors",
                                "{display_name}"
                            }
                        }
                        div { class: "text-sm text-secondary-5 line-clamp-1",
                            "Repository for {owner}/{name}"
                        }
                    }

                    // Middle: Tags
                    if !tags.is_empty() {
                        div { class: "flex flex-wrap gap-2",
                            for tag in tags.iter().take(5) {
                                span { class: "px-3 py-1 text-xs font-medium rounded-md bg-primary-3 text-secondary-4",
                                    "{tag.label}"
                                }
                            }
                            if tags.len() > 5 {
                                span { class: "px-3 py-1 text-xs text-secondary-5",
                                    "+{tags.len() - 5}"
                                }
                            }
                        }
                    }
                }

                // Far Right: Stats and timestamp
                div { class: "flex-shrink-0 text-right space-y-2",
                    if let Some(last) = last_fetched_at {
                        div { class: "text-xs text-secondary-5",
                            "Pushed {last}"
                        }
                    }
                    div { class: "text-sm text-secondary-4",
                        div { class: "font-semibold", "{stars}★" }
                    }
                }
        }
    }
}
