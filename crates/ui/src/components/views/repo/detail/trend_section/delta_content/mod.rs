use super::{apply_metric_visibility, TrendContext};
use crate::{types::snapshot_deltas::SnapshotDeltaDto, IO::repos::list_repo_deltas_in_duration};
use app::prelude::{DurationRange, Page};
use dioxus::prelude::*;

use super::super::{
    build_delta_chart_config, chart_dom_id, short_date_label, ChartJsCanvas, RepoDetailContext,
};

pub(super) mod skeleton;

#[component]
pub(crate) fn DeltaContent() -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let trend_ctx = use_context::<TrendContext>();
    let delta_timeframe = trend_ctx.delta_timeframe;
    let deltas_fut = use_server_future(move || {
        let duration = if delta_timeframe() == "weekly" {
            DurationRange::Weekly
        } else {
            DurationRange::Monthly
        };
        list_repo_deltas_in_duration((ctx.owner)(), (ctx.name)(), duration)
    })?;

    match deltas_fut() {
        Some(Ok(page)) => rsx! { DeltaChartContent { page: page.clone() } },
        Some(Err(e)) => Err(e)?,
        None => rsx! { skeleton::DeltaContentSkeleton {} },
    }
}

#[component]
fn DeltaChartContent(page: Page<SnapshotDeltaDto>) -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let trend_ctx = use_context::<TrendContext>();
    let metric = trend_ctx.metric;
    let delta_timeframe = trend_ctx.delta_timeframe;

    let chart_id_memo = use_memo({
        let owner = ctx.owner;
        let name = ctx.name;
        let delta_timeframe = delta_timeframe;
        move || {
            let timeframe = delta_timeframe();
            chart_dom_id(&owner(), &name(), &format!("delta-{timeframe}"))
        }
    });

    let chart_config_memo = use_memo({
        let page = page.clone();
        let metric = metric;
        let delta_timeframe = delta_timeframe;
        move || {
            let current_metric = metric();
            let _current_timeframe = delta_timeframe();
            let sorted_items = page.items.clone();

            let labels = sorted_items
                .iter()
                .map(|item| short_date_label(&item.snapshot_date))
                .collect::<Vec<_>>();
            let stars_deltas = sorted_items
                .iter()
                .map(|item| item.stars_delta.unwrap_or(0))
                .collect::<Vec<_>>();
            let forks_deltas = sorted_items
                .iter()
                .map(|item| item.forks_delta.unwrap_or(0))
                .collect::<Vec<_>>();
            let issues_deltas = sorted_items
                .iter()
                .map(|item| item.open_issues_delta.unwrap_or(0))
                .collect::<Vec<_>>();

            let config =
                build_delta_chart_config(labels, stars_deltas, forks_deltas, issues_deltas);
            apply_metric_visibility(config, &current_metric, "delta")
        }
    });

    let id: ReadSignal<String> = chart_id_memo.into();
    let chart_config: ReadSignal<serde_json::Value> = chart_config_memo.into();

    rsx! {
        div { class: "flex h-full flex-col gap-2",
            if page.items.is_empty() {
                div { class: "text-sm text-secondary-5", "No delta data" }
            } else {
                div { class: "min-h-0 flex-1 border border-primary-6 bg-primary-1 p-3",
                    ChartJsCanvas {
                        id,
                        config: chart_config,
                    }
                }
            }
        }
    }
}
