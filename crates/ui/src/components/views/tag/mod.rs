use dioxus::prelude::*;

use crate::components::common::{
    GradientDirection, GridBackground, GridPadding, GridPattern, GridSlashTransition, GridType,
    GridWrapper,
};
use crate::IO::repos::list_tags_with_meta;

mod mini_repo_card;
mod tag_row;

use tag_row::TagRow;

#[component]
pub fn TagList() -> Element {
    let mut page_size = use_signal(|| 20u32);
    let mut current_page = use_signal(|| 1u32);
    let tags = use_server_future(move || {
        list_tags_with_meta(Some(current_page()), Some(page_size()), None, Some(5))
    })?;

    rsx! {
        GridWrapper {
            bg_class: "opacity-60",
            grid_type: GridType::Default,
            padding: GridPadding::Lg,
            is_dot_on: true,
            background: GridBackground {
                pattern: GridPattern::Dot,
                gradient: GradientDirection::ToBottom,
            },
            section { class: "relative overflow-hidden",
                div { class: "relative z-10 space-y-6",
                    div { class: "inline-flex items-center gap-2 border border-primary-6 bg-primary-1 px-2 py-1 font-mono text-xs tracking-[0.3em] text-secondary-4 uppercase",
                        "tag_archive"
                    }
                    h1 { class: "text-4xl md:text-6xl font-bold tracking-tight text-secondary-1",
                        "Taxonomy "
                        span { style: "color: var(--grid-accent);", "Atlas" }
                    }
                    p {
                        class: "max-w-3xl border-l-2 pl-5 text-base leading-relaxed text-secondary-4 font-serif italic",
                        style: "border-left-color: color-mix(in oklab, var(--grid-accent) 50%, transparent);",
                        "System classification index with repository clusters. Select a tag to jump into filtered exploration."
                    }
                }
                section { class: "space-y-6",
                    div { class: "flex items-center gap-3",
                        span { class: "text-sm font-medium text-secondary-5", "page size" }
                        select {
                            class: "border border-primary-6 bg-primary-1 px-2 py-1 text-sm text-secondary-3",
                            value: "{page_size()}",
                            onchange: move |e| {
                                if let Ok(v) = e.value().parse::<u32>() {
                                    page_size.set(v);
                                    current_page.set(1);
                                }
                            },
                            option { value: "10", "10" }
                            option { value: "20", "20" }
                            option { value: "50", "50" }
                        }
                    }

                    match tags() {
                        Some(Ok(page)) => {
                            rsx! {
                                div { class: "flex items-center justify-between gap-3 border border-primary-6 bg-primary-1 px-4 py-3",
                                    div { class: "text-xs font-mono tracking-wide text-secondary-5",
                                        "TAGS: "
                                        span { class: "font-semibold text-secondary-1", "{page.meta.total}" }
                                    }
                                    div { class: "text-xs font-mono tracking-wide text-secondary-5",
                                        "PAGE: "
                                        span { class: "font-semibold text-secondary-1",
                                            "{page.meta.current_page}/{page.meta.total_pages.max(1)}"
                                        }
                                    }
                                }


                                if page.items.is_empty() {
                                    div { class: "flex min-h-[220px] flex-col items-center justify-center border border-dashed border-primary-6 bg-primary-1 text-center",
                                        span { class: "mb-3 font-mono text-sm tracking-widest text-secondary-6", "NO_DATA" }
                                        span { class: "text-sm text-secondary-6", "No tags found" }
                                    }
                                } else {
                                    div {
                                        for (index , tag) in page.items.into_iter().enumerate() {
                                            TagRow {
                                                key: "{tag.label}:{tag.value}",
                                                tag,
                                                index,
                                                current_page: current_page(),
                                                page_size: page_size(),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(Err(e)) => rsx! {
                            div { class: "border border-primary-error bg-primary p-4 text-sm text-primary-error",
                                "{e}"
                            }
                        },
                        None => rsx! {
                            div { class: "text-sm text-secondary-6", "Loading..." }
                        },
                    }
                }

            }
        }
    }
}
