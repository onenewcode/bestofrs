use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::{
    common::{
        CommonMarkdown, GradientDirection, GridBackground, GridPadding, GridPattern,
        GridSlashTransition, GridWrapper, SEOHead, SEOProp,
    },
    providers::PreferenceContext,
};

const ABOUT_EN_US_MD: &str = include_str!("./about_en-US.md");
const ABOUT_ZH_CN_MD: &str = include_str!("./about_zh-CN.md");

fn about_markdown_for_locale(locale: &str) -> &'static str {
    let lang = locale
        .split('-')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();

    match lang.as_str() {
        "zh" => ABOUT_ZH_CN_MD,
        "en" => ABOUT_EN_US_MD,
        _ => ABOUT_EN_US_MD,
    }
}

#[component]
pub fn About() -> Element {
    let preference = use_context::<PreferenceContext>();
    let current_locale = preference().locale;
    let md_str = about_markdown_for_locale(&current_locale);

    rsx! {
        SEOHead {
            data: SEOProp {
                title: t!("view_about_seo_title").to_string(),
                description: t!("view_about_seo_description").to_string(),
                keywords: t!("view_about_seo_keywords").to_string(),
                canonical_url: "/about".into(),
                og_type: "website".into(),
                ..Default::default()
            },
        }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        GridSlashTransition { }
        GridWrapper {
            bg_class: "opacity-76",
            padding: GridPadding::None,
            background: GridBackground {
                pattern: GridPattern::Dot,
                gradient: GradientDirection::ToTop,
            },
            section { class: "min-h-screen px-4 py-10 md:px-30 md:py-20",
                CommonMarkdown {
                    src: md_str.to_string(),
                    class: Some("about-markdown max-w-5xl".to_string()),
                }
            }
        }
        GridSlashTransition {  }
    }
}
