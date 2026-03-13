use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn RepoListTagsSkeleton() -> Element {
    rsx! {
        Skeleton { class: "skeleton h-[104px] rounded-none border border-primary-6" }
    }
}
