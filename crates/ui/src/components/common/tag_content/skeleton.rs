use dioxus::prelude::*;

use crate::components::ui::skeleton::Skeleton;

#[component]
pub(super) fn TagContentSkeleton() -> Element {
    rsx! {
        div { class: "flex min-h-72 w-80 flex-col gap-3",
            Skeleton { class: "skeleton h-14 w-full" }
            Skeleton { class: "skeleton h-32 w-full" }
            Skeleton { class: "skeleton h-8 w-full mt-auto" }
        }
    }
}
