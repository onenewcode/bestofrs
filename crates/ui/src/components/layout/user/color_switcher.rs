use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::root::theme::set_grid_theme;

const GRID_THEMES: [(&str, &str); 7] = [
    ("red", "#e8473c"),
    ("orange", "#f28c1b"),
    ("yellow", "#d4b100"),
    ("green", "#2fa84f"),
    ("cyan", "#1aa6a6"),
    ("blue", "#2f6fd4"),
    ("purple", "#8756c9"),
];

#[component]
pub fn ColorSwitcher() -> Element {
    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                aria_label: t!("layout_user_color_switcher_select_theme"),
                style: "padding:0; width:1.6rem; height:1.6rem; border-radius:9999px; background:transparent; box-shadow:none; display:flex; align-items:center; justify-content:center;",
                span {
                    class: "block h-4 w-4 rounded-full bg-grid-accent",
                }
            }
            DropdownMenuContent {
                style: "min-width: auto;",
                div { class: "flex flex-col items-center gap-1 p-1",
                    for (idx, (theme, color)) in GRID_THEMES.iter().enumerate() {
                        DropdownMenuItem::<String> {
                            key: "{theme}",
                            index: idx,
                            value: theme.to_string(),
                            aria_label: t!("layout_user_color_switcher_set_theme", theme: *theme),
                            on_select: move |value: String| {
                                set_grid_theme(value.as_str());
                            },
                            span {
                                class: "block h-3 w-3 rounded-full",
                                style: "background-color: {color};",
                            }
                        }
                    }
                }
            }
        }
    }
}
