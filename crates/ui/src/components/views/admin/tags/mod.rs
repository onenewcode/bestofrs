mod context;
mod edit_panel;
mod search;
mod tag_table;

use dioxus::prelude::*;

use crate::components::common::{GridSlashTransition, GridType, GridWrapper, IOCell};
use context::{TagPanelMode, TagsContext};
use edit_panel::EditPanel;
use search::TagsSearch;
use tag_table::skeleton::TagTableSkeleton;
use tag_table::TagTable;

#[component]
pub fn Tags() -> Element {
    let refresh = use_signal(|| 0u32);
    let mut search_key = use_signal(String::new);
    let mut panel_mode = use_signal(|| Option::<TagPanelMode>::None);

    use_context_provider(|| TagsContext {
        refresh,
        search_key,
    });
    let active_id = if let Some(TagPanelMode::Edit(tag)) = panel_mode() {
        Some(format!("{}:{}", tag.label, tag.value))
    } else {
        None
    };

    rsx! {
        section { class: "h-full min-h-0 w-full overflow-x-hidden overflow-y-auto space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
            GridWrapper {
                grid_type: GridType::Inner,
                div { class: "space-y-1 mb-10",
                    h2 { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5",
                        "TAGS / MANAGEMENT"
                    }
                    p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
                        "右侧面板打开时，左侧只保留 tag 与 edit，便于快速切换。"
                    }
                }

                TagsSearch {
                    on_add: move |_| panel_mode.set(Some(TagPanelMode::Add)),
                    on_search: move |key| search_key.set(key),
                    on_clear: move |_| search_key.set(String::new()),
                }
            }
            GridSlashTransition {  }

            div { class: "flex h-full min-h-full min-w-0 items-stretch gap-4 overflow-x-auto md:overflow-visible",
                div { class: if panel_mode().is_some() { "h-full w-full md:w-105 shrink-0 flex flex-col gap-3" } else { "h-full min-w-0 flex-1 flex flex-col gap-3" },
                    div { class: "h-full min-h-0 flex-1",
                        IOCell {
                            loading_fallback: rsx! {
                                TagTableSkeleton {}
                            },
                            TagTable {
                                panel_open: panel_mode().is_some(),
                                active_id: active_id.clone(),
                                on_edit: move |tag| panel_mode.set(Some(TagPanelMode::Edit(tag))),
                            }
                        }
                    }
                }
                if let Some(mode) = panel_mode() {
                    EditPanel { mode, on_close: move |_| panel_mode.set(None) }
                }
            }
        }
    }
}
