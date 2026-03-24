pub(super) mod delta_content;
pub(super) mod snapshot_content;
pub(super) mod summary_content;

use crate::components::common::IOCell;
use crate::components::tabs::{TabContent, TabList, TabTrigger, Tabs, TabsVariant};
use dioxus::prelude::*;
use dioxus_i18n::t;
use serde_json::Value;

use delta_content::skeleton::DeltaContentSkeleton;
use delta_content::DeltaContent;
use snapshot_content::skeleton::SnapshotContentSkeleton;
use snapshot_content::SnapshotContent;
use summary_content::skeleton::TrendSummarySkeleton;
use summary_content::TrendSummary;

#[derive(Clone, Copy)]
pub(super) struct TrendContext {
    pub(super) metric: Signal<String>,
    pub(super) delta_timeframe: Signal<String>,
    pub(super) snapshot_timeframe: Signal<String>,
    pub(super) active_tab: Signal<Option<String>>,
}

fn metric_dataset_label(metric: &str, chart_kind: &str) -> &'static str {
    match chart_kind {
        "delta" => match metric {
            "forks" => "ΔFORKS",
            "issues" => "ΔISSUES",
            _ => "ΔSTARS",
        },
        _ => match metric {
            "forks" => "FORKS",
            "issues" => "ISSUES",
            "watchers" => "WATCHERS",
            _ => "STARS",
        },
    }
}

pub(super) fn apply_metric_visibility(mut config: Value, metric: &str, chart_kind: &str) -> Value {
    let active_label = metric_dataset_label(metric, chart_kind);
    if let Some(datasets) = config
        .get_mut("data")
        .and_then(|data| data.get_mut("datasets"))
        .and_then(|datasets| datasets.as_array_mut())
    {
        for dataset in datasets {
            let is_active = dataset
                .get("label")
                .and_then(|label| label.as_str())
                .map(|label| label == active_label)
                .unwrap_or(false);
            if let Some(dataset_obj) = dataset.as_object_mut() {
                dataset_obj.insert("hidden".to_string(), Value::Bool(!is_active));
            }
        }
    }
    config
}

fn normalize_metric(metric: Option<&str>) -> String {
    match metric.unwrap_or_default().trim().to_lowercase().as_str() {
        "fork" | "forks" => "forks".to_string(),
        "issue" | "issues" => "issues".to_string(),
        "star" | "stars" => "stars".to_string(),
        _ => "stars".to_string(),
    }
}

