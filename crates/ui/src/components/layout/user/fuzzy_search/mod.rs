use std::collections::HashSet;

use app::prelude::{Page, Pagination};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_time::use_debounce;
use dioxus_use_js::use_js;

use crate::components::icons::{CommandIcon, SearchIcon};
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs, TabsVariant};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::dialog::{DialogContent, DialogRoot};
use crate::components::ui::input::Input;
use crate::components::ui::virtual_list::VirtualList;
use crate::root::Route;
use crate::types::search::{SearchRepoDto, SearchTagDto};
use crate::IO::search::{search_repo_page, search_tag_page};

use repo_row::{RepoRow, RepoRowSkeleton};
use skeleton::{FuzzySearchCachedFallback, FuzzySearchIdleFallback};
use tag_row::{TagRow, TagRowSkeleton};

mod repo_row;
mod skeleton;
mod tag_row;

use_js!("src/js/dom_bridge.js"::mount_fuzzy_search_hotkey);

const SEARCH_PAGE_SIZE: u32 = 20;
const SEARCH_BUFFER: usize = 8;
const SEARCH_PREFETCH_THRESHOLD: usize = 5;

fn search_pagination(offset: u32) -> Pagination {
    Pagination {
        limit: Some(SEARCH_PAGE_SIZE),
        offset: Some(offset),
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(super) struct PagedSearchState<T> {
    pub(super) items: Vec<T>,
    pub(super) total: usize,
    pub(super) next_offset: u32,
    pub(super) has_more: bool,
    pub(super) loading_initial: bool,
    pub(super) loading_more: bool,
    pub(super) error: Option<String>,
    pub(super) load_more_error: Option<String>,
}

impl<T> Default for PagedSearchState<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            total: 0,
            next_offset: 0,
            has_more: false,
            loading_initial: false,
            loading_more: false,
            error: None,
            load_more_error: None,
        }
    }
}

