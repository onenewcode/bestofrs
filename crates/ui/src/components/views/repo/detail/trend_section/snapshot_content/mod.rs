use dioxus::prelude::*;
use dioxus_i18n::t;
use std::collections::BTreeMap;

use super::{apply_metric_visibility, TrendContext};
use crate::types::snapshots::SnapshotDto;
use crate::IO::repos::list_repo_snapshots_in_duration;
use app::prelude::{DurationRange, Page};

use super::super::{
    build_trend_chart_config, chart_dom_id, short_date_label, ChartJsCanvas, RepoDetailContext,
};

pub(super) mod skeleton;

fn build_snapshot_visible_config(
    items: &[SnapshotDto],
    metric: &str,
    timeframe: &str,
) -> serde_json::Value {
    let (labels, stars_series, forks_series, issues_series, watchers_series) =
        if timeframe == "yearly" {
            let mut groups: BTreeMap<String, (i64, i64, i64, i64, i64)> = BTreeMap::new();
            for item in items {
                let month_key = item
                    .snapshot_date
                    .get(0..7)
                    .unwrap_or(&item.snapshot_date)
                    .to_string();
                let entry = groups.entry(month_key).or_insert((0, 0, 0, 0, 0));
                entry.0 += item.stars;
                entry.1 += item.forks;
                entry.2 += item.open_issues;
                entry.3 += item.watchers;
                entry.4 += 1;
            }
            let grouped = groups.into_iter().collect::<Vec<_>>();
            let start = grouped.len().saturating_sub(12);
            let recent = grouped[start..].to_vec();
            (
                recent
                    .iter()
                    .map(|(month, _)| month.clone())
                    .collect::<Vec<_>>(),
                recent
                    .iter()
                    .map(|(_, (stars, _, _, _, count))| stars / *count)
                    .collect::<Vec<_>>(),
                recent
                    .iter()
                    .map(|(_, (_, forks, _, _, count))| forks / *count)
                    .collect::<Vec<_>>(),
                recent
                    .iter()
                    .map(|(_, (_, _, issues, _, count))| issues / *count)
                    .collect::<Vec<_>>(),
                recent
                    .iter()
                    .map(|(_, (_, _, _, watchers, count))| watchers / *count)
                    .collect::<Vec<_>>(),
            )
        } else {
            (
                items
                    .iter()
                    .map(|item| short_date_label(&item.snapshot_date))
                    .collect::<Vec<_>>(),
                items.iter().map(|item| item.stars).collect::<Vec<_>>(),
                items.iter().map(|item| item.forks).collect::<Vec<_>>(),
                items
                    .iter()
                    .map(|item| item.open_issues)
                    .collect::<Vec<_>>(),
                items.iter().map(|item| item.watchers).collect::<Vec<_>>(),
            )
        };

    apply_metric_visibility(
        build_trend_chart_config(
            labels,
            stars_series,
            forks_series,
            issues_series,
            watchers_series,
        ),
        metric,
        "snapshot",
    )
}

#[component]
pub(crate) fn SnapshotContent() -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let trend_ctx = use_context::<TrendContext>();
    let snapshot_timeframe = trend_ctx.snapshot_timeframe;
    let selected: ReadSignal<Option<Result<Page<SnapshotDto>, ServerFnError>>> =
        use_server_future(move || {
            let duration = if snapshot_timeframe() == "yearly" {
                DurationRange::Yearly
            } else {
                DurationRange::Monthly
            };
            list_repo_snapshots_in_duration((ctx.owner)(), (ctx.name)(), duration)
        })?
        .into();
    let page: ReadSignal<Option<Page<SnapshotDto>>> =
        use_memo(move || selected().and_then(|result| result.ok())).into();

    match selected() {
        Some(Ok(_)) => rsx! { SnapshotChartContent { page } },
        Some(Err(e)) => Err(e)?,
        None => rsx! { skeleton::SnapshotContentSkeleton {} },
    }
}

#[component]
fn SnapshotChartContent(page: ReadSignal<Option<Page<SnapshotDto>>>) -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let trend_ctx = use_context::<TrendContext>();
    let metric = trend_ctx.metric;
    let snapshot_timeframe = trend_ctx.snapshot_timeframe;
    let active_tab = trend_ctx.active_tab;

    let id = chart_dom_id(&(ctx.owner)(), &(ctx.name)(), "trend");
    let config: ReadSignal<serde_json::Value> = use_memo(move || {
        let current_metric = metric();
        let current_timeframe = snapshot_timeframe();
        page()
            .as_ref()
            .map(|page| {
                build_snapshot_visible_config(&page.items, &current_metric, &current_timeframe)
            })
            .unwrap_or(serde_json::Value::Null)
    })
    .into();
    let active = active_tab().as_deref() == Some("snapshot");
    let has_items = page()
        .as_ref()
        .map(|page| !page.items.is_empty())
        .unwrap_or(false);

    rsx! {
        div { class: "flex h-full flex-col gap-2",
            if !has_items {
                div { class: "text-sm text-secondary-5", {t!("view_repo_detail_trend_no_snapshot_data")} }
            } else {
                div { class: "min-h-0 flex-1 md:border md:border-primary-6 md:bg-primary-1 md:p-3",
                    ChartJsCanvas {
                        id,
                        config,
                        active,
                    }
                }
            }
        }
    }
}
