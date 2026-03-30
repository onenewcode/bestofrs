pub(crate) mod skeleton;

use crate::components::icons::{PlusIcon, SearchIcon, TrashIcon};
use dioxus::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::types::search::SearchResultDto;
use crate::IO::repos::{bulk_update_repo_tag, search_repos};

use app::prelude::Pagination;

use super::super::context::TagPanelMode;

fn empty_search_result(page: Pagination) -> SearchResultDto {
    SearchResultDto {
        repos: page.to_page(Vec::new(), 0),
        tags: page.to_page(Vec::new(), 0),
    }
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct GroupTabProps {
    pub mode: TagPanelMode,
    pub busy: Signal<bool>,
}

#[component]
pub(super) fn GroupTab(props: GroupTabProps) -> Element {
    let group_page = Pagination {
        limit: Some(100),
        offset: Some(0),
    };

    let mut repo_search_key = use_signal(String::new);
    let mut selected_repo_ids = use_signal(Vec::<String>::new);
    let mut bulk_message = use_signal(|| Option::<String>::None);
    let mut bulk_pending = use_signal(|| false);
    let mut repo_search = use_action({
        let page = group_page;
        move |key: String| async move {
            if key.trim().is_empty() {
                return Ok(empty_search_result(page));
            }
            search_repos(key, page).await
        }
    });

    let mut mode_snapshot = use_signal(|| Option::<TagPanelMode>::None);
    if mode_snapshot() != Some(props.mode.clone()) {
        mode_snapshot.set(Some(props.mode.clone()));
        repo_search_key.set(String::new());
        selected_repo_ids.set(Vec::new());
        bulk_message.set(None);
        bulk_pending.set(false);
    }

    let mut busy = props.busy;
    if busy() != bulk_pending() {
        busy.set(bulk_pending());
    }

    match props.mode.clone() {
        TagPanelMode::Add => rsx! {
            div { class: "flex h-full min-h-0 flex-col",
                div { class: "min-h-0 flex-1 overflow-y-auto",
                    div { class: "rounded-md border border-dashed border-primary-6 px-3 py-6 text-center text-sm text-secondary-5",
                        "请先从左侧列表选择一个 tag 进入 Edit 后再使用 group 功能"
                    }
                }
            }
        },
        TagPanelMode::Edit(current_tag) => rsx! {
            div { class: "flex h-full min-h-0 flex-col",
                div { class: "min-h-0 flex-1 space-y-3 overflow-y-auto pr-1",
                    section { class: "space-y-3",
                        div { class: "text-xs font-mono text-secondary-5",
                            "TAG BULK TO REPOS / {current_tag.label}:{current_tag.value}"
                        }
                        div { class: "flex flex-col gap-2 md:flex-row",
                            Input {
                                class: "input w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                placeholder: "搜索 repo（owner/name）",
                                value: repo_search_key,
                                oninput: move |e: FormEvent| *repo_search_key.write() = e.value(),
                                onkeydown: move |e: KeyboardEvent| {
                                    if e.key() == Key::Enter {
                                        repo_search.call(repo_search_key());
                                    }
                                },
                            }
                            Button {
                                class: "button rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3",
                                onclick: move |_: MouseEvent| repo_search.call(repo_search_key()),
                                SearchIcon { width: 14, height: 14 }
                            }
                        }

                        if let Some(Ok(result)) = repo_search.value() {
                            {
                                let repos_for_select_all = result().repos.items.clone();
                                let repos_for_list = result().repos.items.clone();
                                rsx! {
                                    div { class: "space-y-2",
                                        div { class: "flex items-center gap-2",
                                            Button {
                                                class: "button rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3",
                                                onclick: move |_: MouseEvent| {
                                                    let mut ids = selected_repo_ids();
                                                    for repo in &repos_for_select_all {
                                                        if !ids.contains(&repo.id) {
                                                            ids.push(repo.id.clone());
                                                        }
                                                    }
                                                    selected_repo_ids.set(ids);
                                                },
                                                "全选当前结果"
                                            }
                                            Button {
                                                class: "button rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3",
                                                onclick: move |_: MouseEvent| selected_repo_ids.set(Vec::new()),
                                                "清空"
                                            }
                                            span { class: "text-xs text-secondary-5", "已选 {selected_repo_ids().len()} 项" }
                                        }
                                        div { class: "max-h-[220px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary-1 p-2",
                                            for repo in repos_for_list {
                                                {
                                                    let repo_id = repo.id.clone();
                                                    let repo_id_for_handler = repo_id.clone();
                                                    rsx! {
                                                        label { key: "bulk-{repo_id}", class: "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1 hover:bg-primary-3",
                                                            Input {
                                                                class: "input",
                                                                r#type: "checkbox",
                                                                checked: selected_repo_ids().contains(&repo_id),
                                                                onchange: move |_: FormEvent| {
                                                                    let mut ids = selected_repo_ids();
                                                                    if let Some(idx) = ids.iter().position(|id| *id == repo_id_for_handler) {
                                                                        ids.remove(idx);
                                                                    } else {
                                                                        ids.push(repo_id_for_handler.clone());
                                                                    }
                                                                    selected_repo_ids.set(ids);
                                                                },
                                                            }
                                                            span { class: "text-xs", "{repo_id}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "mt-3 shrink-0 border-t border-primary-6 pt-3",
                    {
                        let label_for_add = current_tag.label.clone();
                        let value_for_add = current_tag.value.clone();
                        let label_for_remove = current_tag.label.clone();
                        let value_for_remove = current_tag.value.clone();
                        rsx! {
                            section { class: "space-y-2",
                                div { class: "text-xs font-mono text-secondary-5", "BULK ACTIONS" }
                                div { class: "flex flex-wrap gap-2",
                                    Button {
                                        class: "button rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                                        disabled: bulk_pending(),
                                        onclick: move |_: MouseEvent| {
                                            let repo_ids = selected_repo_ids();
                                            if repo_ids.is_empty() {
                                                *bulk_message.write() = Some("请先选择 repo".to_string());
                                                return;
                                            }
                                            *bulk_pending.write() = true;
                                            *bulk_message.write() = None;
                                            let target_label = label_for_add.clone();
                                            let target_value = value_for_add.clone();
                                            spawn(async move {
                                                match bulk_update_repo_tag(repo_ids, target_label, target_value, "add".to_string()).await {
                                                    Ok(res) => *bulk_message.write() = Some(format!(
                                                        "Add 完成: total={} updated={} skipped={}",
                                                        res.total, res.updated, res.skipped
                                                    )),
                                                    Err(err) => *bulk_message.write() = Some(err.to_string()),
                                                }
                                                *bulk_pending.write() = false;
                                            });
                                        },
                                        span { class: "inline-flex items-center gap-1",
                                            PlusIcon { width: 14, height: 14 }
                                            "Add"
                                        }
                                    }
                                    Button {
                                        class: "button rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                                        disabled: bulk_pending(),
                                        onclick: move |_: MouseEvent| {
                                            let repo_ids = selected_repo_ids();
                                            if repo_ids.is_empty() {
                                                *bulk_message.write() = Some("请先选择 repo".to_string());
                                                return;
                                            }
                                            *bulk_pending.write() = true;
                                            *bulk_message.write() = None;
                                            let target_label = label_for_remove.clone();
                                            let target_value = value_for_remove.clone();
                                            spawn(async move {
                                                match bulk_update_repo_tag(repo_ids, target_label, target_value, "remove".to_string()).await {
                                                    Ok(res) => *bulk_message.write() = Some(format!(
                                                        "Remove 完成: total={} updated={} skipped={}",
                                                        res.total, res.updated, res.skipped
                                                    )),
                                                    Err(err) => *bulk_message.write() = Some(err.to_string()),
                                                }
                                                *bulk_pending.write() = false;
                                            });
                                        },
                                        span { class: "inline-flex items-center gap-1",
                                            TrashIcon { width: 14, height: 14 }
                                            "Remove"
                                        }
                                    }
                                }
                                if bulk_pending() {
                                    div { class: "text-xs text-secondary-5", "处理中..." }
                                }
                                if let Some(msg) = bulk_message() {
                                    div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                                }
                            }
                        }
                    }
                }
            }
        },
    }
}
