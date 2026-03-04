use dioxus::prelude::*;

use crate::components::common::{RepoAvatar, TypingText};
use crate::components::ui::avatar::AvatarImageSize;
use crate::root::Route;
use crate::types::tags::TagTopRepoDto;

fn parse_owner_name(repo_id: &str) -> Option<(String, String)> {
    let (owner, name) = repo_id.split_once('/')?;
    if owner.is_empty() || name.is_empty() {
        return None;
    }
    Some((owner.to_string(), name.to_string()))
}

#[component]
pub fn MiniRepoCard(repo: TagTopRepoDto) -> Element {
    let mut is_hovered = use_signal(|| false);
    let default_offset = 10.0f32;
    let press_down = default_offset * 0.382;
    let translate = if is_hovered() { press_down } else { 0.0 };
    let avatar_style = format!("transform: translate({translate}px, {translate}px);");
    let repo_name = repo.repo_id.to_uppercase();
    let base_style = if is_hovered() {
        "background-color: color-mix(in oklab, var(--grid-accent) 20%, transparent);"
    } else {
        "background-color: color-mix(in oklab, var(--primary-color-6) 36%, transparent);"
    };
    let avatar_ring_style = if is_hovered() {
        "box-shadow: 0 0 20px color-mix(in oklab, var(--grid-accent) 30%, transparent);"
    } else {
        ""
    };
    let tooltip_style = "color: var(--grid-accent);";

    rsx! {
        if let Some((owner, name)) = parse_owner_name(&repo.repo_id) {
            div {
                class: "group relative cursor-pointer",
                onmouseenter: move |_| is_hovered.set(true),
                onmouseleave: move |_| is_hovered.set(false),
                Link {
                    class: "contents",
                    to: Route::RepoDetailView { owner, name },
                    div {
                        class: if is_hovered() {
                            "absolute inset-0 translate-x-[10px] translate-y-[10px] rounded-full border border-[var(--grid-accent)] transition-all duration-300 ease-out"
                        } else {
                            "absolute inset-0 translate-x-[10px] translate-y-[10px] rounded-full border border-primary-6 transition-all duration-300 ease-out"
                        },
                        style: "{base_style}",
                    }
                    div {
                        class: if is_hovered() {
                            "relative z-10 h-12 w-12 overflow-hidden rounded-full border-4 border-[var(--grid-accent)] bg-primary-1 transition-all duration-300 ease-out"
                        } else {
                            "relative z-10 h-12 w-12 overflow-hidden rounded-full border-4 border-primary-6 bg-primary-1 grayscale transition-all duration-300 ease-out"
                        },
                        style: "{avatar_style} {avatar_ring_style}",
                        RepoAvatar {
                            repo_id: repo.repo_id.clone(),
                            avatar_urls: repo.avatar_urls.clone(),
                            class: "h-12 w-12 border-none bg-transparent".to_string(),
                            fallback_class: "flex h-12 w-12 items-center justify-center border-none bg-primary-2 text-xs font-bold text-secondary-4".to_string(),
                            size: AvatarImageSize::Small,
                        }
                    }
                }
                if is_hovered() {
                    div {
                        class: "pointer-events-none absolute left-0 top-[calc(100%+24px)] z-20 whitespace-nowrap text-[10px] font-mono font-bold tracking-[0.2em]",
                        style: "{tooltip_style}",
                        TypingText { text: repo_name.clone(), active: is_hovered(), speed_ms: 30 }
                    }
                }
            }
        } else {
            div {
                class: "group relative cursor-pointer",
                onmouseenter: move |_| is_hovered.set(true),
                onmouseleave: move |_| is_hovered.set(false),
                div {
                    class: if is_hovered() {
                        "absolute inset-0 translate-x-[10px] translate-y-[10px] rounded-full border border-[var(--grid-accent)] transition-all duration-300 ease-out"
                    } else {
                        "absolute inset-0 translate-x-[10px] translate-y-[10px] rounded-full border border-primary-6 transition-all duration-300 ease-out"
                    },
                    style: "{base_style}",
                }
                div {
                    class: if is_hovered() {
                        "relative z-10 h-12 w-12 overflow-hidden rounded-full border-4 border-[var(--grid-accent)] bg-primary-1 transition-all duration-300 ease-out"
                    } else {
                        "relative z-10 h-12 w-12 overflow-hidden rounded-full border-4 border-primary-6 bg-primary-1 grayscale transition-all duration-300 ease-out"
                    },
                    style: "{avatar_style} {avatar_ring_style}",
                    RepoAvatar {
                        repo_id: repo.repo_id.clone(),
                        avatar_urls: repo.avatar_urls.clone(),
                        class: "h-12 w-12 border-none bg-transparent".to_string(),
                        fallback_class: "flex h-12 w-12 items-center justify-center border-none bg-primary-2 text-xs font-bold text-secondary-4".to_string(),
                        size: AvatarImageSize::Small,
                    }
                }
                if is_hovered() {
                    div {
                        class: "pointer-events-none absolute left-0 top-[calc(100%+24px)] z-20 whitespace-nowrap text-[10px] font-mono font-bold tracking-[0.2em]",
                        style: "{tooltip_style}",
                        TypingText { text: repo_name.clone(), active: is_hovered(), speed_ms: 30 }
                    }
                }
            }
        }
    }
}
