pub mod layouts;
mod routes;
pub mod theme;

use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
pub use routes::Route;
use crate::impls::i18n::build_i18n_config;

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
const CHART_JS_CDN: &str = "https://cdn.jsdelivr.net/npm/chart.js@4.5.1/dist/chart.umd.min.js";

#[component]
pub fn App() -> Element {
    let _i18n = use_init_i18n(build_i18n_config);

    rsx! {
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

        document::Link { rel: "stylesheet", href: DX_COMPONENT_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: FONT_CSS }
        document::Link {
            rel: "prefetch",
            href: IA_WRITER_QUATTRO_ITALIC,
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "prefetch",
            href: IA_WRITER_QUATTRO_BOLD,
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "prefetch",
            href: IA_WRITER_MONO_ITALIC,
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "prefetch",
            href: IA_WRITER_MONO_BOLD,
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Script { src: CHART_JS_CDN, r#async: true, defer: true }

        Router::<Route> {}
    }
}
