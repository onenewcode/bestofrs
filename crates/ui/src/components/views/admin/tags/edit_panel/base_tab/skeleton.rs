use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn BaseTabSkeleton() -> Element {
    rsx! {
        div { class: "space-y-3",
            Skeleton { class: "skeleton h-10 w-full border border-primary-6" }
            Skeleton { class: "skeleton h-10 w-full border border-primary-6" }
            Skeleton { class: "skeleton h-[220px] w-full border border-primary-6" }
        }
    }
}
