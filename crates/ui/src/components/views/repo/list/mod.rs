use dioxus::prelude::*;
use std::collections::BTreeSet;

use crate::components::button::{Button, ButtonVariant};
use crate::components::common::{
    CommonPagination, GradientDirection, GridBackground, GridPadding, GridPattern,
    GridSlashTransition, GridType, GridWrapper, IOCell, RepoManuscriptCard,
};
use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};
use crate::types::repos::RepoDto;
use crate::IO::repos::{list_repo_tag_facets, list_repos};
use app::prelude::Pagination as PageQuery;
#[derive(Clone, Copy, PartialEq, Eq)]
enum RepoListHeroType {
    WeeklyCurated,
    TagSearchResult,
}

#[derive(Clone, PartialEq, Eq)]
struct TagAdviceItem {
    key: String,
    count: u64,
}

fn parse_tags_query(tags: Option<&str>) -> Vec<String> {
    let mut dedup = BTreeSet::new();
    let mut result = Vec::new();
    if let Some(raw) = tags {
        for tag in raw.split(',') {
            let trimmed = tag.trim();
            if trimmed.is_empty() {
                continue;
            }
            if dedup.insert(trimmed.to_string()) {
                result.push(trimmed.to_string());
            }
        }
    }
    result
}

fn repo_contains_all_tags(repo: &RepoDto, active_tags: &[String]) -> bool {
    active_tags
        .iter()
        .all(|active| repo.tags.iter().any(|tag| tag.value == *active))
}


fn append_tag_query(active_tags: &[String], append: &str) -> String {
    let mut next = active_tags.to_vec();
    if !next.iter().any(|tag| tag == append) {
        next.push(append.to_string());
    }
    next.join(",")
}

fn remove_tag_query(active_tags: &[String], remove: &str) -> Option<String> {
    let next = active_tags
        .iter()
        .filter(|tag| tag.as_str() != remove)
        .cloned()
        .collect::<Vec<_>>();
    if next.is_empty() {
        None
    } else {
        Some(next.join(","))
    }
}

