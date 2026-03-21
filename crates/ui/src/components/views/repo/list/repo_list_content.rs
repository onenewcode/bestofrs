use dioxus::prelude::*;

use crate::components::common::{CommonPagination, RepoManuscriptCard};

use super::{
    repo_anchor_id_for_list, repo_list_memory_key, repo_list_route_from_ctx, RepoListContext,
    RepoListHeroType, RepoListMemory, REPO_LIST_MEMORY,
};
use crate::types::repos::RepoDto;

#[component]
pub(super) fn RepoListContent(
    items: Vec<RepoDto>,
    total_pages: u32,
    current_page: u32,
    hero_type: RepoListHeroType,
) -> Element {
    let ctx = use_context::<RepoListContext>();
    let navigator = use_navigator();
    let list_key = repo_list_memory_key(ctx);
    let memory = REPO_LIST_MEMORY.peek().clone();
    let restore_anchor = if memory.list_key.as_deref() == Some(list_key.as_str()) {
        memory.anchor_id.clone()
    } else {
        None
    };
    let rendered_items = items
        .into_iter()
        .map(|repo| {
            let card_anchor = repo_anchor_id_for_list(&repo.id);
            let should_restore = restore_anchor.as_deref() == Some(card_anchor.as_str());
            (repo, should_restore)
        })
        .collect::<Vec<_>>();

    let restore_anchor_for_effect = restore_anchor.clone();
    let mut restore_target = use_signal(|| None::<MountedEvent>);
    let mut restored = use_signal(|| false);

    use_effect(move || {
        if restored() {
            return;
        }
        if let Some(mounted) = restore_target() {
            restored.set(true);
            if restore_anchor_for_effect.is_some() {
                *REPO_LIST_MEMORY.write() = RepoListMemory::default();
            }
            spawn(async move {
                let _ = mounted.scroll_to(ScrollBehavior::Instant).await;
            });
        }
    });

    rsx! {
        div { class: "space-y-8",
            if rendered_items.is_empty() {
                div { class: "flex min-h-[320px] flex-col items-center justify-center border border-dashed border-primary-6 bg-primary text-center",
                    span { class: "mb-3 font-mono text-sm tracking-widest text-secondary-5",
                        "NO_DATA"
                    }
                    if hero_type == RepoListHeroType::AllProjects {
                        span { class: "text-sm text-secondary-5", "No repos found" }
                    } else {
                        span { class: "text-sm text-secondary-5",
                            "No repos for selected tag set"
                        }
                    }
                }
            } else {
                div { class: "space-y-4",
                    for (r, should_restore) in rendered_items {
                        if should_restore && !restored() {
                            div {
                                key: "{r.id}",
                                style: "scroll-margin-top: clamp(5rem, 34vh, 20rem);",
                                onmounted: move |evt| {
                                    restore_target.set(Some(evt.clone()));
                                },
                                RepoManuscriptCard {
                                    repo: r,
                                    on_open: {
                                        let list_key = list_key.clone();
                                        move |anchor_id: String| {
                                            *REPO_LIST_MEMORY.write() = RepoListMemory {
                                                list_key: Some(list_key.clone()),
                                                anchor_id: Some(anchor_id),
                                            };
                                        }
                                    },
                                }
                            }
                        } else {
                            RepoManuscriptCard {
                                key: "{r.id}",
                                repo: r,
                                on_open: {
                                    let list_key = list_key.clone();
                                    move |anchor_id: String| {
                                        *REPO_LIST_MEMORY.write() = RepoListMemory {
                                            list_key: Some(list_key.clone()),
                                            anchor_id: Some(anchor_id),
                                        };
                                    }
                                },
                            }
                        }
                    }
                }
            }
            if total_pages > 1 {
                div { class: "pt-2",
                    CommonPagination {
                        current_page,
                        total_pages,
                        on_page_change: move |p| {
                            navigator.replace(repo_list_route_from_ctx(ctx, p, (ctx.page_size)()));
                        },
                    }
                }
            }
        }
    }
}
