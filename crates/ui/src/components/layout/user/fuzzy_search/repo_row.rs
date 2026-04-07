use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::icons::GithubIcon;
use crate::components::skeleton::Skeleton;
use crate::root::Route;
use crate::types::search::SearchRepoDto;

#[derive(Props, Clone, PartialEq)]
pub(super) struct RepoRowProps {
    pub repo: SearchRepoDto,
    pub route: Route,
    pub on_select: Callback<Route>,
}

#[component]
pub(super) fn RepoRow(props: RepoRowProps) -> Element {
    let repo_id = props.repo.id.clone();
    let full_name = props.repo.full_name.clone().unwrap_or(repo_id);
    let description = props
        .repo
        .description
        .clone()
        .unwrap_or_else(|| t!("layout_user_fuzzy_search_no_description").to_string());
    let route = props.route.clone();

    rsx! {
        div { class: "rounded-md hover:bg-primary-3",
            button {
                class: "w-full text-left rounded-md px-3 py-2 text-sm hover:cursor-pointer transition-colors",
                onclick: move |_| {
                    props.on_select.call(route.clone());
                },
                div { class: "flex items-center gap-3",
                    GithubIcon { width: 18, height: 18, class: "shrink-0 text-secondary-4" }
                    div { class: "min-w-0 flex-1",
                        div { class: "font-medium text-secondary-1 truncate", "{full_name}" }
                        div { class: "text-xs text-secondary-5 truncate", "{description}" }
                    }
                }
            }
        }
    }
}

#[component]
pub(super) fn RepoRowSkeleton() -> Element {
    rsx! {
        div { class: "rounded-md px-3 py-2",
            div { class: "flex items-center gap-3",
                Skeleton { class: "skeleton h-[18px] w-[18px] shrink-0 rounded-sm" }
                div { class: "min-w-0 flex-1 space-y-2",
                    Skeleton { class: "skeleton h-4 w-2/5 rounded-sm" }
                    Skeleton { class: "skeleton h-3 w-4/5 rounded-sm" }
                }
            }
        }
    }
}
