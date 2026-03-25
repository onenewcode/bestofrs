use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_use_js::use_js;

use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::icons::{LetterChineseAIcon, LetterEnglishAIcon};
use crate::components::providers::PreferenceContext;
use crate::impls::i18n::{parse_language, DEFAULT_LANGUAGE, ZH_CN_LANGUAGE};
use crate::IO::user::set_locale;

use_js!("src/js/theme_bridge.js"::js_apply_lang);

#[component]
pub fn LanguageSwitcher() -> Element {
    let mut i18n = i18n();
    let mut preference = use_context::<PreferenceContext>();
    let mut persist_locale =
        use_action(move |locale: String| async move { set_locale(locale).await });
    let current_locale = preference().locale;
    let is_zh = current_locale == ZH_CN_LANGUAGE.to_string();

    use_effect(move || {
        let locale = preference().locale.clone();
        spawn(async move {
            let _ = js_apply_lang::<()>(locale).await;
        });
    });

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                aria_label: t!("layout_user_language_switcher_aria_label"),
                style: "padding:0; width:1.6rem; height:1.6rem; border-radius:9999px; background:transparent; box-shadow:none; display:flex; align-items:center; justify-content:center;",
                if is_zh {
                    LetterChineseAIcon { width: 16, height: 16 }
                } else {
                    LetterEnglishAIcon { width: 16, height: 16 }
                }
            }
            DropdownMenuContent {
                style: "min-width: 8rem;",
                DropdownMenuItem::<String> {
                    index: 0usize,
                    value: DEFAULT_LANGUAGE.to_string(),
                    aria_label: t!("layout_user_language_switcher_en_us"),
                    on_select: move |locale: String| {
                        let next_locale = parse_language(&locale).to_string();
                        i18n.set_language(parse_language(&next_locale));
                        preference.with_mut(|pref| pref.locale = next_locale.clone());
                        persist_locale.call(next_locale.clone());
                        spawn(async move {
                            let _ = js_apply_lang::<()>(next_locale).await;
                        });
                    },
                    span { class: "inline-flex items-center gap-2",
                        LetterEnglishAIcon { width: 14, height: 14 }
                        span { class: "text-xs", {t!("layout_user_language_switcher_en_us")} }
                    }
                }
                DropdownMenuItem::<String> {
                    index: 1usize,
                    value: ZH_CN_LANGUAGE.to_string(),
                    aria_label: t!("layout_user_language_switcher_zh_cn"),
                    on_select: move |locale: String| {
                        let next_locale = parse_language(&locale).to_string();
                        i18n.set_language(parse_language(&next_locale));
                        preference.with_mut(|pref| pref.locale = next_locale.clone());
                        persist_locale.call(next_locale.clone());
                        spawn(async move {
                            let _ = js_apply_lang::<()>(next_locale).await;
                        });
                    },
                    span { class: "inline-flex items-center gap-2",
                        LetterChineseAIcon { width: 14, height: 14 }
                        span { class: "text-xs", {t!("layout_user_language_switcher_zh_cn")} }
                    }
                }
            }
        }
    }
}
