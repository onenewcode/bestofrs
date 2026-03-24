use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub(super) fn HomeFaqSection() -> Element {
    rsx! {
        div { class: "w-full max-w-7xl px-2 md:px-8 mb-32 relative z-10",
            div { class: "pt-20",
                div { class: "flex items-center gap-3 mb-8",
                    div { class: "w-8 h-[1px] bg-secondary-6" }
                    span { class: "font-mono text-[10px] tracking-[0.5em] uppercase text-secondary-6 font-bold", "FAQ" }
                }
                h3 { class: "text-4xl md:text-5xl font-black font-sans uppercase tracking-tighter italic text-secondary-1 leading-none mb-14",
                    {t!("view_home_faq_title_prefix")}
                    br {}
                    span { class: "text-secondary-6 not-italic", {t!("view_home_faq_title_suffix")} }
                }
                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-x-12 gap-y-16",
                    HomeFaqItem {
                        index: 1,
                        question: t!("view_home_faq_q1_question").to_string(),
                        answer: t!("view_home_faq_q1_answer").to_string(),
                    }
                    HomeFaqItem {
                        index: 2,
                        question: t!("view_home_faq_q2_question").to_string(),
                        answer: t!("view_home_faq_q2_answer").to_string(),
                    }
                    HomeFaqItem {
                        index: 3,
                        question: t!("view_home_faq_q3_question").to_string(),
                        answer: t!("view_home_faq_q3_answer").to_string(),
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct HomeFaqItemProps {
    index: usize,
    question: String,
    answer: String,
}

#[component]
fn HomeFaqItem(props: HomeFaqItemProps) -> Element {
    rsx! {
        div { class: "space-y-4",
            h4 { class: "font-black font-sans uppercase tracking-tight text-secondary-2 flex items-center gap-2",
                span { class: "text-secondary-6 font-mono text-xs", "Q{props.index}." }
                "{props.question}"
            }
            p { class: "text-sm text-secondary-4 font-sans italic leading-relaxed",
                "{props.answer}"
            }
        }
    }
}
