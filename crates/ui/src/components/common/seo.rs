use crate::components::providers::ConfigContext;
use dioxus::prelude::*;

use crate::root::Route;

#[derive(Clone, PartialEq)]
pub struct SEOContext {
    pub site_name: String,
    pub title_suffix: String,
    pub default_description: String,
    pub default_image: String,
    pub language: String,
    pub site_url: String,
}

impl SEOContext {
    pub fn default_en() -> Self {
        Self {
            site_name: "Best of RS".to_string(),
            title_suffix: " | Best of RS".to_string(),
            default_description: "Discover and track high-quality Rust projects.".to_string(),
            default_image: "/assets/ferris.gif".to_string(),
            language: "en".to_string(),
            site_url: String::new(),
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct SEOData {
    pub title: String,
    #[props(default)]
    pub description: Option<String>,
    #[props(default)]
    pub keywords: Option<String>,
    #[props(default)]
    pub og_image: Option<String>,
    #[props(default)]
    pub og_type: Option<String>,
    #[props(default)]
    pub canonical_url: Option<String>,
    #[props(default)]
    pub noindex: Option<bool>,
    #[props(default)]
    pub language: Option<String>,
}

#[derive(Clone, Props, PartialEq, Default)]
pub struct SEOProp {
    pub title: String,
    #[props(default)]
    pub description: String,
    #[props(default)]
    pub keywords: String,
    #[props(default)]
    pub og_image: String,
    #[props(default)]
    pub og_type: String,
    #[props(default)]
    pub canonical_url: String,
    #[props(default)]
    pub noindex: bool,
    #[props(default)]
    pub language: String,
}

impl From<SEOProp> for SEOData {
    fn from(value: SEOProp) -> Self {
        let empty_to_none = |s: String| if s.trim().is_empty() { None } else { Some(s) };
        Self {
            title: value.title,
            description: empty_to_none(value.description),
            keywords: empty_to_none(value.keywords),
            og_image: empty_to_none(value.og_image),
            og_type: empty_to_none(value.og_type),
            canonical_url: empty_to_none(value.canonical_url),
            noindex: value.noindex.then_some(true),
            language: empty_to_none(value.language),
        }
    }
}

fn to_absolute_url(site_url: &str, url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") || site_url.is_empty() {
        return url.to_string();
    }
    let site = site_url.trim_end_matches('/');
    if url.starts_with('/') {
        format!("{site}{url}")
    } else {
        format!("{site}/{url}")
    }
}

#[component]
pub fn SEOHead(data: SEOProp) -> Element {
    let seo_data: SEOData = data.into();
    let seo_context = SEOContext::default_en();
    let config = use_context::<ConfigContext>();
    let route = use_route::<Route>();

    let base_title = seo_data.title.trim();
    let final_title = if base_title.is_empty() {
        seo_context.site_name.clone()
    } else {
        format!("{base_title}{}", seo_context.title_suffix)
    };
    let final_description = seo_data
        .description
        .unwrap_or_else(|| seo_context.default_description.clone());

    let final_lang = seo_data
        .language
        .unwrap_or_else(|| seo_context.language.clone());

    let final_og_type = seo_data.og_type.unwrap_or_else(|| "website".to_string());

    let final_image = seo_data
        .og_image
        .unwrap_or_else(|| seo_context.default_image.clone());
    let final_image = to_absolute_url(&config.site_url, &final_image);

    let route_path = route.to_string();
    let canonical_input = seo_data.canonical_url.unwrap_or(route_path);
    let canonical_url = to_absolute_url(&config.site_url, &canonical_input);
    let robots_content = if seo_data.noindex.unwrap_or(false) {
        "noindex, nofollow"
    } else {
        "index, follow"
    };

    rsx! {
        document::Title { "{final_title}" }
        document::Meta {
            name: "description",
            content: "{final_description}"
        }
        document::Meta {
            name: "robots",
            content: "{robots_content}"
        }
        document::Meta {
            name: "language",
            content: "{final_lang}"
        }
        if let Some(keywords) = seo_data.keywords {
            if !keywords.trim().is_empty() {
                document::Meta {
                    name: "keywords",
                    content: "{keywords}"
                }
            }
        }

        document::Meta {
            property: "og:title",
            content: "{final_title}"
        }
        document::Meta {
            property: "og:description",
            content: "{final_description}"
        }
        document::Meta {
            property: "og:type",
            content: "{final_og_type}"
        }
        document::Meta {
            property: "og:locale",
            content: "{final_lang}"
        }
        document::Meta {
            property: "og:image",
            content: "{final_image}"
        }
        if !canonical_url.is_empty() {
            document::Meta {
                property: "og:url",
                content: "{canonical_url}"
            }
            document::Link {
                rel: "canonical",
                href: "{canonical_url}"
            }
        }
    }
}
