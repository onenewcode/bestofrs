use dioxus::prelude::*;

use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectTrigger, SelectValue,
};

use super::{
    filter_label, normalize_page_size, repo_list_route_from_ctx, sort_label, FilterType,
    RepoListContext, SortType,
};

#[component]
pub(super) fn RepoListHandler() -> Element {
    let ctx = use_context::<RepoListContext>();
    let navigator = use_navigator();

    rsx! {
        div { class: "flex flex-col md:flex-row items-center justify-between gap-4",
            div { class: "text-xs font-mono tracking-wide text-secondary-5",
                "List "
                span { class: "font-semibold text-grid-accent",
                    "{(ctx.summary)().from}-{(ctx.summary)().to}"
                }
                " of "
                span { class: "font-semibold text-grid-accent", "{(ctx.summary)().total}" }
            }
            div { class: "flex items-center gap-4 w-full md:w-auto",
                Select::<FilterType> {
                    value: Some((ctx.filter_type)()),
                    placeholder: "filter",
                    on_value_change: move |next: Option<FilterType>| {
                        if let Some(next_filter) = next {
                            let next_sort = (ctx.sort_type)();
                            let (metric_q, range_q) =
                                super::query_params_from_filter_sort(next_filter, next_sort);
                            navigator.push(crate::root::Route::RepoListView {
                                tags: super::active_tags_to_query(&(ctx.active_tags)()),
                                metric: metric_q,
                                range: range_q,
                                page: Some(1),
                                size: Some((ctx.page_size)()),
                            });
                        }
                    },
                    SelectTrigger {
                        aria_label: "Select filter",
                        style: "min-width: 9rem;",
                        SelectValue {}
                    }
                    SelectList { aria_label: "Filter options",
                        SelectGroup {
                            SelectGroupLabel { "Filter" }
                            SelectOption::<FilterType> {
                                index: 0usize,
                                value: FilterType::Total,
                                text_value: Some(filter_label(FilterType::Total).to_string()),
                                "{filter_label(FilterType::Total)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<FilterType> {
                                index: 1usize,
                                value: FilterType::Daily,
                                text_value: Some(filter_label(FilterType::Daily).to_string()),
                                "{filter_label(FilterType::Daily)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<FilterType> {
                                index: 2usize,
                                value: FilterType::Weekly,
                                text_value: Some(filter_label(FilterType::Weekly).to_string()),
                                "{filter_label(FilterType::Weekly)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<FilterType> {
                                index: 3usize,
                                value: FilterType::Monthly,
                                text_value: Some(filter_label(FilterType::Monthly).to_string()),
                                "{filter_label(FilterType::Monthly)}"
                                SelectItemIndicator {}
                            }
                        }
                    }
                }
                Select::<u32> {
                    value: Some((ctx.page_size)()),
                    placeholder: "page size",
                    on_value_change: move |v: Option<u32>| {
                        if let Some(v) = v {
                            let next_size = normalize_page_size(v);
                            navigator.replace(repo_list_route_from_ctx(ctx, 1, next_size));
                        }
                    },
                    SelectTrigger {
                        aria_label: "Select page size",
                        style: "min-width: 7rem;",
                        SelectValue {}
                    }
                    SelectList { aria_label: "Page size options",
                        SelectGroup {
                            SelectGroupLabel { "Page size" }
                            SelectOption::<u32> {
                                index: 0usize,
                                value: 20u32,
                                text_value: Some("20".to_string()),
                                "20"
                                SelectItemIndicator {}
                            }
                            SelectOption::<u32> {
                                index: 1usize,
                                value: 50u32,
                                text_value: Some("50".to_string()),
                                "50"
                                SelectItemIndicator {}
                            }
                            SelectOption::<u32> {
                                index: 2usize,
                                value: 100u32,
                                text_value: Some("100".to_string()),
                                "100"
                                SelectItemIndicator {}
                            }
                        }
                    }
                }
                Select::<SortType> {
                    value: Some((ctx.sort_type)()),
                    placeholder: "sort",
                    on_value_change: move |next: Option<SortType>| {
                        if let Some(next_sort) = next {
                            let next_filter = if next_sort == SortType::AddTime {
                                FilterType::Total
                            } else {
                                (ctx.filter_type)()
                            };
                            let (metric_q, range_q) =
                                super::query_params_from_filter_sort(next_filter, next_sort);
                            navigator.push(crate::root::Route::RepoListView {
                                tags: super::active_tags_to_query(&(ctx.active_tags)()),
                                metric: metric_q,
                                range: range_q,
                                page: Some(1),
                                size: Some((ctx.page_size)()),
                            });
                        }
                    },
                    SelectTrigger { aria_label: "Select sort", style: "min-width: 10rem;", SelectValue {} }
                    SelectList { aria_label: "Sort options",
                        SelectGroup {
                            SelectGroupLabel { "Sort" }
                            SelectOption::<SortType> {
                                index: 0usize,
                                value: SortType::Star,
                                text_value: Some(sort_label(SortType::Star).to_string()),
                                "{sort_label(SortType::Star)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<SortType> {
                                index: 1usize,
                                value: SortType::Fork,
                                text_value: Some(sort_label(SortType::Fork).to_string()),
                                "{sort_label(SortType::Fork)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<SortType> {
                                index: 2usize,
                                value: SortType::Issue,
                                text_value: Some(sort_label(SortType::Issue).to_string()),
                                "{sort_label(SortType::Issue)}"
                                SelectItemIndicator {}
                            }
                            SelectOption::<SortType> {
                                index: 3usize,
                                value: SortType::AddTime,
                                text_value: Some(sort_label(SortType::AddTime).to_string()),
                                "{sort_label(SortType::AddTime)}"
                                SelectItemIndicator {}
                            }
                        }
                    }
                }
            }
        }
    }
}
