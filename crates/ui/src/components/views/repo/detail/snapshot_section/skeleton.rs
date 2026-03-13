use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn SnapshotSectionSkeleton() -> Element {
    rsx! {
        section { class: "space-y-5 border border-primary-6 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-2",
                Skeleton { class: "skeleton h-6 w-24 rounded-sm" }
                Skeleton { class: "skeleton h-4 w-40 rounded-sm" }
            }
            Skeleton { class: "skeleton h-4 w-28 rounded-sm" }
            div { class: "border border-primary-6 bg-primary-1 p-3",
                Skeleton { class: "skeleton h-48 w-full rounded-sm" }
            }
        }
    }
}
