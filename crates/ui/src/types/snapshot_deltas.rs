use app::snapshot::SnapshotDelta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotDeltaDto {
    pub repo_id: String,
    pub snapshot_date: String,

    pub prev_snapshot_date: Option<String>,

    pub stars_delta: Option<i64>,
    pub forks_delta: Option<i64>,
    pub open_issues_delta: Option<i64>,
    pub watchers_delta: Option<i64>,
}

impl From<SnapshotDelta> for SnapshotDeltaDto {
    fn from(value: SnapshotDelta) -> Self {
        Self {
            repo_id: value.repo_id.to_string(),
            snapshot_date: value.snapshot_date.to_string(),
            prev_snapshot_date: value.prev_snapshot_date.map(|d| d.to_string()),
            stars_delta: value.stars_delta,
            forks_delta: value.forks_delta,
            open_issues_delta: value.open_issues_delta,
            watchers_delta: value.watchers_delta,
        }
    }
}
