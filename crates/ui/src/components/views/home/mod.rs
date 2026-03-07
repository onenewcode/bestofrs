use dioxus::prelude::*;

use crate::components::common::IOCell;
use crate::root::Route;
use crate::types::repos::RepoDto;
use crate::IO::repos::list_repos;
use app::prelude::Pagination;
const CAT_IMAGE: Asset = asset!(
    "/assets/ferris.gif",
    AssetOptions::builder().with_hash_suffix(false)
);

#[derive(Clone, Copy, PartialEq, Eq)]
enum RankType {
    Star,
    Fork,
    Issue,
    Recent,
}

#[component]
fn HomeFaqSection() -> Element {
    rsx! {
        div { class: "w-full max-w-7xl px-8 mb-32 relative z-10",
            div { class: "border-t border-primary-6 pt-20",
                div { class: "flex items-center gap-3 mb-8",
                    div { class: "w-8 h-[1px] bg-secondary-6" }
                    span { class: "font-mono text-[10px] tracking-[0.5em] uppercase text-secondary-6 font-bold", "FAQ" }
                }
                h3 { class: "text-4xl md:text-5xl font-black font-sans uppercase tracking-tighter italic text-secondary-1 leading-none mb-14",
                    "Frequently Asked"
                    br {}
                    span { class: "text-secondary-6 not-italic", "Questions" }
                }
                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-x-12 gap-y-16",
                    HomeFaqItem {
                        index: 1,
                        question: "How are projects ranked?",
                        answer: "Projects are ranked based on their GitHub star growth over the selected period (daily, weekly, monthly). This highlights what's currently trending in the Rust community.",
                    }
                    HomeFaqItem {
                        index: 2,
                        question: "How to add a project?",
                        answer: "If you know a great Rust project that's missing, use the recommend link in the archive pages. We curate the list to ensure high quality and relevance.",
                    }
                    HomeFaqItem {
                        index: 3,
                        question: "Is this official?",
                        answer: "Best of Rust is a community-driven project inspired by Best of JS. It is not an official Rust Foundation project.",
                    }
                }
            }
        }
    }
}