#[component]
pub(crate) fn TrendSection(initial_metric: ReadSignal<Option<String>>) -> Element {
    let mut metric = use_signal(move || normalize_metric(initial_metric().as_deref()));
    let mut delta_timeframe = use_signal(|| "weekly".to_string());
    let mut snapshot_timeframe = use_signal(|| "monthly".to_string());
    let mut active_tab = use_signal(|| Some("delta".to_string()));

    use_context_provider(|| TrendContext {
        metric,
        delta_timeframe,
        snapshot_timeframe,
        active_tab,
    });
    let active_tab_read: ReadSignal<Option<String>> = active_tab.into();

    rsx! {
        section { class: "space-y-6",
            div { class: "space-y-4",
                h2 { class: "text-5xl leading-[0.8] font-black tracking-tighter text-secondary-2 uppercase md:text-7xl",
                    "trend"
                    br {}
                    span { class: "text-transparent [-webkit-text-stroke:2px_var(--primary-color-6)]",
                        "analyze"
                    }
                }
                div { class: "mb-8 flex flex-wrap justify-center gap-2",
                    for item in ["stars", "forks", "issues"] {
                        button {
                            key: "{item}",
                            class: "px-4 py-2 text-xs font-mono font-bold tracking-widest uppercase hover:cursor-pointer",
                            class: if metric() == item {
                                "border border-secondary-2 bg-secondary-2 text-primary shadow-comic-sm"
                            } else {
                                "border border-primary-6 bg-primary text-secondary-4 hover:bg-primary-1"
                            },
                            onclick: move |_| metric.set(item.to_string()),
                            "{item}"
                        }
                    }
                }
            }

            IOCell {
                loading_fallback: rsx! { TrendSummarySkeleton {} },
                TrendSummary { metric }
            }

            Tabs {
                class: "space-y-4".to_string(),
                variant: TabsVariant::Ghost,
                value: active_tab_read,
                default_value: "delta".to_string(),
                on_value_change: move |value| active_tab.set(Some(value)),
                TabList {
                    TabTrigger {
                        value: "delta".to_string(),
                        index: 0usize,
                        {t!("view_repo_detail_trend_tab_deltas")}
                    }
                    TabTrigger {
                        value: "snapshot".to_string(),
                        index: 1usize,
                        {t!("view_repo_detail_trend_tab_snapshot")}
                    }
                }
                TabContent {
                    value: "delta".to_string(),
                    index: 0usize,
                    class: "flex h-[24rem] flex-col gap-4 border-0 bg-primary-1 p-0 shadow-none md:border md:border-primary-6 md:p-4 md:shadow-comic-sm".to_string(),
                    div { class: "flex justify-end px-3 pt-3 md:px-0 md:pt-0",
                        div { class: "flex gap-2",
                            button {
                                class: "px-3 py-1 text-[10px] font-mono font-bold tracking-widest uppercase hover:cursor-pointer",
                                class: if delta_timeframe() == "weekly" {
                                    "border border-secondary-2 bg-secondary-2 text-primary"
                                } else {
                                    "border border-primary-6 bg-primary text-secondary-4 hover:bg-primary-1"
                                },
                                onclick: move |_| delta_timeframe.set("weekly".to_string()),
                                {t!("view_repo_detail_trend_timeframe_weekly")}
                            }
                            button {
                                class: "px-3 py-1 text-[10px] font-mono font-bold tracking-widest uppercase hover:cursor-pointer",
                                class: if delta_timeframe() == "monthly" {
                                    "border border-secondary-2 bg-secondary-2 text-primary"
                                } else {
                                    "border border-primary-6 bg-primary text-secondary-4 hover:bg-primary-1"
                                },
                                onclick: move |_| delta_timeframe.set("monthly".to_string()),
                                {t!("view_repo_detail_trend_timeframe_monthly")}
                            }
                        }
                    }
                    div { class: "min-h-0 flex-1",
                        div { key: "delta-{delta_timeframe()}",
                            IOCell {
                                loading_fallback: rsx! { DeltaContentSkeleton {} },
                                DeltaContent {}
                            }
                        }
                    }
                }
                TabContent {
                    value: "snapshot".to_string(),
                    index: 1usize,
                    class: "flex h-[24rem] flex-col gap-4 border-0 bg-primary-1 p-0 shadow-none md:border md:border-primary-6 md:p-4 md:shadow-comic-sm".to_string(),
                    div { class: "flex justify-end px-3 pt-3 md:px-0 md:pt-0",
                        div { class: "flex gap-2",
                            button {
                                class: "px-3 py-1 text-[10px] font-mono font-bold tracking-widest uppercase hover:cursor-pointer",
                                class: if snapshot_timeframe() == "monthly" {
                                    "border border-secondary-2 bg-secondary-2 text-primary"
                                } else {
                                    "border border-primary-6 bg-primary text-secondary-4 hover:bg-primary-1"
                                },
                                onclick: move |_| snapshot_timeframe.set("monthly".to_string()),
                                {t!("view_repo_detail_trend_timeframe_monthly")}
                            }
                            button {
                                class: "px-3 py-1 text-[10px] font-mono font-bold tracking-widest uppercase hover:cursor-pointer",
                                class: if snapshot_timeframe() == "yearly" {
                                    "border border-secondary-2 bg-secondary-2 text-primary"
                                } else {
                                    "border border-primary-6 bg-primary text-secondary-4 hover:bg-primary-1"
                                },
                                onclick: move |_| snapshot_timeframe.set("yearly".to_string()),
                                {t!("view_repo_detail_trend_timeframe_yearly")}
                            }
                        }
                    }
                    div { class: "min-h-0 flex-1",
                        div { key: "snapshot-{snapshot_timeframe()}",
                            IOCell {
                                loading_fallback: rsx! { SnapshotContentSkeleton {} },
                                SnapshotContent {}
                            }
                        }
                    }
                }
            }
        }
    }
}
