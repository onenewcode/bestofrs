use dioxus::prelude::*;

use crate::IO::projects::{
    import_projects, import_projects_json, list_projects, remove_project, search_projects,
};
use crate::types::projects::{ProjectDto, ProjectImportItem};
use app::prelude::{Page, Pagination};

fn optional_text(value: String) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn empty_projects_page(page: Pagination) -> Page<ProjectDto> {
    page.to_page(Vec::new(), 0)
}

#[component]
pub fn ProjectManagement() -> Element {
    let mut refresh = use_signal(|| 0u32);
    let projects = use_server_future(move || {
        let _ = refresh();
        list_projects(Pagination {
            limit: Some(500),
            offset: Some(0),
        })
    })?;

    let mut create_repo_id = use_signal(String::new);
    let mut create_name = use_signal(String::new);
    let mut create_slug = use_signal(String::new);
    let mut create_description = use_signal(String::new);
    let mut create_url = use_signal(String::new);
    let mut create_avatar_url = use_signal(String::new);
    let mut create_status = use_signal(String::new);
    let mut create_logo = use_signal(String::new);
    let mut create_twitter = use_signal(String::new);

    let mut selected_project = use_signal(|| Option::<ProjectDto>::None);
    let mut edit_name = use_signal(String::new);
    let mut edit_slug = use_signal(String::new);
    let mut edit_description = use_signal(String::new);
    let mut edit_url = use_signal(String::new);
    let mut edit_avatar_url = use_signal(String::new);
    let mut edit_status = use_signal(String::new);
    let mut edit_logo = use_signal(String::new);
    let mut edit_twitter = use_signal(String::new);

    let mut create_pending = use_signal(|| false);
    let mut editor_pending = use_signal(|| false);
    let mut remove_pending = use_signal(|| false);
    let mut create_message = use_signal(|| Option::<String>::None);
    let mut editor_message = use_signal(|| Option::<String>::None);
    let page = Pagination {
        limit: Some(50),
        offset: Some(0),
    };
    let mut project_editor_search_key = use_signal(String::new);
    let mut project_editor_search = use_action({
        let page = page;
        move |key: String| async move {
            if key.trim().is_empty() {
                return Ok(empty_projects_page(page));
            }
            search_projects(key, page).await
        }
    });

    let mut json_file = use_signal(|| Option::<dioxus_elements::FileData>::None);
    let mut json_file_name = use_signal(String::new);

    rsx! {
        section { class: "space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "PROJECTS / MANAGEMENT" }
                h2 { class: "text-lg font-semibold tracking-tight text-secondary-3", "Project 管理" }
                p { class: "border-l-2 border-primary-6 pl-3 text-sm text-secondary-5",
                    "左侧用于新增与列表，右侧用于单 repo 深度编辑，可快速切换。"
                }
            }

            div { class: "grid grid-cols-1 gap-6 xl:grid-cols-12",
                div { class: "space-y-4 xl:col-span-7",
                    div { class: "space-y-3 border border-primary-6 bg-primary-1 p-4",
                        div { class: "text-sm font-semibold", "新增 Project" }
                        div { class: "text-xs font-medium text-secondary-5", "必填字段" }
                        div { class: "grid grid-cols-1 gap-2 md:grid-cols-2",
                            input {
                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                placeholder: "repo_id (owner/name) *",
                                value: create_repo_id,
                                oninput: move |e| *create_repo_id.write() = e.value(),
                            }
                            input {
                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                placeholder: "name *",
                                value: create_name,
                                oninput: move |e| *create_name.write() = e.value(),
                            }
                            input {
                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                placeholder: "slug *",
                                value: create_slug,
                                oninput: move |e| *create_slug.write() = e.value(),
                            }
                        }
                        textarea {
                            class: "w-full min-h-[96px] rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                            placeholder: "description",
                            value: create_description,
                            oninput: move |e| *create_description.write() = e.value(),
                        }

                        div { class: "pt-1 text-xs font-medium text-secondary-5", "可选字段" }
                        div { class: "grid grid-cols-1 gap-2 md:grid-cols-2",
                            input {
                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                placeholder: "url (homepage fallback)",
                                value: create_url,
                                oninput: move |e| *create_url.write() = e.value(),
                            }
                            input {
                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                placeholder: "avatar_url (highest priority)",
                                value: create_avatar_url,
                                oninput: move |e| *create_avatar_url.write() = e.value(),
                            }
                            input {
                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                placeholder: "status",
                                value: create_status,
                                oninput: move |e| *create_status.write() = e.value(),
                            }
                            input {
                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                placeholder: "logo",
                                value: create_logo,
                                oninput: move |e| *create_logo.write() = e.value(),
                            }
                            input {
                                class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border md:col-span-2",
                                placeholder: "twitter",
                                value: create_twitter,
                                oninput: move |e| *create_twitter.write() = e.value(),
                            }
                        }

                        div { class: "flex items-center gap-3",
                            button {
                                class: "inline-flex items-center justify-center rounded-md border border-primary-6 bg-primary px-4 py-2 text-sm font-medium hover:bg-primary-3 disabled:cursor-not-allowed disabled:opacity-50",
                                disabled: create_pending(),
                                onclick: move |_| {
                                    let repo_id = create_repo_id().trim().to_string();
                                    let name = create_name().trim().to_string();
                                    let slug = create_slug().trim().to_string();
                                    let description = create_description().to_string();
                                    if repo_id.is_empty() || name.is_empty() || slug.is_empty() {
                                        *create_message.write() = Some("必填字段不能为空（repo_id/name/slug）".to_string());
                                        return;
                                    }

                                    *create_pending.write() = true;
                                    *create_message.write() = None;
                                    let item = ProjectImportItem {
                                        id: None,
                                        repo_id,
                                        name,
                                        slug,
                                        description,
                                        url: optional_text(create_url()),
                                        avatar_url: optional_text(create_avatar_url()),
                                        status: optional_text(create_status()),
                                        logo: optional_text(create_logo()),
                                        twitter: optional_text(create_twitter()),
                                    };
                                    spawn(async move {
                                        match import_projects(vec![item]).await {
                                            Ok(res) => {
                                                *create_message.write() = Some(format!(
                                                    "导入完成：total={} upserted={} skipped_invalid={} failed_upsert={}",
                                                    res.total, res.upserted, res.skipped_invalid, res.failed_upsert
                                                ));
                                                create_repo_id.set(String::new());
                                                create_name.set(String::new());
                                                create_slug.set(String::new());
                                                create_description.set(String::new());
                                                create_url.set(String::new());
                                                create_avatar_url.set(String::new());
                                                create_status.set(String::new());
                                                create_logo.set(String::new());
                                                create_twitter.set(String::new());
                                                refresh.with_mut(|v| *v += 1);
                                            }
                                            Err(err) => *create_message.write() = Some(err.to_string()),
                                        }
                                        *create_pending.write() = false;
                                    });
                                },
                                "添加/导入"
                            }
                            if create_pending() {
                                span { class: "text-xs text-secondary-5", "提交中..." }
                            }
                        }
                        if let Some(msg) = create_message() {
                            div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                        }
                    }

                    div { class: "space-y-2 border border-primary-6 bg-primary-1 p-4",
                        div { class: "text-sm font-medium", "通过 JSON 文件批量导入" }
                        div { class: "text-xs text-secondary-5",
                            "JSON 格式: 数组项需包含 name 与 full_name(owner/name) 字段"
                        }
                        input {
                            r#type: "file",
                            accept: ".json,application/json",
                            class: "block w-full text-sm text-secondary-5 file:mr-3 file:rounded-md file:border-0 file:bg-primary-3 file:px-3 file:py-2 file:text-sm file:font-medium file:text-secondary-6 hover:file:bg-primary-4",
                            disabled: create_pending(),
                            onchange: move |e| {
                                let files = e.files();
                                let Some(file) = files.into_iter().next() else {
                                    return;
                                };
                                *json_file.write() = Some(file.clone());
                                *json_file_name.write() = file.name();
                                *create_message.write() = None;
                            },
                        }
                        if !json_file_name().is_empty() {
                            div { class: "text-xs text-secondary-5", "已选择: {json_file_name}" }
                        }
                        button {
                            class: "inline-flex items-center justify-center rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs font-medium hover:bg-primary-3 disabled:cursor-not-allowed disabled:opacity-50",
                            disabled: create_pending() || json_file().is_none(),
                            onclick: move |_| {
                                let Some(file) = json_file() else {
                                    *create_message.write() = Some("请先选择 JSON 文件".to_string());
                                    return;
                                };
                                *create_pending.write() = true;
                                *create_message.write() = None;
                                spawn(async move {
                                    let text = match file.read_string().await {
                                        Ok(v) => v,
                                        Err(err) => {
                                            *create_message.write() = Some(err.to_string());
                                            *create_pending.write() = false;
                                            return;
                                        }
                                    };
                                    match import_projects_json(text).await {
                                        Ok(res) => {
                                            *create_message.write() = Some(format!(
                                                "导入完成：total={} upserted={} skipped_invalid={} failed_upsert={}",
                                                res.total, res.upserted, res.skipped_invalid, res.failed_upsert
                                            ));
                                            json_file.set(None);
                                            json_file_name.set(String::new());
                                            refresh.with_mut(|v| *v += 1);
                                        }
                                        Err(err) => *create_message.write() = Some(err.to_string()),
                                    }
                                    *create_pending.write() = false;
                                });
                            },
                            "上传导入"
                        }
                    }

                    div { class: "space-y-2 border-t border-dashed border-primary-6 pt-2",
                        div { class: "flex items-end justify-between",
                            h3 { class: "text-base font-semibold", "Projects 列表" }
                            match projects() {
                                Some(Ok(page)) => rsx! { span { class: "text-sm text-secondary-5", "total: {page.meta.total}" } },
                                _ => rsx! { span { class: "text-sm text-secondary-5", "" } },
                            }
                        }
                        match projects() {
                            Some(Ok(page)) => rsx! {
                                div { class: "max-h-[560px] space-y-2 overflow-auto",
                                    for p in page.items {
                                        div { key: "{p.id}", class: "flex items-center justify-between gap-3 rounded-md border border-primary-6 bg-primary-1 px-3 py-2",
                                            div { class: "min-w-0",
                                                div { class: "font-medium truncate", "{p.name}" }
                                                div { class: "text-xs text-secondary-5 truncate", "{p.repo_id}" }
                                                if let Some(url) = &p.url {
                                                    div { class: "text-xs text-secondary-5 truncate", "url: {url}" }
                                                }
                                            }
                                            div { class: "flex items-center gap-3 shrink-0",
                                                div { class: "text-xs text-secondary-5", "{p.slug}" }
                                                button {
                                                    class: "text-xs text-secondary-5 hover:underline",
                                                    disabled: editor_pending() || remove_pending(),
                                                    onclick: {
                                                        let project = p.clone();
                                                        move |_| {
                                                            selected_project.set(Some(project.clone()));
                                                            edit_name.set(project.name.clone());
                                                            edit_slug.set(project.slug.clone());
                                                            edit_description.set(project.description.clone());
                                                            edit_url.set(project.url.clone().unwrap_or_default());
                                                            edit_avatar_url.set(project.avatar_url.clone().unwrap_or_default());
                                                            edit_status.set(project.status.clone().unwrap_or_default());
                                                            edit_logo.set(project.logo.clone().unwrap_or_default());
                                                            edit_twitter.set(project.twitter.clone().unwrap_or_default());
                                                            editor_message.set(None);
                                                        }
                                                    },
                                                    "编辑"
                                                }
                                                button {
                                                    class: "text-xs text-red-600 hover:underline disabled:cursor-not-allowed disabled:opacity-50",
                                                    disabled: remove_pending() || editor_pending(),
                                                    onclick: move |_| {
                                                        let repo_id = p.repo_id.clone();
                                                        *remove_pending.write() = true;
                                                        *create_message.write() = None;
                                                        spawn(async move {
                                                            match remove_project(repo_id.clone()).await {
                                                                Ok(()) => {
                                                                    *create_message.write() = Some(format!("已删除 project: {repo_id}"));
                                                                    if selected_project().as_ref().map(|x| x.repo_id.as_str()) == Some(repo_id.as_str()) {
                                                                        selected_project.set(None);
                                                                    }
                                                                    refresh.with_mut(|v| *v += 1);
                                                                }
                                                                Err(err) => *create_message.write() = Some(err.to_string()),
                                                            }
                                                            *remove_pending.write() = false;
                                                        });
                                                    },
                                                    "删除"
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            Some(Err(e)) => Err(e)?,
                            None => rsx! { div { class: "text-sm text-secondary-5", "Loading projects..." } },
                        }
                    }
                }

                div { class: "space-y-4 xl:col-span-5",
                    section { class: "space-y-3 border border-primary-6 bg-primary-1 p-4 xl:sticky xl:top-4",
                        div { class: "space-y-1",
                            div { class: "text-sm font-semibold", "Repo 深度编辑面板" }
                            p { class: "text-xs text-secondary-5", "可先搜索 project 并选择编辑，也可继续从左侧列表点击切换。" }
                        }
                        div { class: "space-y-2 border-b border-dashed border-primary-6 pb-3",
                            div { class: "text-xs font-medium text-secondary-5", "搜索 Project 并编辑" }
                            div { class: "flex flex-col gap-2 md:flex-row",
                                input {
                                    class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                    placeholder: "输入 repo_id / name / slug / description",
                                    value: project_editor_search_key,
                                    oninput: move |e| *project_editor_search_key.write() = e.value(),
                                    onkeydown: move |e| {
                                        if e.key() == Key::Enter {
                                            project_editor_search.call(project_editor_search_key());
                                        }
                                    },
                                }
                                button {
                                    class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3",
                                    onclick: move |_| project_editor_search.call(project_editor_search_key()),
                                    "搜索"
                                }
                            }
                            if let Some(Ok(result)) = project_editor_search.value() {
                                {
                                    let projects = result().items.clone();
                                    rsx! {
                                        div { class: "max-h-[220px] space-y-2 overflow-auto rounded-md border border-primary-6 bg-primary p-2",
                                            if projects.is_empty() {
                                                div { class: "text-xs text-secondary-5", "无匹配结果" }
                                            } else {
                                                for p in projects {
                                                    div { key: "search-{p.id}", class: "flex items-center justify-between gap-2 rounded-md border border-primary-6 bg-primary-1 px-2 py-2",
                                                        div { class: "min-w-0",
                                                            div { class: "truncate text-sm font-medium", "{p.name}" }
                                                            div { class: "truncate text-xs text-secondary-5", "{p.repo_id}" }
                                                        }
                                                        button {
                                                            class: "rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3 disabled:opacity-50",
                                                            disabled: editor_pending() || remove_pending(),
                                                            onclick: {
                                                                let project = p.clone();
                                                                move |_| {
                                                                    selected_project.set(Some(project.clone()));
                                                                    edit_name.set(project.name.clone());
                                                                    edit_slug.set(project.slug.clone());
                                                                    edit_description.set(project.description.clone());
                                                                    edit_url.set(project.url.clone().unwrap_or_default());
                                                                    edit_avatar_url.set(project.avatar_url.clone().unwrap_or_default());
                                                                    edit_status.set(project.status.clone().unwrap_or_default());
                                                                    edit_logo.set(project.logo.clone().unwrap_or_default());
                                                                    edit_twitter.set(project.twitter.clone().unwrap_or_default());
                                                                    editor_message.set(Some(format!("已通过搜索选择: {}", project.repo_id)));
                                                                }
                                                            },
                                                            "选择编辑"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if let Some(Err(err)) = project_editor_search.value() {
                                div { class: "text-xs text-primary-error", "{err}" }
                            }
                        }
                        if let Some(selected) = selected_project() {
                            div { class: "space-y-3",
                                div { class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-xs text-secondary-5",
                                    "repo_id: {selected.repo_id}"
                                }
                                div { class: "text-xs font-medium text-secondary-5", "必填字段" }
                                input {
                                    class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                    placeholder: "name *",
                                    value: edit_name,
                                    oninput: move |e| *edit_name.write() = e.value(),
                                }
                                input {
                                    class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                    placeholder: "slug *",
                                    value: edit_slug,
                                    oninput: move |e| *edit_slug.write() = e.value(),
                                }
                                textarea {
                                    class: "w-full min-h-[96px] rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                    placeholder: "description",
                                    value: edit_description,
                                    oninput: move |e| *edit_description.write() = e.value(),
                                }

                                div { class: "pt-1 text-xs font-medium text-secondary-5", "可选字段" }
                                div { class: "grid grid-cols-1 gap-2",
                                    input {
                                        class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                        placeholder: "url (homepage fallback)",
                                        value: edit_url,
                                        oninput: move |e| *edit_url.write() = e.value(),
                                    }
                                    input {
                                        class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                        placeholder: "avatar_url (highest priority)",
                                        value: edit_avatar_url,
                                        oninput: move |e| *edit_avatar_url.write() = e.value(),
                                    }
                                    input {
                                        class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                        placeholder: "status",
                                        value: edit_status,
                                        oninput: move |e| *edit_status.write() = e.value(),
                                    }
                                    input {
                                        class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                        placeholder: "logo",
                                        value: edit_logo,
                                        oninput: move |e| *edit_logo.write() = e.value(),
                                    }
                                    input {
                                        class: "w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                                        placeholder: "twitter",
                                        value: edit_twitter,
                                        oninput: move |e| *edit_twitter.write() = e.value(),
                                    }
                                }

                                div { class: "flex items-center gap-2 border-t border-dashed border-primary-6 pt-3",
                                    button {
                                        class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3 disabled:opacity-50",
                                        disabled: editor_pending(),
                                        onclick: move |_| {
                                            let Some(project) = selected_project() else {
                                                return;
                                            };
                                            let name = edit_name().trim().to_string();
                                            let slug = edit_slug().trim().to_string();
                                            let description = edit_description().to_string();
                                            if name.is_empty() || slug.is_empty() {
                                                *editor_message.write() = Some("必填字段不能为空（name/slug）".to_string());
                                                return;
                                            }
                                            *editor_pending.write() = true;
                                            *editor_message.write() = None;
                                            let item = ProjectImportItem {
                                                id: None,
                                                repo_id: project.repo_id.clone(),
                                                name: name.clone(),
                                                slug: slug.clone(),
                                                description: description.clone(),
                                                url: optional_text(edit_url()),
                                                avatar_url: optional_text(edit_avatar_url()),
                                                status: optional_text(edit_status()),
                                                logo: optional_text(edit_logo()),
                                                twitter: optional_text(edit_twitter()),
                                            };
                                            spawn(async move {
                                                match import_projects(vec![item]).await {
                                                    Ok(res) => {
                                                        *editor_message.write() = Some(format!(
                                                            "更新完成：total={} upserted={} skipped_invalid={} failed_upsert={}",
                                                            res.total, res.upserted, res.skipped_invalid, res.failed_upsert
                                                        ));
                                                        if let Some(mut selected) = selected_project() {
                                                            selected.name = name;
                                                            selected.slug = slug;
                                                            selected.description = description;
                                                            selected.url = optional_text(edit_url());
                                                            selected.avatar_url = optional_text(edit_avatar_url());
                                                            selected.status = optional_text(edit_status());
                                                            selected.logo = optional_text(edit_logo());
                                                            selected.twitter = optional_text(edit_twitter());
                                                            selected_project.set(Some(selected));
                                                        }
                                                        refresh.with_mut(|v| *v += 1);
                                                    }
                                                    Err(err) => *editor_message.write() = Some(err.to_string()),
                                                }
                                                *editor_pending.write() = false;
                                            });
                                        },
                                        "保存更新"
                                    }
                                    button {
                                        class: "rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3",
                                        disabled: editor_pending(),
                                        onclick: move |_| {
                                            selected_project.set(None);
                                            editor_message.set(None);
                                        },
                                        "取消选择"
                                    }
                                }
                                if editor_pending() {
                                    div { class: "text-xs text-secondary-5", "处理中..." }
                                }
                                if let Some(msg) = editor_message() {
                                    div { class: "text-sm text-secondary-5 whitespace-pre-wrap", "{msg}" }
                                }
                            }
                        } else {
                            div { class: "rounded-md border border-dashed border-primary-6 bg-primary px-3 py-6 text-center text-sm text-secondary-5",
                                "请先从左侧列表选择一个 repo 进行编辑"
                            }
                        }
                    }
                }
            }
        }
    }
}
