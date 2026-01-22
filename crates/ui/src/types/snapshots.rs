use domain::Snapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotDto {
    pub repo_id: String,
    pub snapshot_date: String,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub fetched_at: String,
}

impl From<Snapshot> for SnapshotDto {
    fn from(value: Snapshot) -> Self {
        Self {
            repo_id: value.repo_id.to_string(),
            snapshot_date: value.snapshot_date.to_string(),
            stars: value.stars,
            forks: value.forks,
            open_issues: value.open_issues,
            watchers: value.watchers,
            fetched_at: value.fetched_at,
        }
    }
}
