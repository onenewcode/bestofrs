use dioxus::prelude::*;

use crate::components::common::{
    CommonMarkdown, GradientDirection, GridBackground, GridPadding, GridPattern,
    GridSlashTransition, GridType, GridWrapper, SEOHead, SEOProp,
};

const ABOUT_MD: &str = include_str!("./about.md");

#[component]
pub fn About() -> Element {
    rsx! {
        SEOHead {
            data: SEOProp {
                title: "About".into(),
                description: "Best of RS is a curated Rust ecosystem tracker that follows stars, forks, contributors, issues, and trend snapshots to help users and maintainers understand project momentum.".into(),
                keywords: "best of rs, about best of rs, rust ecosystem, rust open source tracking, rust project trends, github rust metrics".into(),
                canonical_url: "/about".into(),
                og_type: "website".into(),
                ..Default::default()
            },
        }
        GridSlashTransition { }
        GridWrapper {
            grid_type: GridType::Default,
            padding: GridPadding::Lg,
            is_dot_on: true,
            background: GridBackground {
                pattern: GridPattern::Grid,
                gradient: GradientDirection::ToBottom,
            },
            section { class: "min-h-screen",
                CommonMarkdown {
                    src: ABOUT_MD.to_string(),
                    class: Some("max-w-4xl p-6 md:p-10".to_string()),
                }
            }
        }
        GridSlashTransition {  }
    }
}
