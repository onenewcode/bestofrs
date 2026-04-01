use crate::components::icons::{EraserIcon, PlusIcon, SearchIcon};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::input::Input;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub(super) struct ProjectsSearchProps {
    pub on_add: Callback<()>,
    pub on_search: Callback<String>,
    pub on_clear: Callback<()>,
}

#[component]
pub(super) fn ProjectsSearch(props: ProjectsSearchProps) -> Element {
    let mut input_value = use_signal(String::new);

    rsx! {
        div { class: "flex flex-col gap-2 md:flex-row",
            Input {
                class: "input w-full rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-focused-border",
                placeholder: "搜索 repo_id / name / slug / description",
                value: input_value,
                oninput: move |e: FormEvent| *input_value.write() = e.value(),
                onkeydown: move |e: KeyboardEvent| {
                    if e.key() == Key::Enter {
                        props.on_search.call(input_value().trim().to_string());
                    }
                },
            }
            Button {
                variant: ButtonVariant::Primary,
                class: "button rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3",
                onclick: move |_: MouseEvent| props.on_search.call(input_value().trim().to_string()),
                SearchIcon { width: 16, height: 16 }
            }
            Button {
                variant: ButtonVariant::Ghost,
                class: "button rounded-md border border-primary-6 bg-primary px-3 py-2 text-sm hover:bg-primary-3",
                onclick: move |_: MouseEvent| {
                    input_value.set(String::new());
                    props.on_clear.call(());
                },
                EraserIcon { width: 16, height: 16 }
            }
            Button {
                variant: ButtonVariant::Primary,
                class: "button rounded-md border border-secondary-2 bg-secondary-2 px-3 py-2 text-sm font-medium text-primary hover:opacity-90",
                onclick: move |_: MouseEvent| props.on_add.call(()),
                span { class: "inline-flex items-center gap-1",
                    PlusIcon { width: 16, height: 16 }
                    "ADD"
                }
            }
        }
    }
}
