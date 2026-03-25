use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::icons::TagsIcon;
use crate::types::tags::TagDto;

#[derive(Props, Clone, PartialEq)]
pub(super) struct TagRowProps {
    pub tag: TagDto,
    pub on_select: Callback<(String, String)>,
}

#[component]
pub(super) fn TagRow(props: TagRowProps) -> Element {
    let label_text = props.tag.label.clone();
    let value_text = props.tag.value.clone();
    let on_label = label_text.clone();
    let on_value = value_text.clone();
    let description = props
        .tag
        .description
        .clone()
        .unwrap_or_else(|| t!("layout_user_fuzzy_search_no_description").to_string());
    let repos_total = props.tag.repos_total.unwrap_or(0);

    rsx! {
        li {
            class: "flex items-center",
            button {
                class: "w-full text-left rounded-md px-3 py-2 text-sm hover:bg-primary-3 hover:cursor-pointer transition-colors",
                onclick: move |_| {
                    props.on_select.call((on_label.clone(), on_value.clone()));
                },
                div { class: "flex items-center gap-2",
                    TagsIcon { width: 24, height: 24, class: "mt-0.5 shrink-0 text-secondary-4" }
                    div { class: "min-w-0 flex-1",
                        div { class: "font-medium text-secondary-1 truncate", "{label_text}" }
                        div { class: "text-xs text-secondary-5 truncate", "{description}" }
                    }
                }
            }

            div { class: "rounded-md p-1 px-2 font-mono text-secondary-5 bg-grid-accent/20", "{repos_total}" }
        }
    }
}
