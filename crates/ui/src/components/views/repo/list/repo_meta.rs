use dioxus::prelude::*;

use super::{RepoListContext, RepoListHeroType};

#[component]
pub(super) fn RepoMeta() -> Element {
    let ctx = use_context::<RepoListContext>();
    let hero_type = if (ctx.active_tags)().is_empty() {
        RepoListHeroType::AllProjects
    } else {
        RepoListHeroType::SearchResult
    };

    rsx! {
        div { class: "max-w-3xl",
            h1 { class: "text-2xl md:text-3xl font-black font-sans uppercase tracking-tight text-secondary-2 mb-2 flex flex-wrap items-center gap-2",
                if hero_type == RepoListHeroType::SearchResult {
                    "Search Result"
                } else {
                    "All Project"
                }
            }
            p { class: "text-secondary-3 text-sm md:text-base leading-relaxed font-mono italic",
                "A comprehensive catalog of the Rust ecosystem. Monitor growth, track updates, and discover foundational codebases."
            }
        }
    }
}