impl<T> PagedSearchState<T> {
    fn initial_loading() -> Self {
        Self {
            loading_initial: true,
            ..Default::default()
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(super) struct FuzzySearchCachedResult {
    pub(super) repos: PagedSearchState<SearchRepoDto>,
    pub(super) tags: PagedSearchState<SearchTagDto>,
}

fn repo_route(repo: &SearchRepoDto) -> Route {
    match repo.id.split_once('/') {
        Some((owner, name)) => Route::RepoDetailView {
            owner: owner.to_string(),
            name: name.to_string(),
            metric: None,
        },
        None => Route::HomeView {},
    }
}

fn apply_first_page<T>(state: &mut PagedSearchState<T>, page: Page<T>) {
    let next_offset = page.meta.offset.saturating_add(page.items.len() as u32);
    let total = page.meta.total as usize;
    *state = PagedSearchState {
        items: page.items,
        total,
        next_offset,
        has_more: (next_offset as u64) < page.meta.total,
        loading_initial: false,
        loading_more: false,
        error: None,
        load_more_error: None,
    };
}

fn append_page<T, F>(state: &mut PagedSearchState<T>, page: Page<T>, key_of: F)
where
    T: Clone,
    F: Fn(&T) -> String,
{
    let next_offset = page.meta.offset.saturating_add(page.items.len() as u32);
    let total = page.meta.total as usize;
    let mut items = page.items;

    if items.is_empty() {
        state.total = total;
        state.next_offset = next_offset;
        state.has_more = false;
        state.loading_more = false;
        state.load_more_error = None;
        return;
    }

    let mut seen = state.items.iter().map(&key_of).collect::<HashSet<_>>();
    for item in items.drain(..) {
        if seen.insert(key_of(&item)) {
            state.items.push(item);
        }
    }

    state.total = total;
    state.next_offset = next_offset;
    state.has_more = (next_offset as u64) < page.meta.total;
    state.loading_more = false;
    state.load_more_error = None;
}

fn should_load_more<T>(state: &PagedSearchState<T>, idx: usize) -> bool {
    state.has_more
        && !state.loading_initial
        && !state.loading_more
        && idx.saturating_add(SEARCH_PREFETCH_THRESHOLD) >= state.items.len()
}

fn visible_item_count<T>(state: &PagedSearchState<T>) -> usize {
    state.items.len()
}

#[derive(Props, Clone, PartialEq)]
struct SearchLoadMoreStatusProps {
    visible: bool,
    loading: bool,
    error: Option<String>,
    on_retry: Callback<()>,
}

#[component]
fn SearchLoadMoreStatus(props: SearchLoadMoreStatusProps) -> Element {
    if !props.visible || (!props.loading && props.error.is_none()) {
        return rsx! {};
    }

    rsx! {
        div { class: "mt-2 flex shrink-0 items-center justify-between gap-2 rounded-md border border-primary-6 bg-primary-1 px-3 py-2 text-xs text-secondary-5",
            if props.loading {
                span { {t!("layout_user_fuzzy_search_loading_more")} }
            } else if let Some(error) = props.error {
                span { class: "truncate text-primary-error", "{error}" }
                Button {
                    variant: ButtonVariant::Ghost,
                    class: "button rounded-md border border-primary-6 bg-primary px-2 py-1 text-xs hover:bg-primary-3",
                    onclick: move |_| props.on_retry.call(()),
                    {t!("layout_user_fuzzy_search_retry")}
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct RepoResultsPanelProps {
    state: PagedSearchState<SearchRepoDto>,
    on_select: Callback<Route>,
    on_prefetch: Callback<usize>,
    on_retry: Callback<()>,
    allow_load_more: bool,
}

#[component]
fn RepoResultsPanel(props: RepoResultsPanelProps) -> Element {
    if props.state.loading_initial && props.state.items.is_empty() {
        return rsx! {
            div { class: "flex min-h-0 flex-1 flex-col overflow-y-auto pr-1",
                for idx in 0..6 {
                    RepoRowSkeleton { key: "repo-loading-{idx}" }
                }
            }
        };
    }

    if let Some(error) = props.state.error.clone() {
        if props.state.items.is_empty() {
            return rsx! {
                div { class: "flex min-h-0 flex-1 items-center justify-center p-4 text-center text-sm text-primary-error",
                    "{error}"
                }
            };
        }
    }

    if !props.state.loading_initial && props.state.items.is_empty() && props.state.total == 0 {
        return rsx! {
            div { class: "flex min-h-0 flex-1 items-center justify-center p-4 text-center text-sm text-secondary-5",
                {t!("layout_user_fuzzy_search_no_matching_repos")}
            }
        };
    }

    let items: Vec<SearchRepoDto> = props.state.items.clone();
    let visible_count = visible_item_count(&props.state);
    let allow_load_more = props.allow_load_more;
    let on_select = props.on_select;
    let on_prefetch = props.on_prefetch;
    let load_more_error = props.state.load_more_error.clone();

    rsx! {
        div { class: "flex min-h-0 flex-1 flex-col",
            div { class: "min-h-0 flex-1",
                VirtualList {
                    class: "h-full min-h-0 overflow-x-hidden overflow-y-auto pr-1",
                    count: visible_count,
                    buffer: SEARCH_BUFFER,
                    estimate_size: |_idx| 60u32,
                    render_item: move |idx: usize| {
                        if let Some(repo) = items.get(idx).cloned() {
                            let route = repo_route(&repo);
                            let row_key = format!("repo-row-{}", repo.id);
                            let on_select = on_select;
                            let on_prefetch = on_prefetch;
                            rsx! {
                                div {
                                    key: "{row_key}",
                                    onvisible: move |event| {
                                        if allow_load_more
                                            && event.data().is_intersecting().unwrap_or(false)
                                        {
                                            on_prefetch.call(idx);
                                        }
                                    },
                                    RepoRow {
                                        repo,
                                        route,
                                        on_select,
                                    }
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    },
                }
            }
            SearchLoadMoreStatus {
                visible: props.allow_load_more,
                loading: props.state.loading_more,
                error: load_more_error,
                on_retry: props.on_retry,
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TagResultsPanelProps {
    state: PagedSearchState<SearchTagDto>,
    on_select: Callback<(String, String)>,
    on_prefetch: Callback<usize>,
    on_retry: Callback<()>,
    allow_load_more: bool,
}

#[component]
fn TagResultsPanel(props: TagResultsPanelProps) -> Element {
    if props.state.loading_initial && props.state.items.is_empty() {
        return rsx! {
            div { class: "flex min-h-0 flex-1 flex-col overflow-y-auto pr-1",
                for idx in 0..6 {
                    TagRowSkeleton { key: "tag-loading-{idx}" }
                }
            }
        };
    }

    if let Some(error) = props.state.error.clone() {
        if props.state.items.is_empty() {
            return rsx! {
                div { class: "flex min-h-0 flex-1 items-center justify-center p-4 text-center text-sm text-primary-error",
                    "{error}"
                }
            };
        }
    }

    if !props.state.loading_initial && props.state.items.is_empty() && props.state.total == 0 {
        return rsx! {
            div { class: "flex min-h-0 flex-1 items-center justify-center p-4 text-center text-sm text-secondary-5",
                {t!("layout_user_fuzzy_search_no_matching_tags")}
            }
        };
    }

    let items: Vec<SearchTagDto> = props.state.items.clone();
    let visible_count = visible_item_count(&props.state);
    let allow_load_more = props.allow_load_more;
    let on_select = props.on_select;
    let on_prefetch = props.on_prefetch;
    let load_more_error = props.state.load_more_error.clone();

    rsx! {
        div { class: "flex min-h-0 flex-1 flex-col",
            div { class: "min-h-0 flex-1",
                VirtualList {
                    class: "h-full min-h-0 overflow-x-hidden overflow-y-auto pr-1",
                    count: visible_count,
                    buffer: SEARCH_BUFFER,
                    estimate_size: |_idx| 56u32,
                    render_item: move |idx: usize| {
                        if let Some(tag) = items.get(idx).cloned() {
                            let key = format!("{}:{}", tag.label, tag.value);
                            let on_select = on_select;
                            let on_prefetch = on_prefetch;
                            rsx! {
                                div {
                                    key: "{key}",
                                    onvisible: move |event| {
                                        if allow_load_more
                                            && event.data().is_intersecting().unwrap_or(false)
                                        {
                                            on_prefetch.call(idx);
                                        }
                                    },
                                    TagRow {
                                        tag,
                                        on_select,
                                    }
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    },
                }
            }
            SearchLoadMoreStatus {
                visible: props.allow_load_more,
                loading: props.state.loading_more,
                error: load_more_error,
                on_retry: props.on_retry,
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct FuzzySearchResultListProps {
    repo_state: PagedSearchState<SearchRepoDto>,
    tag_state: PagedSearchState<SearchTagDto>,
    on_repo_select: Callback<Route>,
    on_tag_select: Callback<(String, String)>,
    on_repo_prefetch: Callback<usize>,
    on_tag_prefetch: Callback<usize>,
    on_repo_retry: Callback<()>,
    on_tag_retry: Callback<()>,
    allow_load_more: bool,
}

#[component]
pub(super) fn FuzzySearchResultList(props: FuzzySearchResultListProps) -> Element {
    let mut active_tab = use_signal(|| Some("repos".to_string()));
    let active_tab_read: ReadSignal<Option<String>> = active_tab.into();

    rsx! {
        Tabs {
            class: "flex h-full min-h-0 flex-col gap-2".to_string(),
            variant: TabsVariant::Ghost,
            value: active_tab_read,
            default_value: "repos".to_string(),
            on_value_change: move |value| active_tab.set(Some(value)),
            TabList {
                TabTrigger { value: "repos".to_string(), index: 0usize, {t!("layout_user_fuzzy_search_tab_repos", count: props.repo_state.total)} }
                TabTrigger { value: "tags".to_string(), index: 1usize, {t!("layout_user_fuzzy_search_tab_tags", count: props.tag_state.total)} }
            }
            TabContent {
                value: "repos".to_string(),
                index: 0usize,
                class: "p-0 flex min-h-0 flex-1".to_string(),
                RepoResultsPanel {
                    state: props.repo_state,
                    on_select: props.on_repo_select,
                    on_prefetch: props.on_repo_prefetch,
                    on_retry: props.on_repo_retry,
                    allow_load_more: props.allow_load_more,
                }
            }
            TabContent {
                value: "tags".to_string(),
                index: 1usize,
                class: "p-0 flex min-h-0 flex-1".to_string(),
                TagResultsPanel {
                    state: props.tag_state,
                    on_select: props.on_tag_select,
                    on_prefetch: props.on_tag_prefetch,
                    on_retry: props.on_tag_retry,
                    allow_load_more: props.allow_load_more,
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct FuzzySearchResultIOProps {
    draft: String,
    repo_state: PagedSearchState<SearchRepoDto>,
    tag_state: PagedSearchState<SearchTagDto>,
    cached: Option<FuzzySearchCachedResult>,
    on_repo_select: Callback<Route>,
    on_tag_select: Callback<(String, String)>,
    on_repo_prefetch: Callback<usize>,
    on_tag_prefetch: Callback<usize>,
    on_repo_retry: Callback<()>,
    on_tag_retry: Callback<()>,
}

#[component]
fn FuzzySearchResultIO(props: FuzzySearchResultIOProps) -> Element {
    if props.draft.trim().is_empty() {
        return rsx! { FuzzySearchIdleFallback {} };
    }

    if props.repo_state.loading_initial && props.tag_state.loading_initial {
        return rsx! {
            FuzzySearchCachedFallback {
                cached: props.cached,
                on_repo_select: props.on_repo_select,
                on_tag_select: props.on_tag_select,
            }
        };
    }

    rsx! {
        FuzzySearchResultList {
            repo_state: props.repo_state,
            tag_state: props.tag_state,
            on_repo_select: props.on_repo_select,
            on_tag_select: props.on_tag_select,
            on_repo_prefetch: props.on_repo_prefetch,
            on_tag_prefetch: props.on_tag_prefetch,
            on_repo_retry: props.on_repo_retry,
            on_tag_retry: props.on_tag_retry,
            allow_load_more: true,
        }
    }
}

#[component]
pub fn FuzzySearch() -> Element {
    let mut open = use_signal(|| false);
    let mut draft = use_signal(String::new);
    let mut session_id = use_signal(|| 0u64);
    let mut repo_state = use_signal(PagedSearchState::<SearchRepoDto>::default);
    let mut tag_state = use_signal(PagedSearchState::<SearchTagDto>::default);
    let mut last_success = use_signal(|| None::<FuzzySearchCachedResult>);
    let navigator = use_navigator();
    let search_trigger_id = "fuzzy-search-trigger";

    let search_query = use_callback(move |query: String| {
        let query = query.trim().to_string();
        let next_session = session_id().wrapping_add(1);
        session_id.set(next_session);

        if query.is_empty() {
            repo_state.set(PagedSearchState::default());
            tag_state.set(PagedSearchState::default());
            return;
        }

        repo_state.set(PagedSearchState::initial_loading());
        tag_state.set(PagedSearchState::initial_loading());

        let repo_query = query.clone();
        let repo_session = session_id;
        let mut repo_state_signal = repo_state;
        spawn(async move {
            match search_repo_page(repo_query, search_pagination(0)).await {
                Ok(page) => {
                    if repo_session() != next_session {
                        return;
                    }
                    repo_state_signal.with_mut(|state| apply_first_page(state, page));
                }
                Err(err) => {
                    if repo_session() != next_session {
                        return;
                    }
                    repo_state_signal.with_mut(|state| {
                        *state = PagedSearchState::default();
                        state.loading_initial = false;
                        state.error = Some(err.to_string());
                    });
                }
            }
        });

        let tag_session = session_id;
        let mut tag_state_signal = tag_state;
        spawn(async move {
            match search_tag_page(query, search_pagination(0)).await {
                Ok(page) => {
                    if tag_session() != next_session {
                        return;
                    }
                    tag_state_signal.with_mut(|state| apply_first_page(state, page));
                }
                Err(err) => {
                    if tag_session() != next_session {
                        return;
                    }
                    tag_state_signal.with_mut(|state| {
                        *state = PagedSearchState::default();
                        state.loading_initial = false;
                        state.error = Some(err.to_string());
                    });
                }
            }
        });
    });

    let mut debounce_search = use_debounce(
        std::time::Duration::from_millis(300),
        move |text: String| {
            if !open() {
                return;
            }
            search_query.call(text);
        },
    );

    let load_more_repos = use_callback(move |idx: usize| {
        let next_offset = {
            let current = repo_state();
            if !should_load_more(&current, idx) {
                return;
            }
            let next_offset = current.next_offset;
            repo_state.with_mut(|state| {
                state.loading_more = true;
                state.load_more_error = None;
            });
            next_offset
        };

        let query = draft().trim().to_string();
        if query.is_empty() {
            repo_state.with_mut(|state| state.loading_more = false);
            return;
        }

        let current_session = session_id();
        let session_signal = session_id;
        let mut repo_state_signal = repo_state;
        spawn(async move {
            match search_repo_page(query, search_pagination(next_offset)).await {
                Ok(page) => {
                    if session_signal() != current_session {
                        return;
                    }
                    repo_state_signal
                        .with_mut(|state| append_page(state, page, |repo| repo.id.clone()));
                }
                Err(err) => {
                    if session_signal() != current_session {
                        return;
                    }
                    repo_state_signal.with_mut(|state| {
                        state.loading_more = false;
                        state.load_more_error = Some(err.to_string());
                    });
                }
            }
        });
    });

    let load_more_tags = use_callback(move |idx: usize| {
        let next_offset = {
            let current = tag_state();
            if !should_load_more(&current, idx) {
                return;
            }
            let next_offset = current.next_offset;
            tag_state.with_mut(|state| {
                state.loading_more = true;
                state.load_more_error = None;
            });
            next_offset
        };

        let query = draft().trim().to_string();
        if query.is_empty() {
            tag_state.with_mut(|state| state.loading_more = false);
            return;
        }

        let current_session = session_id();
        let session_signal = session_id;
        let mut tag_state_signal = tag_state;
        spawn(async move {
            match search_tag_page(query, search_pagination(next_offset)).await {
                Ok(page) => {
                    if session_signal() != current_session {
                        return;
                    }
                    tag_state_signal.with_mut(|state| {
                        append_page(state, page, |tag| format!("{}:{}", tag.label, tag.value));
                    });
                }
                Err(err) => {
                    if session_signal() != current_session {
                        return;
                    }
                    tag_state_signal.with_mut(|state| {
                        state.loading_more = false;
                        state.load_more_error = Some(err.to_string());
                    });
                }
            }
        });
    });

    let retry_repos = use_callback(move |_| {
        let current = repo_state();
        if current.loading_more || !current.has_more {
            return;
        }
        load_more_repos.call(current.items.len());
    });

    let retry_tags = use_callback(move |_| {
        let current = tag_state();
        if current.loading_more || !current.has_more {
            return;
        }
        load_more_tags.call(current.items.len());
    });

    let close_dialog = use_callback(move |_| {
        open.set(false);
        draft.set(String::new());
        repo_state.set(PagedSearchState::default());
        tag_state.set(PagedSearchState::default());
        session_id.with_mut(|current| *current = current.wrapping_add(1));
    });

    let go_repo = move |route: Route| {
        close_dialog.call(());
        navigator.push(route);
    };
    let go_tag = move |pair: (String, String)| {
        close_dialog.call(());
        navigator.push(Route::RepoListView {
            tags: Some(pair.1),
            metric: None,
            range: None,
            page: None,
            size: None,
        });
    };

    let mut open_dialog = move || {
        if open() {
            return;
        }
        open.set(true);
    };

    let mut on_draft_change = move |value: String| {
        draft.set(value.clone());
        debounce_search.action(value);
    };

    use_effect(move || {
        let search_trigger_id = search_trigger_id.to_string();
        spawn(async move {
            let _ = mount_fuzzy_search_hotkey::<()>(search_trigger_id).await;
        });
    });

    if !draft().trim().is_empty() {
        let repo_snapshot = repo_state();
        let tag_snapshot = tag_state();
        if !repo_snapshot.loading_initial
            && !tag_snapshot.loading_initial
            && repo_snapshot.error.is_none()
            && tag_snapshot.error.is_none()
        {
            let cached = FuzzySearchCachedResult {
                repos: repo_snapshot,
                tags: tag_snapshot,
            };
            if (last_success)().as_ref() != Some(&cached) {
                last_success.set(Some(cached));
            }
        }
    }

    rsx! {
        div { class: "inline-flex items-center gap-2 text-sm text-secondary-4",
            Button {
                id: search_trigger_id,
                variant: ButtonVariant::Outline,
                class: "button inline-flex h-9 w-9 items-center justify-center md:h-10 md:w-auto",
                style: "border-radius: 0.5rem; padding: 0; border-color: var(--primary-color-6);",
                onclick: move |_| {
                    open_dialog();
                },
                span { class: "inline-flex items-center justify-center gap-2 py-1 md:py-2 px-2",
                    SearchIcon { width: 16, height: 16 }
                    span { class: "hidden items-center gap-1 rounded-md bg-secondary-6/40 px-2 py-1 inline-flex",
                        CommandIcon { width: 16, height: 16 }
                        "K"
                    }
                }
            }
        }

        DialogRoot {
            id: None,
            open: open(),
            on_open_change: move |v| {
                open.set(v);
                if !v {
                    close_dialog.call(());
                }
            },
            DialogContent { style: "top: 15%; transform: translate(-50%, 0); height: min(80vh, 34rem); max-height: min(80vh, 34rem); overflow: hidden;",
                Input {
                    class: "input w-full",
                    style: "border-radius: 0.5rem; border-color: var(--primary-color-6);",
                    oninput: move |e: FormEvent| on_draft_change(e.value()),
                    onkeydown: move |e: KeyboardEvent| {
                        if e.key() == Key::Enter {
                            search_query.call(draft());
                        }
                        if e.key() == Key::Escape {
                            close_dialog.call(());
                        }
                    },
                    placeholder: t!("layout_user_fuzzy_search_input_placeholder"),
                    value: draft,
                    aria_label: t!("layout_user_fuzzy_search_input_aria_label"),
                    children: rsx! {},
                }

                div { class: "flex-1 min-h-0 w-full overflow-hidden",
                    FuzzySearchResultIO {
                        draft: draft(),
                        repo_state: repo_state(),
                        tag_state: tag_state(),
                        cached: (last_success)(),
                        on_repo_select: move |route| {
                            go_repo(route);
                        },
                        on_tag_select: move |pair| {
                            go_tag(pair);
                        },
                        on_repo_prefetch: move |idx| {
                            load_more_repos.call(idx);
                        },
                        on_tag_prefetch: move |idx| {
                            load_more_tags.call(idx);
                        },
                        on_repo_retry: move |_| {
                            retry_repos.call(());
                        },
                        on_tag_retry: move |_| {
                            retry_tags.call(());
                        },
                    }
                }
            }
        }
    }
}