#[component]
pub fn RepoList(tags: Option<String>) -> Element {
    let mut page_size = use_signal(|| 50u32);
    let mut current_page = use_signal(|| 1u32);
    let active_tags = parse_tags_query(tags.as_deref());
    let hero_type = if active_tags.is_empty() {
        RepoListHeroType::WeeklyCurated
    } else {
        RepoListHeroType::TagSearchResult
    };
    let route_key = tags.clone().unwrap_or_default();

    rsx! {
        div { class: "space-y-0",
            GridWrapper {
                grid_type: GridType::Default,
                padding: GridPadding::Sm,
                is_dot_on: true,
                background: GridBackground {
                    pattern: GridPattern::Grid,
                    gradient: GradientDirection::ToBottom,
                },
                section { class: "relative overflow-hidden bg-transparent h-120",
                    div { class: "relative z-10 space-y-8",
                        div { class: "inline-flex items-center gap-2 border border-primary-6 bg-transparent px-2 py-1 font-mono text-xs font-semibold tracking-wide text-secondary-5",
                            if hero_type == RepoListHeroType::TagSearchResult {
                                "SEARCH / TAG_FILTER"
                            } else {
                                "VOL. 2026 / ISSUE #01"
                            }
                        }
                        div { class: "flex flex-col gap-6 lg:flex-row lg:items-end lg:justify-between",
                            div { class: "max-w-3xl space-y-4",
                                if hero_type == RepoListHeroType::TagSearchResult {
                                    h1 { class: "text-4xl font-bold tracking-tight text-secondary-2 md:text-6xl",
                                        "Search "
                                        span { class: "text-secondary-6", "Result" }
                                    }
                                    p { class: "border-l-2 border-primary-6 pl-5 text-base leading-relaxed text-secondary-4",
                                        "Apply intersection filtering by active tags. Click advice tags to continue narrowing down results."
                                    }
                                } else {
                                    h1 { class: "text-4xl font-bold tracking-tight text-secondary-2 md:text-6xl",
                                        "Weekly "
                                        span { class: "text-secondary-6", "Curated" }
                                        " List"
                                    }
                                    p { class: "border-l-2 border-primary-6 pl-5 text-base leading-relaxed text-secondary-4",
                                        "Observing the fastest growing projects in the Rust ecosystem. Data aggregated from GitHub activity and filtered for quality."
                                    }
                                }
                            }
                            div { class: "flex flex-wrap items-center gap-3",
                                Button {
                                    variant: ButtonVariant::Outline,
                                    class: "px-5 py-2.5 text-sm",
                                    "Filter"
                                }
                                Button {
                                    variant: ButtonVariant::Primary,
                                    class: "px-5 py-2.5 text-sm",
                                    "Sort: Popular"
                                }
                            }
                        }
                    }
                    div { class: "flex flex-wrap items-center justify-between gap-3 border-b border-dashed border-primary-6 pb-4",
                        div { class: "text-sm font-mono tracking-wider text-secondary-5",
                            "INDEX / REPOSITORIES"
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
                                SelectTrigger {
                                    aria_label: "Select page size",
                                    style: "min-width: 7rem;",
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
                    if hero_type == RepoListHeroType::TagSearchResult {
                        TagSearchGuide {
                            key: "{route_key}",
                            active_tags: active_tags.clone(),
                        }
                    }

                }
            }

            GridSlashTransition {  }

            GridWrapper {
                padding: GridPadding::Sm,
                section { class: "space-y-6 bg-primary-1",
                    IOCell {
                        RepoListIO {
                            page_size,
                            current_page,
                            active_tags,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TagSearchGuide(active_tags: Vec<String>) -> Element {
    let navigator = use_navigator();
    let facets = use_server_future({
        let active_tags = active_tags.clone();
        move || list_repo_tag_facets(active_tags.clone(), Some(12))
    })?;
    match facets() {
        Some(Ok(items)) => {
            let advice_tags = items
                .into_iter()
                .map(|item| TagAdviceItem {
                    key: item.value,
                    count: item.count,
                })
                .collect::<Vec<_>>();

            rsx! {
                div { class: "space-y-3 pt-4",
                    div { class: "space-y-2",
                        div { class: "text-xs font-mono tracking-wide text-secondary-5", "ACTIVE TAGS" }
                        div { class: "flex flex-wrap gap-2",
                            for tag in active_tags.iter() {
                                button {
                                    key: "{tag}",
                                    class: "border border-secondary-2 bg-secondary-2 px-2.5 py-1 text-xs font-medium text-primary shadow-comic-sm",
                                    onclick: {
                                        let active_tags = active_tags.clone();
                                        let tag = tag.clone();
                                        move |_| {
                                            let next_tags = remove_tag_query(&active_tags, &tag);
                                            navigator.push(crate::root::Route::RepoListView {
                                                tags: next_tags,
                                            });
                                        }
                                    },
                                    "{tag}"
                                }
                            }
                        }
                    }
                    if !advice_tags.is_empty() {
                        div { class: "space-y-2",
                            div { class: "text-xs font-mono tracking-wide text-secondary-5", "ADVICE TAGS" }
                            div { class: "flex flex-wrap gap-2",
                                for advice in advice_tags {
                                    button {
                                        key: "{advice.key}",
                                        class: "border border-primary-6 bg-primary px-2.5 py-1 text-xs text-secondary-5 hover:bg-primary-1",
                                        onclick: {
                                            let active_tags = active_tags.clone();
                                            move |_| {
                                                let query = append_tag_query(&active_tags, &advice.key);
                                                navigator.push(crate::root::Route::RepoListView {
                                                    tags: Some(query),
                                                });
                                            }
                                        },
                                        "{advice.key} ({advice.count})"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(Err(_)) => rsx! {},
        None => rsx! {},
    }
}

#[component]
fn RepoListIO(
    mut page_size: Signal<u32>,
    mut current_page: Signal<u32>,
    active_tags: Vec<String>,
) -> Element {
    let repos = use_server_future(move || {
        let limit = page_size();
        let page = current_page().max(1);
        list_repos(PageQuery {
            limit: Some(limit),
            offset: Some(limit.saturating_mul(page.saturating_sub(1))),
        })
    })?;

    match repos() {
        Some(Ok(page)) => {
            let meta = page.meta;
            let total = meta.total;
            let items = page.items;
            let filtered_items = if active_tags.is_empty() {
                items.clone()
            } else {
                items
                    .into_iter()
                    .filter(|repo| repo_contains_all_tags(repo, &active_tags))
                    .collect::<Vec<_>>()
            };
            let filtered_total = filtered_items.len() as u64;
            let total_pages = if active_tags.is_empty() {
                meta.total_pages
            } else {
                1
            };

            let from = if total == 0 {
                0
            } else {
                meta.offset as u64 + 1
            };
            let to = meta.offset as u64 + filtered_total;

            rsx! {
                div { class: "space-y-8",
                    div { class: "flex items-center justify-between gap-4 border border-primary-6 px-4 py-3",
                        div { class: "text-xs font-mono tracking-wide text-secondary-5",
                            if active_tags.is_empty() {
                                "ENTRIES: "
                                span { class: "font-semibold text-secondary-3", "{total}" }
                            } else {
                                "MATCHED: "
                                span { class: "font-semibold text-secondary-3", "{filtered_total}" }
                                span { class: "text-secondary-5", " / {total}" }
                            }
                        }
                        if filtered_total > 0 {
                            div { class: "text-xs font-mono tracking-wide text-secondary-5",
                                "RANGE: "
                                span { class: "font-semibold text-secondary-3", "{from}-{to}" }
                            }
                        }
                    }
                    if filtered_items.is_empty() {
                        div { class: "flex min-h-[320px] flex-col items-center justify-center border border-dashed border-primary-6 bg-primary text-center",
                            span { class: "mb-3 font-mono text-sm tracking-widest text-secondary-5",
                                "NO_DATA"
                            }
                            if active_tags.is_empty() {
                                span { class: "text-sm text-secondary-5", "No repos found" }
                            } else {
                                span { class: "text-sm text-secondary-5", "No repos for selected tag set" }
                            }
                        }
                    } else {
                        div { class: "space-y-4",
                            for r in filtered_items {
                                RepoManuscriptCard { key: "{r.id}", repo: r }
                            }
                        }
                    }
                    if total_pages > 1 && active_tags.is_empty() {
                        div { class: "pt-2",
                            CommonPagination {
                                current_page: current_page(),
                                total_pages,
                                on_page_change: move |p| current_page.set(p),
                            }
                        }
                    }
                }
            }
        }
        Some(Err(e)) => rsx! {
            div { class: "rounded-lg border border-primary-6 bg-primary-1 p-4 text-sm text-primary-error",
                "{e}"
            }
        },
        None => rsx! {},
    }
}
