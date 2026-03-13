use dioxus::prelude::*;

use crate::{types::snapshot_deltas::SnapshotDeltaDto, IO::repos::list_repo_deltas};
use app::prelude::{Page, Pagination};

use super::{
    build_delta_chart_config, chart_dom_id, short_date_label, ChartJsCanvas, RepoDetailContext,
};

pub(super) mod skeleton;

#[component]
pub(crate) fn DeltasSection() -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let deltas_fut = use_server_future(move || {
        list_repo_deltas(
            (ctx.owner)(),
            (ctx.name)(),
            Pagination {
                limit: Some(100),
                offset: Some(0),
            },
        )
    })?;

    rsx! {
        section { class: "space-y-4 border border-primary-6 bg-primary p-5 shadow-comic-sm",
            h3 { class: "text-sm font-semibold text-secondary-4", "Daily deltas" }
            match deltas_fut() {
                Some(Ok(page)) => rsx! { DeltasContent { page: page.clone() } },
                Some(Err(e)) => Err(e)?,
                None => rsx! { skeleton::DeltasSectionSkeleton {} },
            }
        }
    }
}

#[component]
fn DeltasContent(page: Page<SnapshotDeltaDto>) -> Element {
    let ctx = use_context::<RepoDetailContext>();

    let chart_id_memo = use_memo({
        let owner = ctx.owner;
        let name = ctx.name;
        move || chart_dom_id(&owner(), &name(), "delta")
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

            build_delta_chart_config(labels, stars_deltas, forks_deltas, issues_deltas)
        }
    });

    let id: ReadSignal<String> = chart_id_memo.into();
    let chart_config: ReadSignal<serde_json::Value> = chart_config_memo.into();

    rsx! {
        div { class: "text-sm text-secondary-5", "count: {page.meta.total}" }
        if page.items.is_empty() {
            div { class: "text-sm text-secondary-5", "No delta data" }
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
