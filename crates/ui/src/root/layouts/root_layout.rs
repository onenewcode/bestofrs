use crate::{
    components::providers::{ConfigContext, ConfigProvider, PreferenceProvider},
    components::{toast::ToastProvider, ScrollProgress, ScrollToTop},
    impls::i18n::parse_language,
    root::Route,
    types::preference::Preference,
};
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[component]
pub fn RootLayout() -> Element {
    // use `try_use_context` to avoid `client side` error,
    // get it from `serve side` and init ConfigProvider for inner component
    let config = try_use_context::<ConfigContext>().unwrap_or_default();
    let preference = try_use_context::<Preference>().unwrap_or_default();

    let mut i18n = i18n();

    i18n.set_language(parse_language(&preference.locale));

    rsx! {
        ToastProvider {
            ScrollProgress {}
            ScrollToTop {}
            ConfigProvider { config: config,
                PreferenceProvider { initial: preference,
                    Outlet::<Route> {}
                }
            }
        }
    }
}
