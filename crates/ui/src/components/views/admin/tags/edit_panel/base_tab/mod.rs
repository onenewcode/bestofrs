pub(crate) mod skeleton;

use dioxus::prelude::*;
use crate::components::icons::{PlusIcon, SaveIcon};

use crate::components::ui::button::Button;
use crate::components::ui::input::Input;
use crate::components::ui::textarea::Textarea;
use crate::IO::repos::{create_tag, import_tags_json, update_tag};

use super::super::context::{TagPanelMode, TagsContext};

#[derive(Props, Clone, PartialEq)]
pub(super) struct BaseTabProps {
    pub mode: TagPanelMode,
    pub busy: Signal<bool>,
}

#[component]
pub(super) fn BaseTab(props: BaseTabProps) -> Element {
    let mut refresh = use_context::<TagsContext>().refresh;

    let mut form_label = use_signal(String::new);
    let mut form_value = use_signal(String::new);
    let mut form_description = use_signal(String::new);
    let mut action_pending = use_signal(|| false);
    let mut panel_message = use_signal(|| Option::<String>::None);

    let mut json_import_pending = use_signal(|| false);
    let mut json_import_message = use_signal(|| Option::<String>::None);
    let mut json_file_name = use_signal(String::new);
    let mut json_file_text = use_signal(String::new);

    let mut mode_snapshot = use_signal(|| Option::<TagPanelMode>::None);
    if mode_snapshot() != Some(props.mode.clone()) {
        mode_snapshot.set(Some(props.mode.clone()));
        panel_message.set(None);
        json_import_message.set(None);
        json_file_name.set(String::new());
        json_file_text.set(String::new());
        match props.mode.clone() {
            TagPanelMode::Add => {
                form_label.set(String::new());
                form_value.set(String::new());
                form_description.set(String::new());
            }
            TagPanelMode::Edit(tag) => {
                form_label.set(tag.label);
                form_value.set(tag.value);
                form_description.set(tag.description.unwrap_or_default());
            }
        }
    }

    let mut busy = props.busy;
    let next_busy = action_pending() || json_import_pending();
    if busy() != next_busy {
        busy.set(next_busy);
    }
    let is_add_mode = matches!(props.mode, TagPanelMode::Add);

    rsx! {
        div { class: "flex h-full min-h-0 flex-col",
            div { class: "min-h-0 flex-1 space-y-3 overflow-y-auto pr-1",
                if let TagPanelMode::Edit(ref tag) = props.mode {
                    div { class: "rounded-md bg-primary-1 px-3 py-2 text-xs text-secondary-5",
                        "editing: {tag.label}:{tag.value}"
                    }
                }
                Input {
                    class: "input w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                    placeholder: "label *",
                    value: form_label,
                    disabled: matches!(props.mode, TagPanelMode::Edit(_)),
                    oninput: move |e: FormEvent| *form_label.write() = e.value(),
                }
                Input {
                    class: "input w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                    placeholder: "value *",
                    value: form_value,
                    disabled: matches!(props.mode, TagPanelMode::Edit(_)),
                    oninput: move |e: FormEvent| *form_value.write() = e.value(),
                }
                Textarea {
                    class: "textarea w-full min-h-[120px] rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                    placeholder: "description",
                    value: form_description,
                    oninput: move |e: FormEvent| *form_description.write() = e.value(),
                }
                if matches!(props.mode, TagPanelMode::Add) {
                    section { class: "space-y-2",
                        div { class: "text-xs font-mono text-secondary-5", "JSON IMPORT" }
                        p { class: "text-xs text-secondary-5", "支持上传 JSON 文件并批量导入 tags。" }
                        Input {
                            class: "input w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs",
                            r#type: "file",
                            accept: ".json,application/json",
                            disabled: json_import_pending() || action_pending(),
                            onchange: move |e: FormEvent| {
                                *json_import_message.write() = None;
                                let files = e.files();
                                let Some(file_data) = files.first().cloned() else {
                                    *json_import_message.write() = Some("请选择 JSON 文件".to_string());
                                    return;
                                };
                                let file_name = file_data.name();
                                json_file_name.set(file_name.clone());
                                spawn(async move {
                                    match file_data.read_string().await {
                                        Ok(text) => {
                                            json_file_text.set(text);
                                            *json_import_message.write() = Some(format!("已加载文件：{file_name}"));
                                        }
                                        Err(err) => {
                                            json_file_text.set(String::new());
                                            *json_import_message.write() = Some(format!("读取文件失败: {err}"));
                                        }
                                    }
                                });
                            },
                        }
                        if !json_file_name().is_empty() {
                            div { class: "text-xs text-secondary-5", "当前文件：{json_file_name}" }
                        }
                        Button {
                            class: "button w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3 disabled:opacity-50",
                            disabled: json_import_pending() || action_pending(),
                            onclick: move |_: MouseEvent| {
                                let content = json_file_text();
                                if content.trim().is_empty() {
                                    *json_import_message.write() = Some("请先选择并加载 JSON 文件".to_string());
                                    return;
                                }
                                *json_import_pending.write() = true;
                                *json_import_message.write() = None;
                                spawn(async move {
                                    match import_tags_json(content).await {
                                        Ok(res) => {
                                            *json_import_message.write() = Some(format!(
                                                "导入完成：total={} upserted={} skipped_invalid={} failed_upsert={}",
                                                res.total, res.upserted, res.skipped_invalid, res.failed_upsert
                                            ));
                                            refresh.with_mut(|v| *v += 1);
                                        }
                                        Err(err) => *json_import_message.write() = Some(err.to_string()),
                                    }
                                    *json_import_pending.write() = false;
                                });
                            },
                            "导入 JSON"
                        }
                        if json_import_pending() {
                            div { class: "text-xs text-secondary-5", "导入处理中..." }
                        }
                        if let Some(msg) = json_import_message() {
                            div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                        }
                    }
                }
            }
            div { class: "mt-3 shrink-0 border-t border-primary-6 pt-3",
                section { class: "space-y-2",
                    div { class: "text-xs font-mono text-secondary-5", "SINGLE ADD / EDIT" }
                Button {
                    class: "button w-full rounded-md border border-secondary-2 bg-secondary-2 px-3 py-2 text-sm font-medium text-primary hover:opacity-90 disabled:opacity-50",
                    disabled: action_pending() || json_import_pending(),
                    onclick: move |_: MouseEvent| {
                        let label = form_label().trim().to_string();
                        let value = form_value().trim().to_string();
                        if label.is_empty() || value.is_empty() {
                            *panel_message.write() = Some("label/value 不能为空".to_string());
                            return;
                        }
                        let description = form_description().trim().to_string();
                        let description = if description.is_empty() { None } else { Some(description) };
                        *action_pending.write() = true;
                        *panel_message.write() = None;
                        if is_add_mode {
                            spawn(async move {
                                match create_tag(label.clone(), value.clone()).await {
                                    Ok(()) => match update_tag(label.clone(), value.clone(), description).await {
                                        Ok(()) => {
                                            *panel_message.write() = Some("创建成功".to_string());
                                            refresh.with_mut(|v| *v += 1);
                                        }
                                        Err(err) => *panel_message.write() = Some(err.to_string()),
                                    },
                                    Err(err) => *panel_message.write() = Some(err.to_string()),
                                }
                                *action_pending.write() = false;
                            });
                        } else {
                            spawn(async move {
                                match update_tag(label.clone(), value.clone(), description).await {
                                    Ok(()) => {
                                        *panel_message.write() = Some("更新成功".to_string());
                                        refresh.with_mut(|v| *v += 1);
                                    }
                                    Err(err) => *panel_message.write() = Some(err.to_string()),
                                }
                                *action_pending.write() = false;
                            });
                        }
                    },
                    if is_add_mode {
                        span { class: "inline-flex items-center gap-1",
                            PlusIcon { width: 16, height: 16 }
                            "Add"
                        }
                    } else {
                        span { class: "inline-flex items-center gap-1",
                            SaveIcon { width: 16, height: 16 }
                            "Save"
                        }
                    }
                }
                    if action_pending() {
                        div { class: "text-xs text-secondary-5", "处理中..." }
                    }
                    if let Some(msg) = panel_message() {
                        div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                    }
                }
            }
        }
    }
}
