use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::common::RepoAvatar;
use crate::components::icons::TagsIcon;
use crate::components::ui::avatar::AvatarImageSize;
use crate::root::Route;
use crate::IO::repos::get_tag_with_meta_by_value;

mod skeleton;
use skeleton::TagContentSkeleton;

#[component]
pub fn TagContent(value: String, #[props(default = 5)] top_n: u32) -> Element {
    let navigator = use_navigator();

    let item = use_resource({
        let value = value.clone();
        move || {
            let value = value.clone();
            async move {
                get_tag_with_meta_by_value(value, Some(top_n))
                    .await
                    .ok()
                    .flatten()
            }
        }
    });

    match item() {
        Some(Some(tag)) => rsx! {
            div { class: "flex h-full min-h-72 w-80 flex-col cursor-default",
                div { class: "border-b border-primary-5 pb-3",
                    div { class: "flex items-center gap-2",
                        TagsIcon {
                            width: 14,
                            height: 14,
                            class: "text-secondary-4",
                        }
                        h3 { class: "break-words text-sm font-bold leading-tight text-secondary-2",
                            "{tag.label}"
                        }
                    }
                    p { class: "mt-1 text-[10px] font-mono text-secondary-5",
                        "{tag.description.clone().unwrap_or_else(|| t!(\"common_tag_content_no_description\").to_string())}"
                    }
                }
                div { class: "my-3 flex min-h-0 flex-1 flex-col px-2 py-3",
                    div { class: "mb-2 flex items-center justify-between text-[10px] font-mono text-secondary-5",
                        span { {t!("common_tag_content_top_repos")} }
                        span { "{tag.repos_total}" }
                    }
                    ul { class: "min-h-0 flex-1 space-y-1 overflow-y-auto",
                        if tag.top_repos.is_empty() {
                            li { class: "px-2 py-2 text-xs text-secondary-5",
                                {t!("common_tag_content_no_repos")}
                            }
                        } else {
                            for repo in tag.top_repos.into_iter() {
                                li {
                                    key: "{repo.repo_id}",
                                    class: "flex items-center gap-2 px-2 py-2",
                                    div { class: "h-6 w-6 shrink-0 overflow-hidden",
                                        RepoAvatar {
                                            repo_id: repo.repo_id.clone(),
                                            avatar_urls: repo.avatar_urls,
                                            size: AvatarImageSize::Custom,
                                            class: "h-6 w-6 overflow-hidden border border-primary-6 bg-primary cursor-default",
                                            fallback_class: "flex h-6 w-6 items-center justify-center border border-primary-6 bg-primary-2 text-[10px] font-bold text-secondary-4",
                                        }
                                    }
                                    span { class: "truncate text-xs font-mono text-secondary-3",
                                        "{repo.repo_id}"
                                    }
                                }
                            }
                        }
                    }
                }
                button {
                    r#type: "button",
                    class: "mt-1 w-full border border-primary-6 bg-primary-1 px-3 py-2 text-center text-xs font-mono uppercase tracking-wide text-secondary-3 transition-colors hover:bg-primary-2 hover:text-secondary-2 hover:cursor-pointer",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        navigator
                            .push(Route::RepoListView {
                                tags: Some(tag.value.clone()),
                                metric: None,
                                range: None,
                                page: None,
                                size: None,
                            });
                    },
                    {t!("common_tag_content_view_all")}
                }
            }
        },
        Some(None) => rsx! {
            div { class: "flex min-h-72 w-80 items-center justify-center text-sm text-secondary-6",
                {t!("common_tag_content_tag_not_found")}
            }
        },
        None => rsx! {
            TagContentSkeleton {}
        },
    }
}
