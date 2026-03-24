use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::icons::{ArrowRightIcon, StarIcon};

#[component]
pub(super) fn HomeActionSection() -> Element {
    rsx! {
        div { class: "relative z-10 mb-24 w-full max-w-7xl px-3 sm:px-4 md:px-8",
            div { class: "border-t border-primary-6 pt-16 flex flex-col md:flex-row md:items-center md:justify-between gap-8",
                div { class: "space-y-3",
                    div { class: "flex items-center gap-3",
                        div { class: "w-8 h-[1px] bg-secondary-6" }
                        span { class: "font-mono text-[10px] tracking-[0.5em] uppercase text-secondary-6 font-bold", "Action" }
                    }
                    h4 { class: "text-3xl md:text-4xl font-black font-sans uppercase tracking-tighter italic text-secondary-1 leading-none",
                        {t!("view_home_action_support_prefix")}
                        " "
                        span { class: "text-secondary-6 not-italic", "\"Best Of RS\"" }
                    }
                    p { class: "text-sm text-secondary-4 font-sans italic leading-relaxed max-w-2xl",
                        {t!("view_home_action_description")}
                    }
                }
                div { class: "mt-10 flex w-full flex-wrap items-center justify-center gap-5 md:mt-0 md:w-auto",
                    a {
                        href: "https://github.com/zhiyanzhaijie/bestofrs",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "relative group",
                        div { class: "absolute inset-0 rounded-full bg-primary-1 border-2 border-primary-6 translate-x-[10px] translate-y-[10px] transition-all duration-300 group-hover:border-focused-border" }
                        div { class: "relative flex items-center gap-3 px-8 py-3 rounded-full bg-primary border-4 border-secondary-2 text-secondary-2 group-hover:bg-secondary-2 group-hover:text-primary group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] transition-all duration-300 ease-out",
                            span { class: "font-black font-sans text-sm uppercase tracking-[0.2em] italic", "Star On Github" }
                            span { class: "group-hover:translate-x-1 transition-transform",  StarIcon {  } }
                        }
                    }
                    a {
                        href: "https://github.com/zhiyanzhaijie/bestofrs/issues/new?template=recommend_repo.yaml",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "relative group",
                        div { class: "absolute inset-0 rounded-full bg-primary-1 border-2 border-primary-6 translate-x-[10px] translate-y-[10px] transition-all duration-300 group-hover:border-focused-border" }
                        div { class: "relative flex items-center gap-3 px-8 py-3 rounded-full bg-primary border-4 border-secondary-2 text-secondary-2 group-hover:bg-secondary-2 group-hover:text-primary group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] transition-all duration-300 ease-out",
                            span { class: "font-black font-sans text-sm uppercase tracking-[0.2em] italic", "Recommend One" }
                            span { class: "group-hover:translate-x-1 transition-transform", ArrowRightIcon {  } }
                        }
                    }
                }
            }
        }
    }
}
