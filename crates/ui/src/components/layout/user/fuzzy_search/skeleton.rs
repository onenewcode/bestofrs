use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::skeleton::Skeleton;
use crate::components::{common::TypingText, icons::BORSFerrisIcon};
use crate::root::Route;

use super::{FuzzySearchCachedResult, FuzzySearchResultList};

#[component]
pub(super) fn FuzzySearchIdleFallback() -> Element {
    rsx! {
        div { class: "inset-0 p-8 w-full h-full text-grid-accent flex flex-col items-center justify-center gap-3",
            BORSFerrisIcon { width:240.0 }
            h3 { class: "text-base font-semibold text-secondary-2 text-center", {t!("layout_user_fuzzy_search_fallback_hi_ferris")} }
            div { class: "w-full max-w-md text-left text-sm text-secondary-5",
                TypingText { text: t!("layout_user_fuzzy_search_fallback_typing").to_string(), active: true }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct FuzzySearchCachedFallbackProps {
    pub cached: Option<FuzzySearchCachedResult>,
    pub on_repo_select: Callback<Route>,
    pub on_tag_select: Callback<(String, String)>,
}

#[component]
pub(super) fn FuzzySearchCachedFallback(props: FuzzySearchCachedFallbackProps) -> Element {
    if let Some(cached) = props.cached {
        rsx! {
            FuzzySearchResultList {
                repo_state: cached.repos,
                tag_state: cached.tags,
                on_repo_select: props.on_repo_select,
                on_tag_select: props.on_tag_select,
                on_repo_prefetch: |_| {},
                on_tag_prefetch: |_| {},
                on_repo_retry: |_| {},
                on_tag_retry: |_| {},
                allow_load_more: false,
            }
        }
    } else {
        rsx! {
            Skeleton { class: "skeleton w-full h-full min-h-[120px]" }
        }
    }
}
