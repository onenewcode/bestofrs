use crate::components::skeleton::Skeleton;
use dioxus::prelude::*;

#[component]
pub(crate) fn MetaSectionSkeleton() -> Element {
    rsx! {
        section { class: "relative min-h-80 overflow-hidden",
            div { class: "relative z-10 flex items-center justify-start pb-4",
                Skeleton { class: "h-10 w-16 border border-primary-6" }
            }
            div { class: "relative z-10 flex h-full flex-col gap-6 md:flex-row md:items-stretch md:justify-between",
                div { class: "flex min-w-0 flex-1 items-start gap-6",
                    Skeleton { class: "hidden md:block h-24 w-24 shrink-0 border border-primary-6" }
                    div { class: "space-y-4 min-w-0 flex-1",
                        Skeleton { class: "h-4 w-32 border border-primary-6" }
                        Skeleton { class: "h-10 w-64 border border-primary-6" }
                        Skeleton { class: "h-4 w-full max-w-md border border-primary-6" }
                    }
                }
                div { class: "flex min-h-[220px] w-full flex-col items-start gap-3 border-l-2 border-primary-6 pl-6 md:w-64",
                    Skeleton { class: "h-9 w-full border border-primary-6" }
                    Skeleton { class: "h-9 w-full border border-primary-6" }
                }
            }
        }
    }
}
