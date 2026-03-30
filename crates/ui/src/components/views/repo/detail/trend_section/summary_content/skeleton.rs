use dioxus::prelude::*;

use crate::components::icons::RustGearIcon;
use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn TrendSummarySkeleton() -> Element {
    rsx! {
        div { class: "mx-auto grid w-full grid-cols-3 gap-2 md:w-[64%] md:gap-6",
            for _ in 0..3 {
                div { class: "relative flex min-h-[120px] items-center justify-center md:min-h-[230px]",
                    RustGearIcon {
                        width: 180.0,
                        class: "absolute text-primary-6 scale-[0.52] md:scale-100",
                    }
                    div { class: "relative z-10 flex flex-col items-center gap-0.5 text-center md:gap-1",
                        Skeleton { class: "skeleton mb-0.5 h-3 w-14 rounded-sm md:mb-1 md:h-4 md:w-16" }
                        Skeleton { class: "skeleton h-7 w-14 rounded-sm md:h-10 md:w-24" }
                    }
                }
            }
        }
    }
}
