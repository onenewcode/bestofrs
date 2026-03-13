use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn DeltasSectionSkeleton() -> Element {
    rsx! {
        section { class: "space-y-4 border border-primary-6 bg-primary p-5 shadow-comic-sm",
            Skeleton { class: "skeleton h-5 w-32 rounded-sm" }
            Skeleton { class: "skeleton h-4 w-28 rounded-sm" }
            div { class: "border border-primary-6 bg-primary-1 p-3 space-y-2",
                Skeleton { class: "skeleton h-6 w-full rounded-sm" }
                Skeleton { class: "skeleton h-48 w-full rounded-sm" }
            }
        }
    }
}
