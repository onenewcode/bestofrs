use dioxus::prelude::*;
use dioxus_i18n::t;

use super::{rank_desc, rank_title, stat_icon_mobile_tab, stat_icon_tab, RankType};

#[derive(Props, Clone, PartialEq)]
pub(super) struct HomeRankTabItemProps {
    tab: RankType,
    active_tab: RankType,
    on_select: EventHandler<MouseEvent>,
}

#[component]
pub(super) fn HomeRankTabItem(props: HomeRankTabItemProps) -> Element {
    let is_active = props.active_tab == props.tab;
    let metric = rank_title(props.tab);
    rsx! {
        div {
            class: "relative flex min-w-0 flex-grow flex-col rounded-xl transition-all duration-500 md:rounded-r-2xl",
            class: if is_active { "bg-primary-1 shadow-sm" } else { "bg-transparent hover:bg-primary-1/40" },
            if is_active {
                div { class: "absolute left-0 top-0 bottom-0 hidden w-1 bg-secondary-6 md:block" }
            }
            button {
                onclick: move |e| props.on_select.call(e),
                aria_label: t!("view_home_rank_panel_tab_aria_label", metric: metric.clone()),
                class: "relative z-10 flex w-full items-center justify-center px-2 py-3 text-left group hover:cursor-pointer md:justify-between md:px-6 md:py-5",
                div { class: "flex min-w-0 items-center justify-center gap-1.5 md:justify-start md:gap-2",
                    span {
                        class: "inline-flex min-w-0 items-center gap-1.5 font-sans text-[11px] font-black uppercase tracking-wide transition-colors md:gap-2 md:text-lg md:tracking-widest",
                        class: if is_active { "text-secondary-2" } else { "text-secondary-5 group-hover:text-secondary-2" },
                        span { class: "inline-flex items-center md:hidden", {stat_icon_mobile_tab(props.tab)} }
                        span { class: "hidden items-center md:inline-flex", {stat_icon_tab(props.tab)} }
                        span { class: "hidden md:inline", "{metric}" }
                    }
                }
                span {
                    class: "hidden text-xl transition-all duration-300 md:block md:text-3xl",
                    class: if is_active { "rotate-90 text-secondary-6" } else { "text-primary-6" },
                    "›"
                }
            }
            div {
                class: "hidden overflow-hidden px-6 transition-all duration-700 ease-in-out md:flex md:flex-grow md:items-start",
                class: if is_active { "md:pb-8 lg:pb-12" } else { "max-h-0 opacity-0" },
                div { class: "relative pt-2",
                    p { class: "text-sm text-secondary-4 font-mono leading-relaxed pl-4",
                        "{rank_desc(props.tab)}"
                    }
                }
            }
        }
    }
}
