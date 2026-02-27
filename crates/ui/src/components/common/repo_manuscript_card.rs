use dioxus::prelude::*;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::root::Route;
use crate::types::repos::RepoDto;

#[component]
pub fn RepoManuscriptCard(repo: RepoDto) -> Element {
    let RepoDto {
        id,
        stars,
        forks,
        full_name,
        homepage_url,
        avatar_url,
        tags,
        ..
    } = repo;

    let navigator = use_navigator();
    let (owner, name) = id.split_once('/').unwrap_or(("", &id));
    let display_name = full_name.as_deref().unwrap_or(&id);
    let github_url = if owner.is_empty() {
        format!("https://github.com/{id}")
    } else {
        format!("https://github.com/{owner}/{name}")
    };
    let homepage = homepage_url.as_deref().and_then(normalize_url);
    let favicon_url = homepage
        .as_ref()
        .map(|v| format!("{}/favicon.ico", v.trim_end_matches('/')));
    let owner_avatar_url = if owner.is_empty() {
        None
    } else {
        Some(format!("https://github.com/{owner}.png"))
    };
    let mut avatar_candidates = Vec::<String>::new();
    if let Some(url) = favicon_url {
        avatar_candidates.push(url);
    }
    if let Some(url) = avatar_url {
        if !avatar_candidates.contains(&url) {
            avatar_candidates.push(url);
        }
    }
    if let Some(url) = owner_avatar_url {
        if !avatar_candidates.contains(&url) {
            avatar_candidates.push(url);
        }
    }
    let github_fallback = "https://github.com/github.png".to_string();
    if !avatar_candidates.contains(&github_fallback) {
        avatar_candidates.push(github_fallback);
    }
    let avatar_candidates_for_error = avatar_candidates.clone();
    let mut avatar_index = use_signal(|| 0usize);
    let avatar_fallback = name
        .chars()
        .next()
        .map(|c| c.to_ascii_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    let route = if owner.is_empty() {
        Route::HomeView {}
    } else {
        Route::RepoDetailView {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    };

    rsx! {
        article {
            class: "group cursor-pointer border border-primary-6 bg-primary shadow-comic transition-all duration-200 hover:-translate-y-0.5 hover:shadow-comic-hover",
            onclick: move |_| {
                navigator.push(route.clone());
            },
            div { class: "flex items-start justify-between gap-3 p-5 pb-2",
                div { class: "flex min-w-0 items-start gap-4",
                    div { class: "relative h-14 w-14 shrink-0",
                        div { class: "absolute left-1 top-1 h-14 w-14 border border-primary-6 bg-screentone" }
                        if let Some(src) = avatar_candidates.get(avatar_index()).cloned() {
                            Avatar {
                                key: "{src}",
                                class: "relative z-10 h-14 w-14 border border-primary-6 bg-primary grayscale contrast-125 transition-all group-hover:grayscale-0",
                                size: AvatarImageSize::Large,
                                on_error: move |_| {
                                    let next = avatar_index() + 1;
                                    if next < avatar_candidates_for_error.len() {
                                        avatar_index.set(next);
                                    } else {
                                        avatar_index.set(usize::MAX);
                                    }
                                },
                                AvatarImage {
                                    src: src,
                                    alt: "{display_name} avatar",
                                }
                                AvatarFallback { "{avatar_fallback}" }
                            }
                        } else {
                            div { class: "relative z-10 flex h-14 w-14 items-center justify-center border border-primary-6 bg-primary-2 font-bold text-secondary-4",
                                "{avatar_fallback}"
                            }
                        }
                    }
                    div { class: "min-w-0",
                        div { class: "mb-1 flex items-center gap-2",
                            h3 { class: "truncate text-xl font-bold leading-tight text-secondary-2 transition-colors group-hover:text-secondary-6", "{name}" }
                            span { class: "border border-primary-6 px-1 font-mono text-[10px] font-bold text-secondary-5", "#{id}" }
                        }
                        p { class: "truncate text-xs font-mono text-secondary-5", "@{owner}" }
                    }
                }
                a {
                    href: "{github_url}",
                    class: "shrink-0 p-1 text-secondary-5 transition-colors hover:text-secondary-3",
                    target: "_blank",
                    onclick: move |evt| evt.stop_propagation(),
                    "↗"
                }
            }

            div { class: "space-y-2 px-5 py-2",
                p { class: "line-clamp-1 border-l-2 border-primary-5 pl-3 text-sm leading-relaxed text-secondary-4",
                    "Source: {github_url}"
                }
                if let Some(homepage) = homepage {
                    p { class: "truncate text-xs text-secondary-5",
                        "Site: "
                        a { href: "{homepage}", class: "hover:underline", target: "_blank", onclick: move |evt| evt.stop_propagation(), "{homepage}" }
                    }
                }
            }

            div { class: "flex flex-wrap gap-2 px-5 py-3",
                for tag in tags.iter().take(5) {
                    span { class: "border border-primary-6 bg-primary-1 px-2 py-0.5 text-[10px] font-medium text-secondary-5 transition-colors group-hover:border-primary-7",
                        "{tag.label}:{tag.value}"
                    }
                }
            }

            div { class: "mt-auto flex items-center justify-between border-t border-primary-6 bg-hatch px-5 py-3 text-xs font-mono",
                div { class: "flex items-center gap-4",
                    div { class: "flex items-center gap-1.5 border border-primary-6 bg-primary px-2 py-0.5 shadow-comic-sm",
                        span { class: "font-bold", "★" }
                        span { class: "font-bold", "{stars}" }
                    }
                    div { class: "flex items-center gap-1.5 text-secondary-5",
                        span { "⑂" }
                        span { "{forks}" }
                    }
                }
                span { class: "font-bold text-secondary-3", "VIEW →" }
            }
        }
    }
}

fn normalize_url(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}
