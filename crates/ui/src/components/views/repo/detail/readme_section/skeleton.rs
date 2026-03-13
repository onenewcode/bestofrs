use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn ReadmeSectionSkeleton() -> Element {
    rsx! {
        section { class: "space-y-4 border border-primary-6 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-2",
                Skeleton { class: "skeleton h-6 w-28 rounded-sm" }
                Skeleton { class: "skeleton h-4 w-52 rounded-sm" }
            }
            div { class: "rounded-md border border-primary-6 bg-primary-1 p-4 space-y-3",
                Skeleton { class: "skeleton h-4 w-3/4 rounded-sm" }
                Skeleton { class: "skeleton h-4 w-full rounded-sm" }
                Skeleton { class: "skeleton h-4 w-11/12 rounded-sm" }
                Skeleton { class: "skeleton h-32 w-full rounded-sm" }
            }
        }
    }
}
