use dioxus::prelude::*;

use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn FinderTableSkeleton() -> Element {
    rsx! {
        div { class: "space-y-3",
            Skeleton { class: "skeleton h-[720px] w-full border border-primary-6" }
        }
    }
}
