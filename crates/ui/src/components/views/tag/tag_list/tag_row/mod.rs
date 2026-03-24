use crate::components::icons::FishingHookIcon;
use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::root::Route;
use crate::types::tags::TagListItemDto;

use mini_repo_card::MiniRepoCard;

mod mini_repo_card;

#[component]
pub(super) fn TagRow(
    tag: TagListItemDto,
    index: usize,
    current_page: u32,
    page_size: u32,
) -> Element {
    let outlined = tag
        .label
        .split(':')
        .next()
        .unwrap_or(tag.label.as_str())
        .to_uppercase();
    let description = tag
        .description
        .clone()
        .unwrap_or_else(|| t!("view_tag_list_tag_row_pending_description").to_string());

    let rank = (current_page.saturating_sub(1) as usize) * page_size as usize + index + 1;
    let rank_text = format!("{:03}", rank);
    let population = format!("{:03}", tag.repos_total);

    rsx! {
        article {
            id: "{tag.value}",
            class: "group relative overflow-hidden py-6 md:px-8 md:py-12 last:border-0 hover:cursor-pointer scroll-mt-20",
            div {
                class: "relative z-0 max-w-full overflow-hidden text-ellipsis font-mono whitespace-nowrap text-[30px] font-bold leading-none tracking-wide text-transparent opacity-50 transition-all duration-300 md:mb-[-10px] md:text-[120px] [-webkit-text-stroke:1px_var(--secondary-color-5)] group-hover:opacity-80 group-hover:[-webkit-text-stroke:3px_var(--grid-accent)]",
                "{outlined}"
            }
            div { class: "relative z-10 flex flex-col gap-4 px-0 md:px-20 md:gap-6",
                div { class: "flex items-center gap-2 md:gap-4",
                    span {
                        class: "text-[10px] font-mono font-bold uppercase tracking-[0.28em] text-grid-accent md:text-[12px] md:tracking-[0.6em]",
                        "#{tag.value}"
                    }
                    div { class: "h-0.5 flex-grow bg-grid-accent/30 transition-all duration-300 group-hover:h-1" }
                }
                p {
                    class: "max-w-3xl border-l-2 border-grid-accent/30 py-1.5 pl-3 font-sans text-base italic leading-relaxed text-secondary/30 group-hover:text-secondary/50 md:border-l-4 md:py-2 md:pl-10 md:text-2xl",
                    "{description}"
                }
                div { class: "pt-0",
                    div { class: "flex flex-col items-stretch gap-4 md:h-24 md:flex-row md:gap-10",
                        div { class: "flex items-stretch gap-5 self-stretch md:gap-10",
                            div { class: "flex h-full flex-col",
                                span { class: "text-[10px] font-mono uppercase tracking-widest text-secondary-6",
                                    {t!("view_tag_list_tag_row_rank")}
                                }
                                div { class: "flex flex-1 items-center",
                                    span { class: "text-2xl font-mono font-bold text-secondary-4 md:text-3xl", "{rank_text}" }
                                }
                            }
                            div { class: "h-full w-px bg-primary-6" }
                            div { class: "flex h-full flex-col",
                                span { class: "text-[10px] font-mono uppercase tracking-widest text-secondary-6",
                                    {t!("view_tag_list_tag_row_count")}
                                }
                                div { class: "flex flex-1 items-center",
                                    span { class: "text-2xl font-mono font-bold text-secondary-1 md:text-3xl", "{population}" }
                                }
                            }
                        }
                        div { class: "min-w-0 flex-1 self-stretch flex flex-col",
                            div { class: "flex items-center gap-2 md:gap-4",
                                div { class: "text-[10px] font-mono uppercase tracking-[0.4em] text-secondary-6",
                                    "TOP10"
                                }
                                div { class: "flex-grow border-t-2 border-dashed border-primary-6 md:border-t-4" }
                            }
                            div { class: "flex flex-1 flex-wrap content-center items-center justify-start gap-1.5 md:gap-2",
                                for (repo_idx, repo) in tag.top_repos.into_iter().take(10).enumerate() {
                                    MiniRepoCard { key: "{repo.repo_id}:{repo_idx}", repo }
                                }
                            }
                        }
                        Link {
                            class: "ml-auto hidden self-start text-primary-6 transition-colors duration-300 hover:text-grid-accent group-hover:text-grid-accent md:block",
                            to: Route::RepoListView {
                                tags: Some(tag.value.clone()),
                                metric: None,
                                range: None,
                                page: None,
                                size: None,
                            },
                            FishingHookIcon { width: 108, height: 108 }
                        }
                        Link {
                            class: "mt-1 inline-flex items-center gap-2 self-start text-primary-6 transition-colors duration-300 hover:text-grid-accent group-hover:text-grid-accent md:hidden",
                            to: Route::RepoListView {
                                tags: Some(tag.value.clone()),
                                metric: None,
                                range: None,
                                page: None,
                                size: None,
                            },
                            FishingHookIcon { width: 44, height: 44 }
                            span { class: "text-[10px] font-mono uppercase tracking-[0.22em]",
                                {t!("view_tag_list_tag_row_view_repos")}
                            }
                        }
                    }
                }
            }
        }
    }
}
