mod context;
mod finder_table;

use dioxus::prelude::*;

use crate::components::common::{
    CommonPagination, GridSlashTransition, GridType, GridWrapper, IOCell,
};
use context::{FinderContext, FinderSortBy, FinderTablePaginationState};
use finder_table::FinderTable;

#[component]
pub fn Finder() -> Element {
    let mut refresh_nonce = use_signal(|| 0u32);
    let mut committed_limit = use_signal(|| 1000usize);
    let mut input_limit = use_signal(|| 1000usize);
    let mut sort_by = use_signal(|| FinderSortBy::CreatedAtDesc);
    let mut table_pagination = use_signal(FinderTablePaginationState::default);
    let mut has_requested = use_signal(|| false);

    use_context_provider(|| FinderContext {
        refresh_nonce,
        committed_limit,
        sort_by,
        table_pagination,
    });

    rsx! {
        section { class: "h-full min-h-0 w-full overflow-x-hidden overflow-y-auto space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
            GridWrapper {
                is_dot_on: true,
                grid_type: GridType::Inner,
                div { class: "space-y-1 mb-10",
                    h2 { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5",
                        "FINDER / DISCOVERY"
                    }
                    p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
                        "仅抓取 Rust 项目；使用 GitHub Search API 并在落库前按 GitHub Repo ID 过滤已存在项目。"
                    }
                }

                div { class: "flex flex-col gap-2 md:flex-row md:items-end",
                    div { class: "space-y-1",
                        label { class: "text-xs text-secondary-5", "Find limit (max 1000)" }
                        input {
                            class: "w-36 border border-secondary-2 bg-primary px-3 py-2 text-sm",
                            r#type: "number",
                            min: "1",
                            max: "1000",
                            step: "1",
                            value: "{input_limit}",
                            oninput: move |evt| {
                                let value = evt.value().trim().to_string();
                                if value.is_empty() {
                                    return;
                                }
                                if let Ok(parsed) = value.parse::<usize>() {
                                    input_limit.set(parsed.clamp(1, 1000));
                                }
                            },
                        }
                    }
                    button {
                        class: "inline-flex items-center justify-center border border-secondary-2 bg-secondary-2 px-4 py-2 text-sm font-medium text-primary transition-all hover:-translate-y-0.5 hover:shadow-comic-sm",
                        onclick: move |_| {
                            committed_limit.set(input_limit().clamp(1, 1000));
                            table_pagination.with_mut(|p| p.current_page = 1);
                            has_requested.set(true);
                            refresh_nonce.with_mut(|v| *v += 1);
                        },
                        "Find"
                    }
                }
            }

            GridSlashTransition {}

            div { class: "h-full min-h-0 flex flex-col gap-3",
                div { class: "flex flex-wrap items-center justify-between gap-2",
                    div { class: "inline-flex items-center gap-1 rounded-md border border-primary-6 bg-primary-1 p-1 text-xs",
                        button {
                            class: match sort_by() == FinderSortBy::CreatedAtDesc {
                                true => "rounded px-2 py-1 bg-secondary-2 text-primary",
                                false => "rounded px-2 py-1 text-secondary-5 hover:bg-primary-3",
                            },
                            onclick: move |_| {
                                sort_by.set(FinderSortBy::CreatedAtDesc);
                                table_pagination.with_mut(|p| p.current_page = 1);
                            },
                            "Created At"
                        }
                        button {
                            class: match sort_by() == FinderSortBy::StarsDesc {
                                true => "rounded px-2 py-1 bg-secondary-2 text-primary",
                                false => "rounded px-2 py-1 text-secondary-5 hover:bg-primary-3",
                            },
                            onclick: move |_| {
                                sort_by.set(FinderSortBy::StarsDesc);
                                table_pagination.with_mut(|p| p.current_page = 1);
                            },
                            "Stars"
                        }
                    }
                    div { class: "text-xs text-secondary-5",
                        "{table_pagination().total_items} items"
                    }
                }

                if has_requested() && table_pagination().total_pages > 1 {
                    div { class: "shrink-0 flex justify-center",
                        CommonPagination {
                            current_page: table_pagination().current_page,
                            total_pages: table_pagination().total_pages,
                            on_page_change: move |p| table_pagination.with_mut(|state| state.current_page = p),
                        }
                    }
                }

                div { class: "h-full min-h-0 flex-1",
                    if has_requested() {
                        IOCell {
                            FinderTable {}
                        }
                    } else {
                        div { class: "flex h-full min-h-[360px] items-center justify-center rounded-md border border-primary-6 bg-primary-1 text-sm text-secondary-5",
                            "点击 Find 开始抓取候选仓库。"
                        }
                    }
                }
            }
        }
    }
}
