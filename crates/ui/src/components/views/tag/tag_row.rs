use dioxus::prelude::*;

use crate::root::Route;
use crate::types::tags::TagListItemDto;

use super::mini_repo_card::MiniRepoCard;

#[component]
pub fn TagRow(tag: TagListItemDto, index: usize) -> Element {
    let outlined = tag
        .label
        .split(':')
        .next()
        .unwrap_or(tag.label.as_str())
        .to_uppercase();
    let description = tag.description.clone().unwrap_or_else(|| {
        "System classification pending. Data transmission in progress.".to_string()
    });
    let archive_ref = format!("0x{:02x}", index);
    let population = format!("{:03}", tag.repos_total);

    rsx! {
        article { class: "group relative overflow-hidden border-b border-primary-6 px-8 py-24 last:border-0",
            div {
                class: "relative z-0 mb-[-10px] font-mono whitespace-nowrap text-7xl font-bold leading-none tracking-tighter opacity-80 md:mb-[-20px] md:text-[180px]",
                style: "-webkit-text-stroke: 1px color-mix(in oklab, var(--secondary-color) 22%, transparent); color: transparent;",
                "{outlined}"
            }
            div { class: "relative z-10 flex max-w-5xl flex-col gap-10",
                div { class: "flex items-center gap-4",
                    span {
                        class: "text-[12px] font-mono font-bold uppercase tracking-[0.6em]",
                        style: "color: var(--grid-accent);",
                        "#{tag.value}"
                    }
                    div { class: "h-[1px] flex-grow", style: "background-color: color-mix(in oklab, var(--grid-accent) 30%, transparent);" }
                }
                p {
                    class: "max-w-3xl border-l-4 pl-10 py-2 font-serif text-2xl italic leading-relaxed text-secondary-4",
                    style: "border-left-color: color-mix(in oklab, var(--grid-accent) 40%, transparent);",
                    "{description}"
                }
                div { class: "flex items-center gap-10",
                    div { class: "flex flex-col",
                        span { class: "mb-1 text-[10px] font-mono uppercase tracking-widest text-secondary-6",
                            "Population"
                        }
                        span { class: "text-3xl font-mono font-bold text-secondary-1", "{population}" }
                    }
                    div { class: "h-12 w-px bg-primary-6" }
                    div { class: "flex flex-col",
                        span { class: "mb-1 text-[10px] font-mono uppercase tracking-widest text-secondary-6",
                            "Archive_Ref"
                        }
                        span { class: "text-3xl font-mono font-bold text-secondary-4", "{archive_ref}" }
                    }
                    Link {
                        class: "ml-auto border px-3 py-1 text-xs font-mono tracking-wider transition-colors",
                        style: "border-color: var(--grid-accent); color: var(--grid-accent); background-color: color-mix(in oklab, var(--grid-accent) 10%, transparent);",
                        to: Route::RepoListView { tags: Some(tag.value.clone()) },
                        "OPEN"
                    }
                }
                div { class: "pt-4",
                    div { class: "mb-6 flex items-center gap-4",
                        div { class: "text-[10px] font-mono uppercase tracking-[0.4em] text-secondary-6",
                            "Node_Assets"
                        }
                        div { class: "h-px flex-grow border-t border-dashed border-primary-6" }
                    }
                    div { class: "flex flex-wrap justify-start gap-2",
                        for repo in tag.top_repos.into_iter().take(10) {
                            MiniRepoCard { key: "{repo.repo_id}", repo }
                        }
                    }
                }
            }
        }
    }
}
