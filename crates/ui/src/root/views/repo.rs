use dioxus::prelude::*;

use crate::components::common::CommonMarkdown;
use crate::components::IOCell;
use crate::IO::repos::{get_repo, get_repo_readme, list_repo_deltas, list_repo_snapshots};
use app::prelude::Pagination;

#[component]
pub fn RepoDetail(owner: String, name: String) -> Element {
    let route_key = format!("{owner}/{name}");
    rsx! { RepoDetailContent { key: "{route_key}", owner, name } }
}

#[component]
fn RepoDetailContent(owner: String, name: String) -> Element {
    let navigator = use_navigator();
    let mut refresh_tick = use_signal(|| 0u32);
    let mut route_owner = use_signal(|| owner.clone());
    let mut route_name = use_signal(|| name.clone());

    if route_owner() != owner {
        route_owner.set(owner.clone());
    }
    if route_name() != name {
        route_name.set(name.clone());
    }

    let repo_fut = use_server_future({
        let route_owner = route_owner;
        let route_name = route_name;
        move || {
            let _ = refresh_tick();
            let owner = route_owner();
            let name = route_name();
            get_repo(owner, name)
        }
    })?;

    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-8 space-y-6",
            div { class: "flex items-center justify-between gap-4",
                div { class: "space-y-1 min-w-0",
                    h1 { class: "text-2xl font-semibold tracking-tight truncate", "{owner}/{name}" }
                    p { class: "text-sm text-secondary-5", "repo detail" }
                }

                div { class: "flex items-center gap-2",
                    button {
                        class: "text-sm text-secondary-5 hover:underline",
                        onclick: move |_| navigator.go_back(),
                        "返回"
                    }
                    button {
                        class: "inline-flex items-center justify-center rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm text-secondary-5 hover:bg-primary-3 hover:text-secondary-4",
                        onclick: move |_| refresh_tick.with_mut(|v| *v += 1),
                        "刷新"
                    }
                }
            }

            section { class: "rounded-xl border border-primary-6 bg-primary-2 p-5 space-y-4",
                h2 { class: "text-lg font-semibold", "Meta" }
                match repo_fut() {
                    Some(Ok(Some(repo))) => {
                        let github_url = format!("https://github.com/{owner}/{name}");
                        rsx! {
                            div { class: "grid grid-cols-1 gap-2 text-sm sm:grid-cols-2",
                                div { class: "flex items-center justify-between",
                                    span { class: "text-secondary-5", "id" }
                                    span { class: "font-medium", "{repo.id}" }
                                }
                                div { class: "flex items-center justify-between",
                                    span { class: "text-secondary-5", "github_repo_id" }
                                    span { class: "font-medium", "{repo.github_repo_id:?}" }
                                }
                                div { class: "flex items-center justify-between",
                                    span { class: "text-secondary-5", "stars" }
                                    span { class: "font-medium", "{repo.stars}" }
                                }
                                div { class: "flex items-center justify-between",
                                    span { class: "text-secondary-5", "forks" }
                                    span { class: "font-medium", "{repo.forks}" }
                                }
                                div { class: "flex items-center justify-between",
                                    span { class: "text-secondary-5", "open_issues" }
                                    span { class: "font-medium", "{repo.open_issues}" }
                                }
                                div { class: "flex items-center justify-between",
                                    span { class: "text-secondary-5", "watchers" }
                                    span { class: "font-medium", "{repo.watchers}" }
                                }
                            }

                            div { class: "text-sm text-secondary-5",
                                "source: "
                                a { class: "hover:underline", href: "{github_url}", target: "_blank", "{github_url}" }
                            }

                            if let Some(full_name) = repo.full_name {
                                div { class: "text-sm text-secondary-5", "full_name: {full_name}" }
                            }
                            if let Some(last) = repo.last_fetched_at {
                                div { class: "text-sm text-secondary-5", "last_fetched_at: {last}" }
                            }
                            if !repo.tags.is_empty() {
                                div { class: "flex flex-wrap gap-2 text-xs",
                                    for tag in repo.tags {
                                        span { class: "rounded-md border border-primary-6 bg-primary-1 px-2 py-0.5",
                                            "{tag.label}:{tag.value}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(Ok(None)) => rsx! { div { class: "text-sm text-secondary-5", "未找到 repo" } },
                    Some(Err(e)) => Err(e)?,
                    None => rsx! { div { class: "text-sm text-secondary-5", "Loading..." } },
                }
            }

            IOCell {
                SnapshotSection {
                    owner: route_owner,
                    name: route_name,
                    refresh_tick,
                }
            }

            IOCell {
                DeltasSection {
                    owner: route_owner,
                    name: route_name,
                    refresh_tick,
                }
            }

            IOCell {
                ReadmeSection {
                    owner: route_owner,
                    name: route_name,
                    refresh_tick,
                }
            }
        }
    }
}

#[component]
fn SnapshotSection(owner: Signal<String>, name: Signal<String>, refresh_tick: Signal<u32>) -> Element {
    let page = Pagination {
        limit: Some(100),
        offset: Some(0),
    };

    let snapshots_fut = use_server_future({
        let owner = owner;
        let name = name;
        move || {
            let _ = refresh_tick();
            let owner = owner();
            let name = name();
            list_repo_snapshots(owner, name, page)
        }
    })?;

    rsx! {
        section { class: "rounded-xl border border-primary-6 bg-primary-2 p-5 space-y-5",
            div { class: "space-y-1",
                h2 { class: "text-lg font-semibold", "Trend" }
                p { class: "text-sm text-secondary-5", "Snapshots timeline" }
            }

            match snapshots_fut() {
                Some(Ok(page)) => {
                    let mut max_stars = 1i64;
                    for item in &page.items {
                        if item.stars > max_stars {
                            max_stars = item.stars;
                        }
                    }

                    rsx! {
                        div { class: "text-sm text-secondary-5", "count: {page.meta.total}" }
                        if page.items.is_empty() {
                            div { class: "text-sm text-secondary-5", "No snapshot data" }
                        } else {
                            div { class: "space-y-2",
                                for s in page.items {
                                    div { key: "{s.snapshot_date}", class: "space-y-1",
                                        div { class: "flex items-center justify-between text-xs text-secondary-5",
                                            span { "{s.snapshot_date}" }
                                            span { "{s.stars} stars" }
                                        }
                                        div { class: "h-2 rounded-full bg-primary-1 border border-primary-6 overflow-hidden",
                                            div {
                                                class: "h-full bg-primary-4",
                                                style: "width: {((s.stars as f64 / max_stars as f64) * 100.0).clamp(3.0, 100.0):.2}%;",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Err(e)) => Err(e)?,
                None => rsx! { div { class: "text-sm text-secondary-5", "Loading timeline..." } },
            }
        }
    }
}

#[component]
fn DeltasSection(owner: Signal<String>, name: Signal<String>, refresh_tick: Signal<u32>) -> Element {
    let page = Pagination {
        limit: Some(100),
        offset: Some(0),
    };

    let deltas_fut = use_server_future({
        let owner = owner;
        let name = name;
        move || {
            let _ = refresh_tick();
            let owner = owner();
            let name = name();
            list_repo_deltas(owner, name, page)
        }
    })?;

    rsx! {
        section { class: "rounded-xl border border-primary-6 bg-primary-2 p-5 space-y-3",
            h3 { class: "text-sm font-semibold text-secondary-4", "Daily deltas" }
            match deltas_fut() {
                Some(Ok(page)) => rsx! {
                    div { class: "text-sm text-secondary-5", "count: {page.meta.total}" }
                    if page.items.is_empty() {
                        div { class: "text-sm text-secondary-5", "No delta data" }
                    } else {
                        div { class: "max-h-[360px] space-y-2 overflow-auto",
                            for d in page.items {
                                div {
                                    key: "{d.snapshot_date}",
                                    class: "rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-sm",
                                    div { class: "flex items-center justify-between",
                                        span { class: "font-medium", "{d.snapshot_date}" }
                                        span {
                                            class: if d.stars_delta.unwrap_or(0) >= 0 { "text-green-600" } else { "text-red-600" },
                                            if d.stars_delta.unwrap_or(0) >= 0 {
                                                "+{d.stars_delta.unwrap_or(0)}"
                                            } else {
                                                "{d.stars_delta.unwrap_or(0)}"
                                            }
                                        }
                                    }
                                    div { class: "text-xs text-secondary-5",
                                        "forks: {d.forks_delta:?} | issues: {d.open_issues_delta:?} | watchers: {d.watchers_delta:?}"
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => Err(e)?,
                None => rsx! { div { class: "text-sm text-secondary-5", "Loading deltas..." } },
            }
        }
    }
}

#[component]
fn ReadmeSection(owner: Signal<String>, name: Signal<String>, refresh_tick: Signal<u32>) -> Element {
    let readme_fut = use_server_future({
        let owner = owner;
        let name = name;
        move || {
            let _ = refresh_tick();
            let owner = owner();
            let name = name();
            get_repo_readme(owner, name)
        }
    })?;

    rsx! {
        section { class: "rounded-xl border border-primary-6 bg-primary-2 p-5 space-y-4",
            div { class: "space-y-1",
                h2 { class: "text-lg font-semibold", "README" }
                p { class: "text-sm text-secondary-5", "Rendered from GitHub README" }
            }

            match readme_fut() {
                Some(Ok(Some(readme))) => rsx! {
                    div { class: "rounded-md border border-primary-6 bg-primary-1 p-4",
                        CommonMarkdown {
                            src: readme.content,
                            link_base_url: readme.html_url,
                            image_base_url: readme.download_url,
                        }
                    }
                },
                Some(Ok(None)) => rsx! { div { class: "text-sm text-secondary-5", "README not found" } },
                Some(Err(e)) => Err(e)?,
                None => rsx! { div { class: "text-sm text-secondary-5", "Loading README..." } },
            }
        }
    }
}
