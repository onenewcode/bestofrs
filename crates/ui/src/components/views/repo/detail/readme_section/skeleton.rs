use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn ReadmeSectionSkeleton() -> Element {
    rsx! {
        section { class: "space-y-4",
            div { class: "space-y-1",
                div { class: "flex items-end gap-2",
                    Skeleton { class: "skeleton h-10 w-32 rounded-sm md:h-14 md:w-44" }
                    Skeleton { class: "skeleton h-10 w-20 rounded-sm md:h-14 md:w-28" }
                }
                Skeleton { class: "skeleton h-4 w-56 max-w-full rounded-sm md:w-72" }
            }
            div { class: "space-y-3 bg-primary-1 md:rounded-md md:border md:border-primary-6 md:p-4",
                Skeleton { class: "skeleton h-4 w-3/4 rounded-sm" }
                Skeleton { class: "skeleton h-4 w-full rounded-sm" }
                Skeleton { class: "skeleton h-4 w-11/12 rounded-sm" }
                Skeleton { class: "skeleton h-32 w-full rounded-sm" }
            }
        }
    }
}
