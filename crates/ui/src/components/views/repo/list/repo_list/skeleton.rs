use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

use super::super::{repo_list_content::RepoListContent, RepoListContext};

#[component]
pub(crate) fn RepoListCachedFallback() -> Element {
    let ctx = use_context::<RepoListContext>();
    if let Some(cached) = (ctx.last_success)() {
        rsx! {
            RepoListContent {
                items: cached.items,
                total_pages: cached.total_pages,
                current_page: cached.current_page,
                hero_type: cached.hero_type,
            }
        }
    } else {
        rsx! {
            Skeleton { class: "skeleton w-full h-full min-h-[220px] rounded-xl border border-primary-6" }
        }
    }
}
