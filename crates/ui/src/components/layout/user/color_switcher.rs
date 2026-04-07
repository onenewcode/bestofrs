use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_use_js::use_js;

use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::providers::PreferenceContext;
use crate::IO::user::set_grid_theme;

use_js!("src/js/theme_bridge.js"::js_apply_grid_theme);

const GRID_THEMES: [(&str, &str); 7] = [
    ("red", "#e8473c"),
    ("orange", "#f28c1b"),
    ("yellow", "#d4b100"),
    ("green", "#2fa84f"),
    ("cyan", "#1aa6a6"),
    ("blue", "#2f6fd4"),
    ("purple", "#8756c9"),
];
const DEFAULT_GRID_THEME: &str = "green";

fn normalize_grid_theme(raw: &str) -> &'static str {
    match raw {
        "red" => "red",
        "orange" => "orange",
        "yellow" => "yellow",
        "green" => "green",
        "cyan" => "cyan",
        "blue" => "blue",
        "purple" => "purple",
        _ => DEFAULT_GRID_THEME,
    }
}

#[component]
pub fn ColorSwitcher() -> Element {
    let mut preference = use_context::<PreferenceContext>();
    let mut persist_grid_theme =
        use_action(move |grid_theme: String| async move { set_grid_theme(grid_theme).await });

    use_effect(move || {
        let current_theme = preference().grid_theme;
        spawn(async move {
            let theme = normalize_grid_theme(&current_theme).to_string();
            let _ = js_apply_grid_theme::<()>(theme).await;
        });
    });

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
                                let selected = normalize_grid_theme(value.as_str()).to_string();
                                preference.with_mut(|pref| pref.grid_theme = selected.clone());
                                persist_grid_theme.call(selected.clone());
                                spawn(async move {
                                    let _ = js_apply_grid_theme::<()>(selected).await;
                                });
                            },
                            span {
                                class: match normalize_grid_theme(&preference().grid_theme) == *theme {
                                    true => "block h-3 w-3 rounded-full ring-1 ring-secondary-2 ring-offset-1 ring-offset-primary",
                                    false => "block h-3 w-3 rounded-full",
                                },
                                style: "background-color: {color};",
                            }
                        }
                    }
                }
            }
        }
    }
}
