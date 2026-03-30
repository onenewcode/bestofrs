mod project_tab;
mod repo_tab;

use dioxus::prelude::*;

use crate::components::common::IOCell;
use crate::components::icons::XIcon;
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs};
use crate::components::ui::button::{Button, ButtonVariant};

use super::context::{ProjectPanelMode, ProjectsContext};
use project_tab::skeleton::ProjectTabSkeleton;
use project_tab::ProjectTab;
use repo_tab::skeleton::RepoTabSkeleton;
use repo_tab::RepoTab;

#[derive(Props, Clone, PartialEq)]
pub(super) struct EditPanelProps {
    pub mode: ProjectPanelMode,
    pub on_close: Callback<()>,
}

#[component]
pub(super) fn EditPanel(props: EditPanelProps) -> Element {
    let mut panel_tab = use_context::<ProjectsContext>().edit_panel_tab;
    let panel_tab_read: ReadSignal<Option<String>> = panel_tab.into();
    let project_tab_busy = use_signal(|| false);
    let repo_tab_busy = use_signal(|| false);

    rsx! {
        div { class: "sticky top-0 h-full min-h-0 min-w-full md:min-w-0 flex-1",
            section { class: "flex h-full min-h-0 flex-col gap-3 overflow-hidden bg-primary p-4 shadow-comic-sm",
                div { class: "flex items-center justify-between",
                    div { class: "text-sm font-semibold",
                        if matches!(props.mode, ProjectPanelMode::Add) { "Add Project" } else { "Edit Project" }
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        class: "button rounded-md bg-primary-1 px-2 py-1 text-xs hover:bg-primary-3",
                        disabled: project_tab_busy() || repo_tab_busy(),
                        onclick: move |_: MouseEvent| props.on_close.call(()),
                        XIcon { width: 16, height: 16 }
                    }
                }
                Tabs {
                    class: "flex min-h-0 flex-1 flex-col gap-3".to_string(),
                    value: panel_tab_read,
                    default_value: "project".to_string(),
                    on_value_change: move |value| panel_tab.set(Some(value)),
                    TabList {
                        TabTrigger { value: "project".to_string(), index: 0usize, "project" }
                        TabTrigger { value: "repo".to_string(), index: 1usize, "repo" }
                    }
                    TabContent {
                        class: "min-h-0 flex-1 overflow-hidden".to_string(),
                        value: "project".to_string(),
                        index: 0usize,
                        IOCell {
                            loading_fallback: rsx! { ProjectTabSkeleton {} },
                            ProjectTab {
                                mode: props.mode.clone(),
                                busy: project_tab_busy,
                            }
                        }
                    }
                    TabContent {
                        class: "min-h-0 flex-1 overflow-hidden".to_string(),
                        value: "repo".to_string(),
                        index: 1usize,
                        IOCell {
                            loading_fallback: rsx! { RepoTabSkeleton {} },
                            RepoTab {
                                mode: props.mode.clone(),
                                busy: repo_tab_busy,
                            }
                        }
                    }
                }
            }
        }
    }
}
