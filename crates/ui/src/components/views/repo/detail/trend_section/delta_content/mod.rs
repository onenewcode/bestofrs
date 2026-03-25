use super::{apply_metric_visibility, TrendContext};
use crate::{types::snapshot_deltas::SnapshotDeltaDto, IO::repos::list_repo_deltas_in_duration};
use app::prelude::{DurationRange, Page};
use dioxus::prelude::*;
use dioxus_i18n::t;

use super::super::{
    build_delta_chart_config, chart_dom_id, short_date_label, ChartJsCanvas, RepoDetailContext,
};

pub(super) mod skeleton;

fn build_delta_visible_config(items: &[SnapshotDeltaDto], metric: &str) -> serde_json::Value {
    let mut labels = Vec::with_capacity(items.len());
    let mut stars_deltas = Vec::with_capacity(items.len());
    let mut forks_deltas = Vec::with_capacity(items.len());
    let mut issues_deltas = Vec::with_capacity(items.len());

    for item in items {
        labels.push(short_date_label(&item.snapshot_date));
        stars_deltas.push(item.stars_delta.unwrap_or(0));
        forks_deltas.push(item.forks_delta.unwrap_or(0));
        issues_deltas.push(item.open_issues_delta.unwrap_or(0));
    }

    apply_metric_visibility(
        build_delta_chart_config(labels, stars_deltas, forks_deltas, issues_deltas),
        metric,
        "delta",
    )
}

#[component]
pub(crate) fn DeltaContent() -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let trend_ctx = use_context::<TrendContext>();
    let delta_timeframe = trend_ctx.delta_timeframe;
    let selected: ReadSignal<Option<Result<Page<SnapshotDeltaDto>, ServerFnError>>> = use_server_future(
        move || {
            let duration = if delta_timeframe() == "weekly" {
                DurationRange::Weekly
            } else {
                DurationRange::Monthly
            };
            list_repo_deltas_in_duration((ctx.owner)(), (ctx.name)(), duration)
        },
    )?
    .into();
    let page: ReadSignal<Option<Page<SnapshotDeltaDto>>> =
        use_memo(move || selected().and_then(|result| result.ok())).into();

    match selected() {
        Some(Ok(_)) => rsx! { DeltaChartContent { page } },
        Some(Err(e)) => Err(e)?,
        None => rsx! { skeleton::DeltaContentSkeleton {} },
    }
}

#[component]
fn DeltaChartContent(page: ReadSignal<Option<Page<SnapshotDeltaDto>>>) -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let trend_ctx = use_context::<TrendContext>();
    let metric = trend_ctx.metric;
    let active_tab = trend_ctx.active_tab;

    let id = chart_dom_id(&(ctx.owner)(), &(ctx.name)(), "delta");
    let config: ReadSignal<serde_json::Value> = use_memo(move || {
        let current_metric = metric();
        page()
            .as_ref()
            .map(|page| build_delta_visible_config(&page.items, &current_metric))
            .unwrap_or(serde_json::Value::Null)
    })
    .into();

    let active = active_tab().as_deref() == Some("delta");

    let has_items = page()
        .as_ref()
        .map(|page| !page.items.is_empty())
        .unwrap_or(false);

    rsx! {
        div { class: "flex h-full flex-col gap-2 md:gap-2",
            if !has_items {
                div { class: "text-sm text-secondary-5", {t!("view_repo_detail_trend_no_delta_data")} }
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
