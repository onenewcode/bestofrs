use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::icons::MenuIcon;

use super::TagListContext;

#[component]
pub(super) fn TagRouteMenu() -> Element {
    let mut ctx = use_context::<TagListContext>();
    let hovered = (ctx.menu_hovered)();
    let route_tags = (ctx.route_tags)();

    rsx! {
        div {
            class: "fixed right-3 top-18 z-30 h-auto w-auto pointer-events-auto md:right-6 md:top-20 md:h-[calc(100vh-5rem)] md:w-64",
            onmouseenter: move |_| ctx.menu_hovered.set(true),
            onmouseleave: move |_| ctx.menu_hovered.set(false),
            div { class: "relative text-right",
                div { class: if hovered { "mb-2 hidden h-10 w-10 items-center justify-center text-grid-accent transition-colors md:inline-flex" } else { "mb-2 hidden h-10 w-10 items-center justify-center text-secondary-6 transition-colors md:inline-flex" },
                    MenuIcon { width: 18, height: 18 }
                }
                button {
                    r#type: "button",
                    aria_label: t!("view_tag_tab_menu_toggle_aria_label"),
                    class: if hovered { "mb-2 inline-flex h-9 w-9 items-center justify-center text-grid-accent transition-colors md:hidden" } else { "mb-2 inline-flex h-9 w-9 items-center justify-center text-secondary-6 transition-colors md:hidden" },
                    onclick: move |_| ctx.menu_hovered.set(!hovered),
                    MenuIcon { width: 16, height: 16 }
                }
                div { class: if hovered { "absolute right-0 top-9 w-56 max-h-[calc(100vh-7.5rem)] overflow-auto space-y-1 pr-1 opacity-100 transition-opacity duration-150 md:top-10 md:w-64" } else { "pointer-events-none absolute right-0 top-9 w-56 max-h-0 overflow-hidden opacity-0 transition-opacity duration-150 md:top-10 md:w-64" },
                    if hovered {
                        for item in route_tags {
                            a {
                                key: "{item.value}",
                                href: "#{item.value}",
                                class: "block px-2 py-1 text-xs font-mono text-secondary-6 transition-colors hover:text-grid-accent md:text-sm",
                                onclick: move |_| ctx.menu_hovered.set(false),
                                "{item.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
