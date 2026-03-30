pub mod layouts;
mod routes;

use crate::impls::i18n::build_i18n_config;
use crate::types::preference::Preference;
use crate::IO::user::get_preference;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
pub use routes::Route;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FONT_CSS: Asset = asset!("/assets/font.css");
const DX_COMPONENT_CSS: Asset = asset!("/assets/dx-components-theme.css");
const IA_WRITER_QUATTRO_REGULAR: Asset = asset!(
    "/assets/fonts/iAWriterQuattroS-Regular.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const IA_WRITER_QUATTRO_ITALIC: Asset = asset!(
    "/assets/fonts/iAWriterQuattroS-Italic.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const IA_WRITER_QUATTRO_BOLD: Asset = asset!(
    "/assets/fonts/iAWriterQuattroS-Bold.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const IA_WRITER_MONO_REGULAR: Asset = asset!(
    "/assets/fonts/iAWriterMonoS-Regular.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const IA_WRITER_MONO_ITALIC: Asset = asset!(
    "/assets/fonts/iAWriterMonoS-Italic.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const IA_WRITER_MONO_BOLD: Asset = asset!(
    "/assets/fonts/iAWriterMonoS-Bold.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);

#[component]
pub fn App() -> Element {
    let _i18n = use_init_i18n(build_i18n_config);
    let preference_fut = use_server_future(get_preference)?;

    let initial_preference = match preference_fut() {
        Some(Ok(preference)) => preference,
        _ => Preference::default(),
    };
    use_context_provider(|| initial_preference.clone());

    let (theme_value, grid_theme_value, lang_value) = {
        let theme = match initial_preference.theme.as_deref() {
            Some("dark") => "dark".to_string(),
            Some("light") => "light".to_string(),
            _ => "auto".to_string(),
        };
        let grid_theme = match initial_preference.grid_theme.as_str() {
            "red" | "orange" | "yellow" | "green" | "cyan" | "blue" | "purple" => {
                initial_preference.grid_theme
            }
            _ => "green".to_string(),
        };
        let lang = if initial_preference.locale.trim().is_empty() {
            "en-US".to_string()
        } else {
            initial_preference.locale
        };
        (theme, grid_theme, lang)
    };

    let bootstrap_script = format!(
        r#"
(function () {{
  var root = document.documentElement;
  var theme = "{theme_value}";
  var gridTheme = "{grid_theme_value}";
  var lang = "{lang_value}";

  if (lang) {{
    root.setAttribute("lang", lang);
  }}

  if (theme === "dark" || theme === "light") {{
    root.setAttribute("data-theme", theme);
  }} else if (!root.getAttribute("data-theme")) {{
    var isDark = !!(window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches);
    root.setAttribute("data-theme", isDark ? "dark" : "light");
  }}

  root.setAttribute("data-grid-theme", gridTheme);
}})();
"#
    );

    rsx! {
        document::Script { {bootstrap_script} }
        document::Link {
            rel: "preload",
            href: IA_WRITER_QUATTRO_REGULAR,
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "preload",
            href: IA_WRITER_MONO_REGULAR,
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",
        }
        document::Stylesheet { href: DX_COMPONENT_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Stylesheet { href: FONT_CSS }
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
    }
}
