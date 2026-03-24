use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::icons::GithubIcon;
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
        li { class: "cursor-pointer rounded-md hover:bg-primary-3",
            button {
                class: "w-full text-left rounded-md px-3 py-2 text-sm transition-colors",
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
