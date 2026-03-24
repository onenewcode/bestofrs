use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::icons::RustGearIcon;
use crate::IO::repos::list_repo_deltas_summary;
use crate::types::snapshot_deltas_summary::SnapshotMetricDeltaSummaryDto;

use super::super::RepoDetailContext;
pub(super) mod skeleton;

fn format_delta(value: i64) -> String {
    if value > 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

#[component]
pub(crate) fn TrendSummary(metric: Signal<String>) -> Element {
    let repo_ctx = use_context::<RepoDetailContext>();
    let summary_fut = use_server_future(move || list_repo_deltas_summary((repo_ctx.owner)(), (repo_ctx.name)()))?;

    match summary_fut() {
        Some(Ok(summary)) => {
            let metric_summary: SnapshotMetricDeltaSummaryDto = match metric().as_str() {
                "forks" => summary.forks,
                "issues" => summary.issues,
                _ => summary.stars,
            };
            let summary_rows = vec![
                (
                    t!("view_repo_detail_trend_timeframe_daily").to_string(),
                    metric_summary.daily,
                ),
                (
                    t!("view_repo_detail_trend_timeframe_weekly").to_string(),
                    metric_summary.weekly,
                ),
                (
                    t!("view_repo_detail_trend_timeframe_monthly").to_string(),
                    metric_summary.monthly,
                ),
            ];

            rsx! {
                div { class: "mx-auto grid w-full grid-cols-3 gap-2 md:w-[64%] md:gap-6",
                    for (label, value) in summary_rows {
                        div {
                            key: "{label}",
                            class: "relative flex min-h-[120px] items-center justify-center md:min-h-[230px]",
                            RustGearIcon {
                                width: 180.0,
                                class: "absolute text-primary-6 scale-[0.52] md:scale-100",
                            }
                            div { class: "relative z-10 flex flex-col items-center gap-0.5 text-center md:gap-1",
                                div { class: "mb-0.5 text-[9px] font-mono font-black tracking-[0.14em] text-secondary-5 uppercase md:mb-1 md:text-xs md:tracking-[0.25em]", "{label}" }
                                div {
                                    class: "text-lg font-black md:text-3xl",
                                    class: if value > 0 {
                                        "text-grid-accent"
                                    } else if value < 0 {
                                        "text-primary-error"
                                    } else {
                                        "text-secondary-3"
                                    },
                                    "{format_delta(value)}"
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(Err(e)) => Err(e)?,
        None => rsx! { skeleton::TrendSummarySkeleton {} },
    }
}
