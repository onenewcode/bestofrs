use dioxus::prelude::*;

use super::{time_range_text, TimeRange};

#[derive(Props, Clone, PartialEq)]
pub(super) struct HomeTimeRangeButtonProps {
    range: TimeRange,
    active: bool,
    onclick: EventHandler<MouseEvent>,
}

#[component]
pub(super) fn HomeTimeRangeButton(props: HomeTimeRangeButtonProps) -> Element {
    rsx! {
        div { class: "relative group w-full md:w-auto",
            div {
                class: "absolute inset-0 translate-x-[6px] translate-y-[6px] rounded-md border transition-all duration-300 md:translate-x-[10px] md:translate-y-[10px] md:rounded-full md:border-2",
                class: if props.active { "border-focused-border" } else { "bg-primary-1 border-primary-6 group-hover:border-focused-border" }
            }
            button {
                onclick: move |e| props.onclick.call(e),
                class: "relative w-full rounded-md border-2 px-2 py-1.5 font-sans text-[10px] font-black italic uppercase tracking-[0.08em] transition-all duration-300 ease-out hover:cursor-pointer md:w-auto md:rounded-full md:border-4 md:px-8 md:py-3 md:text-sm md:tracking-[0.2em]",
                class: if props.active {
                    "bg-secondary-2 border-secondary-2 text-primary translate-x-[2px] translate-y-[2px] shadow-[0_0_12px_color-mix(in_oklab,var(--grid-accent)_24%,transparent)] md:translate-x-[3.82px] md:translate-y-[3.82px] md:shadow-[0_0_20px_color-mix(in_oklab,var(--grid-accent)_30%,transparent)]"
                } else {
                    "bg-primary border-secondary-2 text-secondary-2 hover:border-focused-border hover:text-secondary-6 hover:translate-x-[2px] hover:translate-y-[2px] hover:shadow-[0_0_12px_color-mix(in_oklab,var(--grid-accent)_20%,transparent)] md:hover:translate-x-[3.82px] md:hover:translate-y-[3.82px] md:hover:shadow-[0_0_20px_color-mix(in_oklab,var(--grid-accent)_22%,transparent)]"
                },
                "{time_range_text(props.range)}"
            }
        }
    }
}
