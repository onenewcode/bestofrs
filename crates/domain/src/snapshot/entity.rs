use serde::{Deserialize, Serialize};

use chrono::NaiveDate;

use crate::RepoId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snapshot {
    pub repo_id: RepoId,
    pub snapshot_date: NaiveDate,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub fetched_at: String,
}
