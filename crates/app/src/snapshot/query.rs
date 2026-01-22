use std::sync::Arc;

use domain::{RepoId, Snapshot};

use crate::app_error::AppResult;
use crate::common::pagination::{Page, Pagination};
use crate::snapshot::{SnapshotDelta, SnapshotDeltaRepo, SnapshotRepo};

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
}
