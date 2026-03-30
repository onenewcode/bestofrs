pub(crate) mod skeleton;

use crate::components::icons::{PlusIcon, TrashIcon, XIcon};
use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use dioxus::prelude::*;

use crate::types::tags::TagDto;
use crate::IO::repos::{get_repo, list_tags_with_meta, replace_repo_tags};

use super::super::context::{ProjectPanelMode, ProjectsContext};

fn parse_owner_name(repo_id: &str) -> Option<(String, String)> {
    let (owner, name) = repo_id.split_once('/')?;
    if owner.is_empty() || name.is_empty() {
        return None;
    }
    Some((owner.to_string(), name.to_string()))
}

fn same_tag_key(left: &TagDto, right: &TagDto) -> bool {
    left.label == right.label && left.value == right.value
}

#[derive(Clone, PartialEq, Default)]
struct RepoEditorState {
    loading: bool,
    current_tags: Vec<TagDto>,
    selected_tags: Vec<TagDto>,
    loaded_repo_id: Option<String>,
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct RepoTabProps {
    pub mode: ProjectPanelMode,
    pub busy: Signal<bool>,
}

#[component]
pub(super) fn RepoTab(props: RepoTabProps) -> Element {
    match props.mode {
        ProjectPanelMode::Add => rsx! {
            RepoTabAddMode { busy: props.busy }
        },
        ProjectPanelMode::Edit(project) => rsx! {
            RepoTabEditMode { project, busy: props.busy }
        },
    }
}

#[derive(Props, Clone, PartialEq)]
struct RepoTabAddModeProps {
    busy: Signal<bool>,
}

#[component]
fn RepoTabAddMode(props: RepoTabAddModeProps) -> Element {
    let mut busy = props.busy;
    if busy() {
        busy.set(false);
    }

    rsx! {
        div { class: "flex h-full min-h-0 flex-col",
            div { class: "rounded-md border border-dashed border-primary-6 bg-primary px-3 py-6 text-center text-sm text-secondary-5",
                "Add Mode: Repo Tags 仅展示，不可编辑。请先创建 project，再进入 Edit Mode 自动加载并编辑关联 repo tags。"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct RepoTabEditModeProps {
    project: crate::types::projects::ProjectDto,
    busy: Signal<bool>,
}

#[component]
fn RepoTabEditMode(props: RepoTabEditModeProps) -> Element {
    let refresh = use_context::<ProjectsContext>().refresh;
    let tags_page = use_server_future(move || {
        let _ = refresh();
        list_tags_with_meta(Some(1), Some(500), None, Some(1))
    })?;

    let mut state = use_signal(RepoEditorState::default);
    let target_repo_id = props.project.repo_id.clone();
    let state_value = state();
    let mut replace_tags = use_action({
        let target_repo_id = props.project.repo_id.clone();
        move |next_tags: Vec<TagDto>| {
            let target_repo_id = target_repo_id.clone();
            async move {
                if let Some((owner, name)) = parse_owner_name(&target_repo_id) {
                    replace_repo_tags(owner, name, next_tags).await
                } else {
                    Ok(())
                }
            }
        }
    });

    if state_value.loaded_repo_id.as_deref() != Some(target_repo_id.as_str()) {
        state.with_mut(|s| {
            s.loading = true;
            s.current_tags.clear();
            s.selected_tags.clear();
            s.loaded_repo_id = Some(target_repo_id.clone());
        });

        let Some((owner, name)) = parse_owner_name(&target_repo_id) else {
            state.with_mut(|s| s.loading = false);
            return rsx! {
                div { class: "space-y-3 border border-primary-6 bg-primary p-3",
                    div { class: "text-xs font-mono text-secondary-5", "REPO TAGS EDITOR" }
                    div { class: "text-xs text-secondary-5", "关联 Repo: {props.project.repo_id}" }
                }
            };
        };

        spawn(async move {
            match get_repo(owner, name).await {
                Ok(Some(repo_detail)) => {
                    state.with_mut(|s| {
                        s.current_tags = repo_detail.tags;
                        s.selected_tags.clear();
                    });
                }
                Ok(None) => {
                    state.with_mut(|s| {
                        s.current_tags.clear();
                        s.selected_tags.clear();
                    });
                }
                Err(err) => {
                    let _ = err;
                    state.with_mut(|s| {
                        s.current_tags.clear();
                        s.selected_tags.clear();
                    });
                }
            }
            state.with_mut(|s| s.loading = false);
        });
    }

    let state_value = state();
    let pending = state_value.loading || replace_tags.pending();
    let mut busy = props.busy;
    if busy() != pending {
        busy.set(pending);
    }

    let all_tags = match tags_page() {
        Some(Ok(page)) => page
            .items
            .into_iter()
            .map(|tag| TagDto {
                label: tag.label,
                value: tag.value,
                description: tag.description,
                repos_total: Some(tag.repos_total),
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };

    rsx! {
        div { class: "flex h-full min-h-0 flex-col p-3",
            div { class: "min-h-0 flex-1 space-y-3 overflow-y-auto pr-1",
                div { class: "text-xs font-mono text-secondary-5", "REPO TAGS EDITOR" }
                div { class: "text-xs text-secondary-5", "关联 Repo: {props.project.repo_id}" }
                div { class: "text-xs font-semibold text-secondary-5", "当前已绑定 Tags" }
                if state_value.current_tags.is_empty() {
                    div { class: "text-xs text-secondary-5", "（空）" }
                } else {
                    div { class: "flex flex-wrap gap-2",
                        for tag in state_value.current_tags.clone() {
                            span { key: "cur-{tag.label}:{tag.value}", class: "inline-flex items-center gap-1 rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs",
                                span { "{tag.label}:{tag.value}" }
                                Button {
                                    class: "inline-flex h-4 w-4 items-center justify-center rounded-sm border border-primary-6 bg-primary-1 text-secondary-5 hover:bg-primary-3 disabled:opacity-50",
                                    disabled: pending,
                                    onclick: {
                                        let target_tag = tag.clone();
                                        move |_| {
                                            let mut next_tags = state().current_tags;
                                            next_tags.retain(|t| !same_tag_key(t, &target_tag));
                                            state.with_mut(|s| s.current_tags = next_tags.clone());
                                            replace_tags.call(next_tags);
                                        }
                                    },
                                    XIcon { width: 10, height: 10 }
                                }
                            }
                        }
                    }
                }
                div { class: "text-xs font-semibold text-secondary-5", "选择 tags 更新到该 Repo" }
                div { class: "max-h-[420px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary-1 p-2",
                    for tag in all_tags.clone() {
                        label { key: "repo-editor-{tag.label}:{tag.value}", class: "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1 hover:bg-primary-3",
                            Input {
                                r#type: "checkbox",
                                checked: state_value.selected_tags.iter().any(|x| x.label == tag.label && x.value == tag.value),
                                onchange: {
                                    let target_tag = tag.clone();
                                    move |_| {
                                        state.with_mut(|s| {
                                            if let Some(index) = s.selected_tags.iter().position(|x| same_tag_key(x, &target_tag)) {
                                                s.selected_tags.remove(index);
                                            } else {
                                                s.selected_tags.push(target_tag.clone());
                                            }
                                        });
                                    }
                                },
                            }
                            span { class: "text-xs", "{tag.label}:{tag.value}" }
                        }
                    }
                }
            }
            div { class: "mt-3 shrink-0 space-y-2 border-t border-primary-6 pt-3",
                div { class: "flex flex-wrap gap-2",
                    Button {
                        class: "button rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                        disabled: pending,
                        onclick: {
                            move |_| {
                                let picked_tags = state().selected_tags;
                                if picked_tags.is_empty() {
                                    return;
                                }
                                let mut next_tags = state().current_tags;
                                for tag in picked_tags {
                                    if !next_tags.iter().any(|current| same_tag_key(current, &tag)) {
                                        next_tags.push(tag);
                                    }
                                }
                                state.with_mut(|s| s.current_tags = next_tags.clone());
                                replace_tags.call(next_tags);
                            }
                        },
                        span { class: "inline-flex items-center gap-1",
                            PlusIcon { width: 14, height: 14 }
                            "Add"
                        }
                    }
                    Button {
                        class: "button rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs hover:bg-primary-3 disabled:opacity-50",
                        disabled: pending,
                        onclick: {
                            move |_| {
                                let picked_tags = state().selected_tags;
                                if picked_tags.is_empty() {
                                    return;
                                }
                                let mut next_tags = state().current_tags;
                                next_tags.retain(|tag| !picked_tags.iter().any(|picked| same_tag_key(tag, picked)));
                                state.with_mut(|s| s.current_tags = next_tags.clone());
                                replace_tags.call(next_tags);
                            }
                        },
                        span { class: "inline-flex items-center gap-1",
                            TrashIcon { width: 14, height: 14 }
                            "Remove"
                        }
                    }
                }
                if pending {
                    div { class: "text-xs text-secondary-5", "处理中..." }
                }
            }
        }
    }
}
