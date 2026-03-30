use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn SnapshotContentSkeleton() -> Element {
    rsx! {
        div { class: "flex h-full w-full flex-col gap-2",
            div { class: "min-h-0 flex-1 md:border md:border-primary-6 md:bg-primary-1 md:p-3",
                Skeleton { class: "skeleton h-72 w-full rounded-sm md:h-80" }
            }
        }
    }
}