#[component]
fn HomeActionSection() -> Element {
    rsx! {
        div { class: "w-full max-w-7xl px-8 mb-24 relative z-10",
            div { class: "border-t border-primary-6 pt-16 flex flex-col md:flex-row md:items-center md:justify-between gap-8",
                div { class: "space-y-3",
                    div { class: "flex items-center gap-3",
                        div { class: "w-8 h-[1px] bg-secondary-6" }
                        span { class: "font-mono text-[10px] tracking-[0.5em] uppercase text-secondary-6 font-bold", "Action" }
                    }
                    h4 { class: "text-3xl md:text-4xl font-black font-sans uppercase tracking-tighter italic text-secondary-1 leading-none",
                        "Support the"
                        span { class: "text-secondary-6 not-italic", " Archive" }
                    }
                    p { class: "text-sm text-secondary-4 font-serif italic leading-relaxed max-w-2xl",
                        "If Best of Rust is useful to you, star the project or help us add more great repositories."
                    }
                }
                div { class: "flex flex-wrap items-center gap-5",
                    a {
                        href: "https://github.com/zhiyanzhaijie/bestofrs",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "relative group",
                        div { class: "absolute inset-0 rounded-full bg-primary-1 border-2 border-primary-6 translate-x-[10px] translate-y-[10px] transition-all duration-300 group-hover:border-focused-border" }
                        div { class: "relative flex items-center gap-3 px-8 py-3 rounded-full bg-primary border-4 border-secondary-2 text-secondary-2 group-hover:bg-secondary-2 group-hover:text-primary group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] transition-all duration-300 ease-out",
                            span { class: "font-black font-sans text-sm uppercase tracking-[0.2em] italic", "Star_On_GitHub" }
                            span { class: "group-hover:translate-x-1 transition-transform", "★" }
                        }
                    }
                    a {
                        href: "https://github.com/zhiyanzhaijie/bestofrs/issues/new",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "relative group",
                        div { class: "absolute inset-0 rounded-full bg-primary-1 border-2 border-primary-6 translate-x-[10px] translate-y-[10px] transition-all duration-300 group-hover:border-focused-border" }
                        div { class: "relative flex items-center gap-3 px-8 py-3 rounded-full bg-primary border-4 border-secondary-2 text-secondary-2 group-hover:bg-secondary-2 group-hover:text-primary group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] transition-all duration-300 ease-out",
                            span { class: "font-black font-sans text-sm uppercase tracking-[0.2em] italic", "Recommend_A_Repo" }
                            span { class: "group-hover:translate-x-1 transition-transform", "→" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct HomeFaqItemProps {
    index: usize,
    question: &'static str,
    answer: &'static str,
}

#[component]
fn HomeFaqItem(props: HomeFaqItemProps) -> Element {
    rsx! {
        div { class: "space-y-4",
            h4 { class: "font-black font-sans uppercase tracking-tight text-secondary-2 flex items-center gap-2",
                span { class: "text-secondary-6 font-mono text-xs", "Q{props.index}." }
                "{props.question}"
            }
            p { class: "text-sm text-secondary-4 font-serif italic leading-relaxed",
                "{props.answer}"
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TimeRange {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Clone, PartialEq, Eq)]
struct HomeRankRepo {
    id: String,
    name: String,
    description: String,
    tags: Vec<String>,
    avatar_url: String,
    stars: i64,
    forks: i64,
    issues: i64,
    updated_at: String,
}

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "min-h-screen bg-primary flex flex-col items-center selection:bg-secondary-2 selection:text-primary relative transition-colors duration-300",
            div { class: "hero-block absolute top-0 left-1/2 -translate-x-1/2 w-screen h-[85vh] z-0 flex items-center transition-colors duration-300 overflow-visible",
                div { class: "absolute inset-0 z-10 overflow-hidden bg-primary transition-colors duration-300",
                    div { class: "absolute inset-0 z-0",
                        img {
                            src: CAT_IMAGE,
                            alt: "Background Animation",
                            class: "w-full h-full object-contain opacity-40 mix-blend-multiply pointer-events-none transition-all"
                        }
                        div { class: "absolute inset-0", style: "background: color-mix(in oklab, var(--primary-color) 20%, transparent);" }
                    }
                    div { class: "absolute inset-y-0 left-0 w-1/4 z-10", style: "background: linear-gradient(to right, var(--primary-color), color-mix(in oklab, var(--primary-color) 90%, transparent), transparent);" }
                    div { class: "absolute inset-y-0 right-0 w-1/6 z-10", style: "background: linear-gradient(to left, var(--primary-color), color-mix(in oklab, var(--primary-color) 90%, transparent), transparent);" }
                    div { class: "absolute inset-x-0 top-0 h-32 z-10", style: "background: linear-gradient(to bottom, var(--primary-color), transparent);" }
                    div { class: "absolute inset-x-0 bottom-0 h-32 z-10", style: "background: linear-gradient(to top, var(--primary-color), transparent);" }
                }
                div { class: "max-w-7xl mx-auto px-8 relative z-20 w-full",
                    div { class: "max-w-3xl",
                        div { class: "flex items-center gap-3 mb-8",
                            div { class: "w-12 h-[1px] bg-secondary-6" }
                            span { class: "font-mono text-[10px] tracking-[0.5em] uppercase text-secondary-6 font-bold",
                                "System_Archive_v.2.6"
                            }
                        }
                        h1 { class: "text-7xl md:text-[120px] font-black font-sans leading-[0.8] tracking-tighter uppercase mb-10 italic text-secondary-1",
                            "Best Of"
                            br {}
                            span { class: "text-secondary-6 not-italic", "Rust" }
                        }
                        p { class: "text-xl md:text-2xl text-secondary-4 font-serif italic leading-relaxed max-w-xl mb-12",
                            "A high-density archival transmission of the most exceptional projects in the Rust ecosystem. Curated for technical precision."
                        }
                        div { class: "flex items-center gap-8",
                            Link {
                                to: Route::RepoListView { tags: None },
                                class: "group flex items-center gap-4 text-sm font-mono font-bold uppercase tracking-[0.3em] text-secondary-2 hover:text-secondary-6 transition-colors",
                                "Access Archive"
                                span { class: "group-hover:translate-x-2 transition-transform", "→" }
                            }
                            div { class: "h-4 w-[1px] bg-primary-6" }
                            div { class: "text-[10px] font-mono text-secondary-5 uppercase tracking-widest",
                                "// Transmission_Active"
                            }
                        }
                    }
                }
            }

            div { class: "h-[65vh] w-full" }

            div { class: "w-full max-w-7xl px-8 relative z-30 mt-20 pb-32",
                div { class: "absolute inset-x-8 bottom-24 top-0 bg-screentone opacity-10 pointer-events-none z-0 rounded-[3.5rem]" }
                IOCell {
                    HomeRankPanel {}
                }
            }
            HomeFaqSection {}
            HomeActionSection {}

        }
    }
}

fn row_border_style(index: usize) -> String {
    let accent = rainbow_color(index);
    format!(
        "border-left-color: color-mix(in oklab, {accent} 86%, var(--secondary-color-2));\
         box-shadow: inset 0 0 0 1px color-mix(in oklab, {accent} 22%, var(--primary-color-6));\
         background: linear-gradient(90deg, color-mix(in oklab, {accent} 8%, var(--primary-color)) 0%, var(--primary-color) 28%);"
    )
}

fn rainbow_color(index: usize) -> &'static str {
    match index % 7 {
        0 => "#ef4444",
        1 => "#f97316",
        2 => "#eab308",
        3 => "#22c55e",
        4 => "#3b82f6",
        5 => "#6366f1",
        _ => "#a855f7",
    }
}

#[component]
fn HomeRankPanel() -> Element {
    let mut active_tab = use_signal(|| RankType::Star);
    let mut time_range = use_signal(|| TimeRange::Weekly);
    let repos = use_server_future(move || {
        list_repos(Pagination {
            limit: Some(7),
            offset: Some(0),
        })
    })?;
    match repos() {
        Some(Ok(page)) => {
            let base_list = page
                .items
                .into_iter()
                .map(map_rank_repo)
                .collect::<Vec<_>>();
            let mut star_rank = base_list.clone();
            star_rank.sort_by(|a, b| b.stars.cmp(&a.stars));
            let mut fork_rank = base_list.clone();
            fork_rank.sort_by(|a, b| b.forks.cmp(&a.forks));
            let mut issue_rank = base_list.clone();
            issue_rank.sort_by(|a, b| b.issues.cmp(&a.issues));
            let mut recent_rank = base_list.clone();
            recent_rank.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
            let current_list = match active_tab() {
                RankType::Star => star_rank,
                RankType::Fork => fork_rank,
                RankType::Issue => issue_rank,
                RankType::Recent => recent_rank,
            };
            rsx! {
                div { class: "bg-primary border border-2 border-x-4 border-primary-6 shadow-2xl rounded-[3.5rem] overflow-hidden flex flex-col lg:flex-row min-h-[600px] transition-colors duration-300 relative z-10",
                    div { class: "w-full lg:w-[260px] flex flex-col bg-primary border-r border-primary-6 self-stretch p-4",
                        div { class: "p-4 mb-2",
                            h3 { class: "text-2xl font-black font-sans uppercase tracking-tighter italic text-secondary-1", "Rankings" }
                            p { class: "text-[10px] font-mono text-secondary-6 uppercase tracking-widest mt-1 font-bold", "Metric_Select" }
                        }
                        div { class: "flex flex-col flex-grow gap-2 overflow-hidden",
                            for (idx, tab) in [RankType::Star, RankType::Fork, RankType::Issue, RankType::Recent].into_iter().enumerate() {
                                HomeRankTabItem {
                                    idx,
                                    tab,
                                    active_tab: active_tab(),
                                    on_select: move |_| active_tab.set(tab),
                                }
                            }
                        }
                        div { class: "mt-auto p-6 border-t border-primary-6",
                            div { class: "flex items-center gap-2 text-[8px] font-mono text-secondary-5 uppercase tracking-widest",
                                div { class: "w-1 h-1 bg-secondary-6 rounded-full animate-pulse" }
                                "Active"
                            }
                        }
                    }
                    div { class: "w-full lg:flex-grow p-5 md:p-6 bg-primary-1/60 flex flex-col self-stretch m-4 rounded-[2.5rem]",
                        div { class: "flex flex-col xl:flex-row items-start xl:items-center justify-between mb-6 pb-4 border-b-2 border-primary-6 gap-4",
                            div { class: "flex flex-wrap items-center gap-6",
                                if active_tab() != RankType::Recent {
                                    for range in [TimeRange::Daily, TimeRange::Weekly, TimeRange::Monthly] {
                                        HomeTimeRangeButton {
                                            range,
                                            active: time_range() == range,
                                            onclick: move |_| time_range.set(range),
                                        }
                                    }
                                } else {
                                    div { class: "relative group",
                                        div { class: "absolute inset-0 rounded-full bg-primary-1 border-2 border-primary-6 translate-x-[10px] translate-y-[10px]" }
                                        div { class: "relative px-8 py-3 rounded-full text-sm font-black font-sans uppercase tracking-[0.2em] italic bg-secondary-2 text-primary border-4 border-secondary-2 translate-x-[3.82px] translate-y-[3.82px] shadow-[0_0_20px_color-mix(in_oklab,var(--grid-accent)_24%,transparent)]",
                                            "Latest_Transmissions"
                                        }
                                    }
                                }
                            }
                            Link { to: Route::RepoListView { tags: None }, class: "relative group",
                                div { class: "absolute inset-0 rounded-full bg-primary-1 border-2 border-primary-6 translate-x-[10px] translate-y-[10px] transition-all duration-300 group-hover:border-focused-border" }
                                div { class: "relative flex items-center gap-3 px-8 py-3 rounded-full bg-primary border-4 border-secondary-2 text-secondary-2 group-hover:bg-secondary-2 group-hover:text-primary group-hover:translate-x-[3.82px] group-hover:translate-y-[3.82px] transition-all duration-300 ease-out",
                                    span { class: "font-black font-sans text-sm uppercase tracking-[0.2em] italic", "View_All" }
                                    span { class: "group-hover:translate-x-1 transition-transform", "→" }
                                }
                            }
                        }

                        div { class: "flex flex-col gap-2.5",
                            for (idx, repo) in current_list.into_iter().enumerate() {
                                HomeRankRepoRow {
                                    idx,
                                    repo,
                                    active_tab: active_tab(),
                                }
                            }
                        }

                    }
                }
            }
        }
        Some(Err(e)) => rsx! {
            div { class: "bg-primary border border-primary-6 shadow-2xl rounded-[3.5rem] overflow-hidden p-6 text-sm text-primary-error relative z-10",
                "{e}"
            }
        },
        None => rsx! {},
    }
}

#[derive(Props, Clone, PartialEq)]
struct HomeRankTabItemProps {
    idx: usize,
    tab: RankType,
    active_tab: RankType,
    on_select: EventHandler<MouseEvent>,
}

#[component]
fn HomeRankTabItem(props: HomeRankTabItemProps) -> Element {
    let is_active = props.active_tab == props.tab;
    rsx! {
        div {
            class: if is_active {
                "rounded-2xl transition-all duration-500 relative flex flex-col bg-primary-1 shadow-sm flex-grow"
            } else {
                "rounded-2xl transition-all duration-500 relative flex flex-col bg-transparent hover:bg-primary-1/40"
            },
            if is_active {
                div { class: "absolute left-0 top-0 bottom-0 w-1 bg-secondary-6" }
            }
            button {
                onclick: move |e| props.on_select.call(e),
                class: "w-full px-6 py-6 flex items-center justify-between group text-left relative z-10",
                div { class: "flex flex-col items-start",
                    span { class: "text-[10px] font-mono uppercase tracking-[0.2em] text-secondary-5 mb-1",
                        "M_0{props.idx + 1}"
                    }
                    span {
                        class: if is_active {
                            "text-lg font-black font-sans uppercase tracking-widest transition-colors text-secondary-2"
                        } else {
                            "text-lg font-black font-sans uppercase tracking-widest transition-colors text-secondary-5 group-hover:text-secondary-2"
                        },
                        "{rank_title(props.tab)}"
                    }
                }
                span {
                    class: if is_active {
                        "transition-all duration-300 rotate-90 text-secondary-6"
                    } else {
                        "transition-all duration-300 text-primary-6"
                    },
                    "›"
                }
            }
            div {
                class: if is_active {
                    "px-6 flex-grow flex items-start overflow-hidden transition-all duration-700 ease-in-out opacity-100 pb-12"
                } else {
                    "px-6 flex-grow flex items-start overflow-hidden transition-all duration-700 ease-in-out max-h-0 opacity-0"
                },
                div { class: "relative pt-2",
                    div { class: "absolute -left-3 top-0 bottom-0 w-[2px]", style: "background: color-mix(in oklab, var(--grid-accent) 30%, transparent);" }
                    p { class: "text-sm text-secondary-4 font-serif italic leading-relaxed pl-4",
                        "{rank_desc(props.tab)}"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct HomeTimeRangeButtonProps {
    range: TimeRange,
    active: bool,
    onclick: EventHandler<MouseEvent>,
}

#[component]
fn HomeTimeRangeButton(props: HomeTimeRangeButtonProps) -> Element {
    rsx! {
        div { class: "relative group",
            div {
                class: if props.active {
                    "absolute inset-0 rounded-full border-2 transition-all duration-300 translate-x-[10px] translate-y-[10px] border-focused-border"
                } else {
                    "absolute inset-0 rounded-full border-2 transition-all duration-300 translate-x-[10px] translate-y-[10px] bg-primary-1 border-primary-6 group-hover:border-focused-border"
                }
            }
            button {
                onclick: move |e| props.onclick.call(e),
                class: if props.active {
                    "relative px-8 py-3 rounded-full text-sm font-black font-sans uppercase tracking-[0.2em] italic border-4 transition-all duration-300 ease-out bg-secondary-2 border-secondary-2 text-primary translate-x-[3.82px] translate-y-[3.82px] shadow-[0_0_20px_color-mix(in_oklab,var(--grid-accent)_30%,transparent)]"
                } else {
                    "relative px-8 py-3 rounded-full text-sm font-black font-sans uppercase tracking-[0.2em] italic border-4 transition-all duration-300 ease-out bg-primary border-secondary-2 text-secondary-2 hover:border-focused-border hover:text-secondary-6 hover:translate-x-[3.82px] hover:translate-y-[3.82px] hover:shadow-[0_0_20px_color-mix(in_oklab,var(--grid-accent)_22%,transparent)]"
                },
                "{time_range_text(props.range)}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct HomeRankRepoRowProps {
    idx: usize,
    repo: HomeRankRepo,
    active_tab: RankType,
}

#[component]
fn HomeRankRepoRow(props: HomeRankRepoRowProps) -> Element {
    let route = parse_repo_route(&props.repo.id);
    let stat_text = stat_value(&props.repo, props.active_tab);
    let accent_color = rainbow_color(props.idx);
    let tag_items = props.repo.tags.iter().take(3).cloned().collect::<Vec<_>>();
    let more_tags = props.repo.tags.len().saturating_sub(3);
    let border_left_style = row_border_style(props.idx);
    let card_style = format!("{border_left_style} --rank-accent: {accent_color};");
    let rank_no_style =
        format!("color: color-mix(in oklab, {accent_color} 46%, var(--secondary-color-6));");
    let avatar_style =
        format!("border-color: color-mix(in oklab, {accent_color} 56%, var(--primary-color-6));");
    let stat_icon_style =
        format!("color: color-mix(in oklab, {accent_color} 84%, var(--secondary-color-2));");
    let tag_chip_style = format!(
        "border-color: color-mix(in oklab, {accent_color} 34%, var(--primary-color-6));\
         background: color-mix(in oklab, {accent_color} 10%, var(--primary-color));"
    );
    let tag_more_style =
        format!("color: color-mix(in oklab, {accent_color} 76%, var(--secondary-color-4));");
    let card_class = format!(
        "rank-card bg-primary border-l-4 border-y border-r border-primary-6 shadow-sm transition-all duration-300 flex items-center p-3 group relative overflow-hidden rounded-2xl"
    );
    let detail = rsx! {
        div { class: "{card_class}", style: "{card_style}",
            div { class: "rank-card-number w-10 flex-shrink-0 font-mono font-bold transition-colors text-xl", style: "{rank_no_style}",
                "{(props.idx + 1).to_string()}"
            }
            div { class: "relative mr-6",
                img {
                    src: "{props.repo.avatar_url}",
                    alt: "{props.repo.name}",
                    class: "rank-card-avatar w-9 h-9 rounded-full border border-primary-6 grayscale group-hover:grayscale-0 transition-all duration-500",
                    style: "{avatar_style}",
                    referrerpolicy: "no-referrer"
                }
            }
            div { class: "flex-grow min-w-0 mr-6",
                h4 { class: "rank-card-title font-black text-sm font-sans uppercase tracking-tight text-secondary-2 transition-colors line-clamp-1",
                    "{props.repo.name}"
                }
                p { class: "text-[11px] text-secondary-5 font-serif italic line-clamp-1 mt-0",
                    "{props.repo.description}"
                }
                if !tag_items.is_empty() {
                    div { class: "mt-1.5 flex flex-wrap items-center gap-1",
                        for tag in tag_items {
                            span {
                                class: "rank-card-tag inline-flex items-center rounded-full border px-1.5 py-0.5 text-[9px] font-mono uppercase tracking-wide text-secondary-4",
                                style: "{tag_chip_style}",
                                "{tag}"
                            }
                        }
                        if more_tags > 0 {
                            span { class: "text-[10px] font-mono uppercase tracking-wide", style: "{tag_more_style}",
                                "+{more_tags}"
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col items-end gap-1 flex-shrink-0",
                div { class: "text-sm font-mono font-bold text-secondary-2 flex items-center gap-1.5",
                    "{stat_text}"
                    span { class: "rank-card-icon", style: "{stat_icon_style}", "{stat_icon(props.active_tab)}" }
                }
                div { class: "rank-card-tail w-4 h-[1px] bg-primary-6 group-hover:w-8 transition-all duration-500" }
            }
            div { class: "absolute inset-0 bg-screentone opacity-[0.01] pointer-events-none" }
        }
    };
    if let Some(route) = route {
        rsx! {
            Link { to: route, class: "contents", {detail} }
        }
    } else {
        rsx! { {detail} }
    }
}

fn map_rank_repo(repo: RepoDto) -> HomeRankRepo {
    let name = repo.full_name.clone().unwrap_or_else(|| repo.id.clone());
    let description = repo
        .description
        .clone()
        .unwrap_or_else(|| "No description".to_string());
    let avatar_url = repo
        .avatar_url
        .clone()
        .or_else(|| repo.avatar_urls.first().cloned())
        .unwrap_or_else(|| fallback_avatar(&repo.id));
    let updated_at = repo
        .last_fetched_at
        .clone()
        .unwrap_or_else(|| "1970-01-01".to_string());
    HomeRankRepo {
        id: repo.id,
        name,
        description,
        tags: repo
            .tags
            .iter()
            .map(|tag| {
                if tag.label.is_empty() {
                    tag.value.clone()
                } else {
                    tag.label.clone()
                }
            })
            .collect(),
        avatar_url,
        stars: repo.stars,
        forks: repo.forks,
        issues: repo.open_issues,
        updated_at,
    }
}

fn fallback_avatar(repo_id: &str) -> String {
    if let Some((owner, _)) = repo_id.split_once('/') {
        if !owner.is_empty() {
            return format!("https://github.com/{owner}.png");
        }
    }
    "https://github.com/github.png".to_string()
}

fn parse_repo_route(repo_id: &str) -> Option<Route> {
    let (owner, name) = repo_id.split_once('/')?;
    if owner.is_empty() || name.is_empty() {
        return None;
    }
    Some(Route::RepoDetailView {
        owner: owner.to_string(),
        name: name.to_string(),
    })
}

fn rank_title(tab: RankType) -> &'static str {
    match tab {
        RankType::Star => "stars",
        RankType::Fork => "forks",
        RankType::Issue => "issues",
        RankType::Recent => "Recent",
    }
}

fn rank_desc(tab: RankType) -> &'static str {
    match tab {
        RankType::Star => {
            "Highest community validation and visibility. The gold standard of Rust excellence."
        }
        RankType::Fork => {
            "Most active foundations for extension. High-utility codebases built for growth."
        }
        RankType::Issue => {
            "High-velocity development environments. Active problem-solving and iteration."
        }
        RankType::Recent => {
            "Freshly tracked transmissions. The latest additions to the manuscript archive."
        }
    }
}

fn time_range_text(range: TimeRange) -> &'static str {
    match range {
        TimeRange::Daily => "daily",
        TimeRange::Weekly => "weekly",
        TimeRange::Monthly => "monthly",
    }
}

fn stat_value(repo: &HomeRankRepo, tab: RankType) -> String {
    match tab {
        RankType::Star => repo.stars.to_string(),
        RankType::Fork => repo.forks.to_string(),
        RankType::Issue => repo.issues.to_string(),
        RankType::Recent => short_date(&repo.updated_at),
    }
}

fn short_date(value: &str) -> String {
    let cutoff = value.find('T').unwrap_or(value.len());
    value[..cutoff].to_string()
}

fn stat_icon(tab: RankType) -> &'static str {
    match tab {
        RankType::Star => "★",
        RankType::Fork => "⑂",
        RankType::Issue => "!",
        RankType::Recent => "↻",
    }
}
