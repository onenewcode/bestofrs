use chrono::NaiveDate;

use crate::RepoId;

/// A snapshot for a repo has been recorded for a given day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotRecorded {
    pub repo_id: RepoId,
    pub snapshot_date: NaiveDate,
}
