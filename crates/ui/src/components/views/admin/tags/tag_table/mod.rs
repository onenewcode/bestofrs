pub(crate) mod skeleton;

use dioxus::prelude::*;

use crate::components::button::ButtonVariant;
use crate::components::common::CommonPagination;
use crate::components::icons::{TrashIcon, WrenchIcon};
use crate::components::ui::alert_dialog::{
    AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogRoot, AlertDialogTitle,
};
use crate::components::ui::button::Button;
use crate::types::tags::TagListItemDto;
use crate::IO::repos::{delete_tag, list_tags_with_meta};

use super::context::TagsContext;

fn paginate_items<T: Clone>(items: &[T], current_page: u32, page_size: usize) -> (Vec<T>, u32) {
    if items.is_empty() {
        return (Vec::new(), 1);
    }
    let total_pages = items.len().div_ceil(page_size) as u32;
    let current_page = current_page.clamp(1, total_pages);
    let start = ((current_page - 1) as usize) * page_size;
    let end = (start + page_size).min(items.len());
    (items[start..end].to_vec(), total_pages)
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct TagTableProps {
    pub panel_open: bool,
    pub active_id: Option<String>,
    pub on_edit: Callback<TagListItemDto>,
}

#[component]
pub(super) fn TagTable(props: TagTableProps) -> Element {
    let mut refresh = use_context::<TagsContext>().refresh;
    let search_key = use_context::<TagsContext>().search_key;
    let mut page = use_signal(|| 1u32);
    let mut last_search_key = use_signal(String::new);
    let mut action_pending = use_signal(|| false);
    let mut table_message = use_signal(|| Option::<String>::None);
    let mut delete_confirm_open = use_signal(|| false);
    let mut delete_target_tag = use_signal(|| Option::<(String, String)>::None);
    let page_size = 20usize;

    use_effect(move || {
        let key = search_key();
        if last_search_key() != key {
            last_search_key.set(key);
            page.set(1);
        }
    });

    let tags_page = use_server_future(move || {
        let _ = refresh();
        list_tags_with_meta(Some(1), Some(500), None, Some(1))
    })?;

    let key = search_key().trim().to_lowercase();
    let table_items = match tags_page() {
        Some(Ok(page_data)) => page_data
            .items
            .into_iter()
            .filter(|tag| {
                if key.is_empty() {
                    return true;
                }
                let joined = format!(
                    "{}:{} {}",
                    tag.label.to_lowercase(),
                    tag.value.to_lowercase(),
                    tag.description.clone().unwrap_or_default().to_lowercase()
                );
                joined.contains(&key)
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };

    let (paged_items, total_pages) = paginate_items(&table_items, page(), page_size);
    let total_items = table_items.len() as u32;

    rsx! {
        div { class: "flex h-full min-h-0 flex-col gap-3",
        if total_pages > 1 {
            div { class: "shrink-0 flex justify-center",
                CommonPagination {
                    current_page: page(),
                    total_pages,
                    on_page_change: move |p| page.set(p),
                }
            }
        }
        div { class: "flex min-h-0 flex-1 flex-col overflow-hidden rounded-md border border-primary-6 bg-primary-1",

            div { class: "text-xs text-secondary-5", "{total_items} items" }
            div { class: "min-h-0 flex-1 overflow-x-auto overflow-y-auto",
                table { class: "min-w-full text-sm",
                    thead { class: "border-b border-primary-6 bg-primary",
                        tr {
                            th { class: "px-3 py-2 text-left font-medium text-secondary-5", "TAG" }
                            if !props.panel_open {
                                th { class: "px-3 py-2 text-left font-medium text-secondary-5", "DESCRIPTION" }
                            }
                            th { class: "px-3 py-2 text-right font-medium text-secondary-5", "ACTIONS" }
                        }
                    }
                    tbody {
                        match tags_page() {
                            Some(Err(err)) => rsx! {
                                tr { td { class: "px-3 py-6 text-center text-primary-error", colspan: if props.panel_open { "2" } else { "3" }, "{err}" } }
                            },
                            None => rsx! {
                                tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if props.panel_open { "2" } else { "3" }, "Loading..." } }
                            },
                            Some(Ok(_)) => {
                                if paged_items.is_empty() {
                                    rsx! {
                                        tr { td { class: "px-3 py-6 text-center text-secondary-5", colspan: if props.panel_open { "2" } else { "3" }, "无匹配结果" } }
                                    }
                                } else {
                                    rsx! {
                                        for tag in paged_items {
                                            {
                                                let row_id = format!("{}:{}", tag.label, tag.value);
                                                let is_active = props.active_id.as_deref() == Some(row_id.as_str());
                                                rsx! {
                                                    tr {
                                                        key: "{row_id}",
                                                        class: if is_active {
                                                            "border-b border-primary-6 bg-secondary-2 text-primary last:border-b-0"
                                                        } else {
                                                            "border-b border-primary-6 last:border-b-0"
                                                        },
                                                        td {
                                                            class: if is_active {
                                                                "px-3 py-2 font-mono text-xs font-medium text-primary"
                                                            } else {
                                                                "px-3 py-2 font-mono text-xs"
                                                            },
                                                            "{tag.label}:{tag.value}"
                                                        }
                                                        if !props.panel_open {
                                                            td {
                                                                class: if is_active {
                                                                    "px-3 py-2 max-w-[320px] truncate text-primary"
                                                                } else {
                                                                    "px-3 py-2 text-secondary-5 max-w-[320px] truncate"
                                                                },
                                                                "{tag.description.clone().unwrap_or_default()}"
                                                            }
                                                        }
                                                        td { class: "px-3 py-2",
                                                            div { class: "flex justify-end gap-2",
                                                                Button {
                                                                    variant: ButtonVariant::Secondary,
                                                                    class: "button rounded-md border border-primary-6 bg-primary p-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                                    disabled: action_pending(),
                                                                    onclick: {
                                                                        let t = tag.clone();
                                                                        move |_: MouseEvent| props.on_edit.call(t.clone())
                                                                    },
                                                                    WrenchIcon { width: 14, height: 14 }
                                                                }
                                                                if !props.panel_open {
                                                                    Button {
                                                                        variant: ButtonVariant::Destructive,
                                                                        class: "button rounded-md border border-primary-6 bg-primary p-2 text-xs text-primary-error hover:bg-primary-3 disabled:opacity-50",
                                                                        disabled: action_pending(),
                                                                        onclick: {
                                                                            let t = tag.clone();
                                                                            move |_: MouseEvent| {
                                                                                delete_target_tag.set(Some((t.label.clone(), t.value.clone())));
                                                                                delete_confirm_open.set(true);
                                                                            }
                                                                        },
                                                                        TrashIcon { width: 14, height: 14 }
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
                        }
                    }
                }
            }
        }
        if action_pending() {
            div { class: "text-xs text-secondary-5", "处理中..." }
        }
        if let Some(msg) = table_message() {
            div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
        }
        AlertDialogRoot {
            open: delete_confirm_open(),
            on_open_change: move |v| delete_confirm_open.set(v),
            AlertDialogContent {
                AlertDialogTitle { "确认删除 Tag" }
                AlertDialogDescription {
                    if let Some((label, value)) = delete_target_tag() {
                        "将删除 tag: {label}:{value}，此操作不可撤销。"
                    } else {
                        "此操作不可撤销。"
                    }
                }
                AlertDialogActions {
                    AlertDialogCancel { "Cancel" }
                    AlertDialogAction {
                        on_click: move |_| {
                            let Some((label, value)) = delete_target_tag() else {
                                delete_confirm_open.set(false);
                                return;
                            };
                            delete_confirm_open.set(false);
                            delete_target_tag.set(None);
                            *action_pending.write() = true;
                            *table_message.write() = None;
                            spawn(async move {
                                match delete_tag(label.clone(), value.clone()).await {
                                    Ok(()) => {
                                        *table_message.write() = Some(format!("已删除 tag: {}:{}", label, value));
                                        refresh.with_mut(|v| *v += 1);
                                    }
                                    Err(err) => *table_message.write() = Some(err.to_string()),
                                }
                                *action_pending.write() = false;
                            });
                        },
                        "Confirm"
                    }
                }
            }
        }
        }
    }
}
