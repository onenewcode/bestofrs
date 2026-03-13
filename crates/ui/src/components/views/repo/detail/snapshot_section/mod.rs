use dioxus::prelude::*;

use crate::types::snapshots::SnapshotDto;
use crate::IO::repos::list_repo_snapshots;
use app::prelude::{Page, Pagination};

use super::{
    build_trend_chart_config, chart_dom_id, short_date_label, ChartJsCanvas, RepoDetailContext,
};

pub(super) mod skeleton;

#[component]
pub(crate) fn SnapshotSection() -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let snapshots_fut = use_server_future(move || {
        list_repo_snapshots(
            (ctx.owner)(),
            (ctx.name)(),
            Pagination {
                limit: Some(100),
                offset: Some(0),
            },
        )
    })?;

    rsx! {
        section { class: "space-y-5 border border-primary-6 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                h2 { class: "text-lg font-semibold", "Trend" }
                p { class: "text-sm text-secondary-5", "Snapshots timeline" }
            }

            match snapshots_fut() {
                Some(Ok(page)) => rsx! { SnapshotContent { page: page.clone() } },
                Some(Err(e)) => Err(e)?,
                None => rsx! { skeleton::SnapshotSectionSkeleton {} },
            }
        }
    }
}

#[component]
fn SnapshotContent(page: Page<SnapshotDto>) -> Element {
    let ctx = use_context::<RepoDetailContext>();

    let chart_id_memo = use_memo({
        let owner = ctx.owner;
        let name = ctx.name;
        move || chart_dom_id(&owner(), &name(), "trend")
    });

    let chart_config_memo = use_memo({
        let page = page.clone();
        move || {
            let mut sorted_items = page.items.clone();
            sorted_items.sort_by(|a, b| a.snapshot_date.cmp(&b.snapshot_date));

            let labels = sorted_items
                .iter()
                .map(|item| short_date_label(&item.snapshot_date))
                .collect::<Vec<_>>();
            let stars_series = sorted_items
                .iter()
                .map(|item| item.stars)
                .collect::<Vec<_>>();
            let forks_series = sorted_items
                .iter()
                .map(|item| item.forks)
                .collect::<Vec<_>>();
            let issues_series = sorted_items
                .iter()
                .map(|item| item.open_issues)
                .collect::<Vec<_>>();
            let watchers_series = sorted_items
                .iter()
                .map(|item| item.watchers)
                .collect::<Vec<_>>();

            build_trend_chart_config(
                labels,
                stars_series,
                forks_series,
                issues_series,
                watchers_series,
            )
        }
    });

    let id: ReadSignal<String> = chart_id_memo.into();
    let chart_config: ReadSignal<serde_json::Value> = chart_config_memo.into();

    rsx! {
        div { class: "text-sm text-secondary-5", "count: {page.meta.total}" }
        if page.items.is_empty() {
            div { class: "text-sm text-secondary-5", "No snapshot data" }
        } else {
            div { class: "border border-primary-6 bg-primary-1 p-3",
                ChartJsCanvas {
                    id,
                    config: chart_config,
                }
            }
        }
    }
}
