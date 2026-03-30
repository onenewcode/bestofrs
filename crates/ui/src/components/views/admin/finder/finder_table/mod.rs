pub(super) mod skeleton;

use dioxus::prelude::*;

use crate::IO::repos::find_latest_pushed_repos;
use crate::types::finder::{LatestPushedRepoDto, LatestPushedRepoQueryResultDto};

use super::context::{FinderContext, FinderSortBy, FinderTablePaginationState};
use skeleton::FinderTableSkeleton;

#[component]
pub(super) fn FinderTable() -> Element {
    let refresh_nonce = use_context::<FinderContext>().refresh_nonce;
    let committed_limit = use_context::<FinderContext>().committed_limit;
    let sort_by = use_context::<FinderContext>().sort_by;
    let table_pagination = use_context::<FinderContext>().table_pagination;

    let result = use_server_future(move || {
        let _ = refresh_nonce();
        let limit = committed_limit().clamp(1, 1000);
        async move { find_latest_pushed_repos(limit).await }
    })?;

    rsx! {
        match result() {
            Some(Ok(data)) => rsx! {
                FinderTableLoaded {
                    data,
                    sort_by: sort_by(),
                    table_pagination,
                }
            },
            Some(Err(err)) => rsx! {
                div { class: "flex h-full min-h-[360px] items-center justify-center rounded-md border border-primary-6 bg-primary-1 text-sm text-primary-error",
                    "{err}"
                }
            },
            None => rsx! {
                FinderTableSkeleton {}
            },
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct FinderTableLoadedProps {
    data: LatestPushedRepoQueryResultDto,
    sort_by: FinderSortBy,
    table_pagination: Signal<FinderTablePaginationState>,
}

#[component]
fn FinderTableLoaded(mut props: FinderTableLoadedProps) -> Element {
    let mut sorted_items = props.data.items.clone();
    match props.sort_by {
        FinderSortBy::CreatedAtDesc => sorted_items.sort_unstable_by(|a, b| {
            b.created_at
                .cmp(&a.created_at)
                .then_with(|| b.stargazers_count.cmp(&a.stargazers_count))
        }),
        FinderSortBy::StarsDesc => sorted_items.sort_unstable_by(|a, b| {
            b.stargazers_count
                .cmp(&a.stargazers_count)
                .then_with(|| b.created_at.cmp(&a.created_at))
        }),
    }

    let mut state = (props.table_pagination)();
    let total_items = sorted_items.len() as u64;
    let page_size = state.page_size.max(1);
    let total_pages = if total_items == 0 {
        0
    } else {
        ((total_items + page_size as u64 - 1) / page_size as u64) as u32
    };

    if total_pages > 0 && state.current_page > total_pages {
        state.current_page = total_pages;
    }
    if total_pages == 0 {
        state.current_page = 1;
    }
    if state.total_pages != total_pages || state.total_items != total_items {
        state.total_pages = total_pages;
        state.total_items = total_items;
    }
    if (props.table_pagination)() != state {
        props.table_pagination.set(state);
    }

    let current_page = state.current_page.max(1);
    let start = ((current_page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(sorted_items.len());
    let page_items: Vec<LatestPushedRepoDto> = if start < sorted_items.len() {
        sorted_items[start..end].to_vec()
    } else {
        Vec::new()
    };

    rsx! {
        div { class: "flex h-full min-h-0 flex-col overflow-hidden rounded-md border border-primary-6 bg-primary-1",
            div { class: "grid grid-cols-2 gap-2 border-b border-primary-6 px-3 py-2 text-xs text-secondary-5 md:grid-cols-3 lg:grid-cols-6",
                div { "requested: {props.data.requested_limit}" }
                div { "upstream total: {props.data.upstream_total_count.unwrap_or(0)}" }
                div { "fetched raw: {props.data.fetched_raw_count}" }
                div { "unique: {props.data.unique_count}" }
                div { "filtered existing: {props.data.filtered_existing_count}" }
                div { "returned: {props.data.returned_count}" }
            }
            div { class: "min-h-0 flex-1 overflow-x-auto overflow-y-auto",
                table { class: "min-w-full text-sm",
                    thead { class: "border-b border-primary-6 bg-primary",
                        tr {
                            th { class: "px-3 py-2 text-left font-medium text-secondary-5", "Repo" }
                            th { class: "px-3 py-2 text-right font-medium text-secondary-5", "Stars" }
                            th { class: "px-3 py-2 text-left font-medium text-secondary-5", "Created At" }
                            th { class: "px-3 py-2 text-left font-medium text-secondary-5", "Pushed At" }
                        }
                    }
                    tbody {
                        if page_items.is_empty() {
                            tr {
                                td { class: "px-3 py-6 text-center text-secondary-5", colspan: "4", "暂无数据" }
                            }
                        } else {
                            for item in page_items {
                                tr { key: "{item.id}", class: "border-b border-primary-6 last:border-b-0",
                                    td { class: "px-3 py-2",
                                        a {
                                            class: "font-mono text-xs text-secondary-3 hover:text-grid-accent hover:underline",
                                            href: "https://github.com/{item.full_name}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            "{item.full_name}"
                                        }
                                    }
                                    td { class: "px-3 py-2 text-right font-mono text-xs text-secondary-3",
                                        "{item.stargazers_count}"
                                    }
                                    td { class: "px-3 py-2 font-mono text-xs text-secondary-5", "{item.created_at}" }
                                    td { class: "px-3 py-2 font-mono text-xs text-secondary-5", "{item.pushed_at}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
