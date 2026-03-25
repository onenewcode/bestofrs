use dioxus::prelude::*;
use dioxus_use_js::use_js;
use serde_json::{json, Value};

use_js!("src/js/chart_bridge.js"::upsert_chart);

pub(super) fn chart_dom_id(owner: &str, name: &str, suffix: &str) -> String {
    let owner = owner
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>();
    let name = name
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>();
    format!("repo-{owner}-{name}-{suffix}")
}

pub(super) fn short_date_label(input: &str) -> String {
    let parts = input.split('-').collect::<Vec<_>>();
    if parts.len() >= 3 {
        return format!("{}-{}", parts[1], parts[2]);
    }
    input.to_string()
}

pub(super) fn build_trend_chart_config(
    labels: Vec<String>,
    stars_series: Vec<i64>,
    forks_series: Vec<i64>,
    issues_series: Vec<i64>,
    watchers_series: Vec<i64>,
) -> Value {
    let single_point = labels.len() <= 1;
    json!({
        "type": "line",
        "data": {
            "labels": labels,
            "datasets": [
                {
                    "label": "STARS",
                    "data": stars_series,
                    "borderColor": "#10b981",
                    "backgroundColor": "rgba(16, 185, 129, 0.15)",
                    "borderWidth": 2,
                    "pointRadius": 0,
                    "tension": 0.25
                },
                {
                    "label": "FORKS",
                    "data": forks_series,
                    "borderColor": "#7c3aed",
                    "backgroundColor": "rgba(124, 58, 237, 0.15)",
                    "borderWidth": 2,
                    "pointRadius": 0,
                    "tension": 0.25
                },
                {
                    "label": "ISSUES",
                    "data": issues_series,
                    "borderColor": "#f59e0b",
                    "backgroundColor": "rgba(245, 158, 11, 0.15)",
                    "borderWidth": 2,
                    "pointRadius": 0,
                    "tension": 0.25
                },
                {
                    "label": "WATCHERS",
                    "data": watchers_series,
                    "borderColor": "#2563eb",
                    "backgroundColor": "rgba(37, 99, 235, 0.15)",
                    "borderWidth": 2,
                    "pointRadius": 0,
                    "tension": 0.25
                }
            ]
        },
        "options": {
            "responsive": true,
            "maintainAspectRatio": false,
            "interaction": { "mode": "index", "intersect": false },
            "plugins": {
                "legend": { "position": "bottom" },
                "tooltip": { "mode": "index", "intersect": false }
            },
            "scales": {
                "x": {
                    "offset": single_point,
                    "ticks": {
                        "autoSkip": true,
                        "maxTicksLimit": 12,
                        "maxRotation": 45
                    }
                },
                "y": {
                    "ticks": {
                        "maxTicksLimit": 8
                    }
                }
            }
        }
    })
}

pub(super) fn build_delta_chart_config(
    labels: Vec<String>,
    stars_deltas: Vec<i64>,
    forks_deltas: Vec<i64>,
    issues_deltas: Vec<i64>,
) -> Value {
    json!({
        "type": "bar",
        "data": {
            "labels": labels,
            "datasets": [
                {
                    "label": "ΔSTARS",
                    "data": stars_deltas,
                    "backgroundColor": "rgba(16, 185, 129, 0.7)",
                    "borderColor": "#10b981",
                    "borderWidth": 1
                },
                {
                    "label": "ΔFORKS",
                    "data": forks_deltas,
                    "backgroundColor": "rgba(124, 58, 237, 0.7)",
                    "borderColor": "#7c3aed",
                    "borderWidth": 1
                },
                {
                    "label": "ΔISSUES",
                    "data": issues_deltas,
                    "backgroundColor": "rgba(245, 158, 11, 0.7)",
                    "borderColor": "#f59e0b",
                    "borderWidth": 1
                }
            ]
        },
        "options": {
            "responsive": true,
            "maintainAspectRatio": false,
            "interaction": { "mode": "index", "intersect": false },
            "plugins": {
                "legend": { "position": "bottom" },
                "tooltip": { "mode": "index", "intersect": false }
            },
            "scales": {
                "x": {
                    "ticks": {
                        "autoSkip": true,
                        "maxTicksLimit": 14,
                        "maxRotation": 45
                    }
                },
                "y": {
                    "ticks": {
                        "maxTicksLimit": 8
                    }
                }
            }
        }
    })
}

const CHART_JS_CDN: &str = "https://cdn.jsdelivr.net/npm/chart.js@4.5.1/dist/chart.umd.min.js";

#[component]
pub(super) fn ChartJsCanvas(
    id: String,
    config: ReadSignal<Value>,
    active: bool,
    #[props(default = String::from(""))] class: String,
) -> Element {
    use_effect(use_reactive!(|id, config, active| {
        let config_value = config();
        let chart_id = id.clone();
        spawn(async move {
            let _ = upsert_chart::<()>(chart_id, config_value, active).await;
        });
    }));

    rsx! {
        document::Script { src: CHART_JS_CDN, defer: true }
        div { class: "h-72 w-full md:h-80 md:border md:border-primary-6 md:bg-primary-1 md:p-3 {class}",
            canvas { class: "w-full h-full", id }
        }
    }
}
