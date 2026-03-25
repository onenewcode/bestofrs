use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

use super::super::RepoListContext;
use super::RepoListTagsContent;

#[component]
pub(crate) fn RepoListTagsSkeleton() -> Element {
    rsx! {
        Skeleton { class: "skeleton h-[120px] rounded-none border border-primary-6" }
    }
}

#[component]
pub(crate) fn RepoListTagsCachedFallback() -> Element {
    let ctx = use_context::<RepoListContext>();
    if let Some(cached) = (ctx.last_success_tags)() {
        rsx! {
            RepoListTagsContent {
                advice_tags: cached.advice_tags,
            }
        }
    } else {
        rsx! { RepoListTagsSkeleton {} }
    }
}
