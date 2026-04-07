use chrono::{Duration, Utc};
use std::sync::Arc;

use domain::{RepoId, Snapshot};

use crate::app_error::AppResult;
use crate::common::{DurationRange, Page, Pagination};
use crate::snapshot::{
    SnapshotDelta, SnapshotDeltaRepo, SnapshotDeltasSummary, SnapshotMetricDeltaSummary,
    SnapshotRepo,
};

#[derive(Clone)]
pub struct SnapshotQueryHandler {
    snapshots: Arc<dyn SnapshotRepo>,
    deltas: Arc<dyn SnapshotDeltaRepo>,
}

impl SnapshotQueryHandler {
    pub fn new(snapshots: Arc<dyn SnapshotRepo>, deltas: Arc<dyn SnapshotDeltaRepo>) -> Self {
        Self { snapshots, deltas }
    }

    pub async fn list_by_repo(
        &self,
        repo_id: &RepoId,
        page: Pagination,
    ) -> AppResult<Page<Snapshot>> {
        self.snapshots.list_by_repo(repo_id, page).await
    }

    pub async fn list_by_owner_name(
        &self,
        owner: &str,
        name: &str,
        page: Pagination,
    ) -> AppResult<Page<Snapshot>> {
        let full_name = format!("{owner}/{name}");
        let repo_id = RepoId::parse(&full_name)?;
        self.list_by_repo(&repo_id, page).await
    }

    pub async fn list_by_repo_in_duration(
        &self,
        repo_id: &RepoId,
        duration: DurationRange,
    ) -> AppResult<Page<Snapshot>> {
        let to_date = Utc::now().date_naive();
        let from_date = to_date - Duration::days(duration.days() - 1);
        self.snapshots
            .list_by_repo_in_date_range(repo_id, from_date, to_date)
            .await
    }

    pub async fn list_by_owner_name_in_duration(
        &self,
        owner: &str,
        name: &str,
        duration: DurationRange,
    ) -> AppResult<Page<Snapshot>> {
        let full_name = format!("{owner}/{name}");
        let repo_id = RepoId::parse(&full_name)?;
        self.list_by_repo_in_duration(&repo_id, duration).await
    }

    pub async fn list_deltas_by_repo(
        &self,
        repo_id: &RepoId,
        page: Pagination,
    ) -> AppResult<Page<SnapshotDelta>> {
        self.deltas.list_by_repo(repo_id, page).await
    }

    pub async fn list_deltas_by_owner_name(
        &self,
        owner: &str,
        name: &str,
        page: Pagination,
    ) -> AppResult<Page<SnapshotDelta>> {
        let full_name = format!("{owner}/{name}");
        let repo_id = RepoId::parse(&full_name)?;
        self.list_deltas_by_repo(&repo_id, page).await
    }

    pub async fn list_deltas_by_repo_in_duration(
        &self,
        repo_id: &RepoId,
        duration: DurationRange,
    ) -> AppResult<Page<SnapshotDelta>> {
        let to_date = Utc::now().date_naive();
        let from_date = to_date - Duration::days(duration.days() - 1);
        self.deltas
            .list_by_repo_in_date_range(repo_id, from_date, to_date)
            .await
    }

    pub async fn list_deltas_by_owner_name_in_duration(
        &self,
        owner: &str,
        name: &str,
        duration: DurationRange,
    ) -> AppResult<Page<SnapshotDelta>> {
        let full_name = format!("{owner}/{name}");
        let repo_id = RepoId::parse(&full_name)?;
        self.list_deltas_by_repo_in_duration(&repo_id, duration)
            .await
    }

    pub async fn list_deltas_summary_by_repo(
        &self,
        repo_id: &RepoId,
    ) -> AppResult<SnapshotDeltasSummary> {
        let page = Pagination {
            limit: Some(31),
            offset: Some(0),
        };
        let snapshots = self.snapshots.list_by_repo(repo_id, page).await?;
        let mut items = snapshots.items;
        items.sort_by_key(|a| a.snapshot_date);

        if items.is_empty() {
            return Ok(SnapshotDeltasSummary {
                stars: SnapshotMetricDeltaSummary {
                    daily: 0,
                    weekly: 0,
                    monthly: 0,
                },
                forks: SnapshotMetricDeltaSummary {
                    daily: 0,
                    weekly: 0,
                    monthly: 0,
                },
                issues: SnapshotMetricDeltaSummary {
                    daily: 0,
                    weekly: 0,
                    monthly: 0,
                },
            });
        }

        let last_idx = items.len() - 1;
        let prev_idx = items.len().saturating_sub(2);
        let week_idx = items.len().saturating_sub(8);
        let month_idx = items.len().saturating_sub(31);

        let latest = &items[last_idx];
        let prev = &items[prev_idx];
        let week = &items[week_idx];
        let month = &items[month_idx];

        let to_summary =
            |latest: i64, prev: i64, week: i64, month: i64| SnapshotMetricDeltaSummary {
                daily: latest - prev,
                weekly: (latest - week) / 7,
                monthly: (latest - month) / 30,
            };

        Ok(SnapshotDeltasSummary {
            stars: to_summary(latest.stars, prev.stars, week.stars, month.stars),
            forks: to_summary(latest.forks, prev.forks, week.forks, month.forks),
            issues: to_summary(
                latest.open_issues,
                prev.open_issues,
                week.open_issues,
                month.open_issues,
            ),
        })
    }

    pub async fn list_deltas_summary_by_owner_name(
        &self,
        owner: &str,
        name: &str,
    ) -> AppResult<SnapshotDeltasSummary> {
        let full_name = format!("{owner}/{name}");
        let repo_id = RepoId::parse(&full_name)?;
        self.list_deltas_summary_by_repo(&repo_id).await
    }
}
