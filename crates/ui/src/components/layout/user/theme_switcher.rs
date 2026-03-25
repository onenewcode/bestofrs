use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_use_js::use_js;

use crate::components::icons;
use crate::components::providers::PreferenceContext;
use crate::IO::user::set_theme;

use_js!("src/js/theme_bridge.js"::{js_apply_theme, js_toggle_theme});

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut preference = use_context::<PreferenceContext>();
    let mut persist_theme = use_action(move |theme: String| async move { set_theme(theme).await });

    use_effect(move || {
        let preferred_theme = preference().theme.clone();
        spawn(async move {
            let theme = preferred_theme.as_deref().unwrap_or("auto");
            let _ = js_apply_theme::<()>(theme).await;
        });
    });

    rsx! {
        button {
            class: "inline-flex h-[1.6rem] w-[1.6rem] items-center justify-center rounded-full p-0 text-secondary-5 shadow-none transition-colors hover:text-secondary-4 hover:cursor-pointer",
            onclick: move |_| {
                let mut preference = preference;
                let mut persist_theme = persist_theme;
                spawn(async move {
                    let next_theme = js_toggle_theme::<String>()
                        .await
                        .unwrap_or_else(|_| "dark".to_string());
                    preference.with_mut(|pref| pref.theme = Some(next_theme.clone()));
                    persist_theme.call(next_theme);
                });
            },
            aria_label: t!("layout_user_theme_switcher_toggle"),
            if preference().theme.as_deref() == Some("dark") {
                icons::MoonIcon { size: 18 }
            } else {
                icons::SunIcon { size: 18 }
            }
        }
    }
}
