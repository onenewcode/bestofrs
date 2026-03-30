mod base_tab;
mod group_tab;

use dioxus::prelude::*;

use crate::components::common::IOCell;
use crate::components::icons::XIcon;
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::components::ui::button::{Button, ButtonVariant};

use super::context::TagPanelMode;
use base_tab::skeleton::BaseTabSkeleton;
use base_tab::BaseTab;
use group_tab::skeleton::GroupTabSkeleton;
use group_tab::GroupTab;

#[derive(Props, Clone, PartialEq)]
pub(super) struct EditPanelProps {
    pub mode: TagPanelMode,
    pub on_close: Callback<()>,
}

#[component]
pub(super) fn EditPanel(props: EditPanelProps) -> Element {
    let mut panel_tab = use_signal(|| Some("base".to_string()));
    let panel_tab_read: ReadSignal<Option<String>> = panel_tab.into();
    let mut mode_snapshot = use_signal(|| Option::<TagPanelMode>::None);
    let base_tab_busy = use_signal(|| false);
    let group_tab_busy = use_signal(|| false);

    if mode_snapshot() != Some(props.mode.clone()) {
        mode_snapshot.set(Some(props.mode.clone()));
        panel_tab.set(Some("base".to_string()));
    }

    rsx! {
        div { class: "sticky top-0 h-full min-h-0 min-w-full md:min-w-0 flex-1",
            section { class: "flex h-full min-h-0 flex-col gap-3 overflow-hidden p-4",
                div { class: "flex items-center justify-between",
                    div { class: "text-sm font-semibold",
                        if matches!(props.mode, TagPanelMode::Add) { "Add Tag" } else { "Edit Tag" }
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        class: "button rounded-md bg-primary-1 px-2 py-1 text-xs hover:bg-primary-3",
                        disabled: base_tab_busy() || group_tab_busy(),
                        onclick: move |_: MouseEvent| props.on_close.call(()),
                        XIcon { width: 16, height: 16 }
                    }
                }
                Tabs {
                    class: "flex min-h-0 flex-1 flex-col gap-3".to_string(),
                    value: panel_tab_read,
                    default_value: "base".to_string(),
                    on_value_change: move |value| panel_tab.set(Some(value)),
                    TabList {
                        TabTrigger { value: "base".to_string(), index: 0usize, "base" }
                        TabTrigger { value: "group".to_string(), index: 1usize, "group" }
                    }
                    TabContent {
                        class: "min-h-0 flex-1 overflow-hidden".to_string(),
                        value: "base".to_string(),
                        index: 0usize,
                        IOCell {
                            loading_fallback: rsx! { BaseTabSkeleton {} },
                            BaseTab {
                                mode: props.mode.clone(),
                                busy: base_tab_busy,
                            }
                        }
                    }
                    TabContent {
                        class: "min-h-0 flex-1 overflow-hidden".to_string(),
                        value: "group".to_string(),
                        index: 1usize,
                        IOCell {
                            loading_fallback: rsx! { GroupTabSkeleton {} },
                            GroupTab {
                                mode: props.mode.clone(),
                                busy: group_tab_busy,
                            }
                        }
                    }
                }
            }
        }
    }
}
