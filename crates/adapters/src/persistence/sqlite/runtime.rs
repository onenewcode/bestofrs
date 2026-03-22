use std::sync::Arc;

use app::app_error::AppResult;
use app::backup::DatabaseBackupPort;
use app::project::ProjectRepo;
use app::repo::{RepoRepo, RepoTagRepo};
use app::snapshot::{SnapshotDeltaRepo, SnapshotRepo};

use crate::persistence::{DbRepos, DbRuntime, PersistenceBackend};

use super::{
    connect_and_migrate, SqliteBackupAdapter, SqliteProjectRepo, SqliteRepoRepo, SqliteRepoTagRepo,
    SqliteSnapshotRepo,
};

pub struct SqliteBackend;

#[async_trait::async_trait]
impl PersistenceBackend for SqliteBackend {
    fn name(&self) -> &'static str {
        "sqlite"
    }

    fn can_handle(&self, url: &str) -> bool {
        url.starts_with("sqlite:")
    }

    async fn build_runtime(&self, database_url: &str, backup_dir: &str) -> AppResult<DbRuntime> {
        let pool = connect_and_migrate(database_url).await?;

        let project: Arc<dyn ProjectRepo> = Arc::new(SqliteProjectRepo::new(pool.clone()));
        let repo: Arc<dyn RepoRepo> = Arc::new(SqliteRepoRepo::new(pool.clone()));
        let repo_tag: Arc<dyn RepoTagRepo> = Arc::new(SqliteRepoTagRepo::new(pool.clone()));

        let snapshot_repo = Arc::new(SqliteSnapshotRepo::new(pool.clone()));
        let snapshot: Arc<dyn SnapshotRepo> = snapshot_repo.clone();
        let snapshot_delta: Arc<dyn SnapshotDeltaRepo> = snapshot_repo;

        let repos = DbRepos {
            project,
            repo,
            repo_tag,
            snapshot,
            snapshot_delta,
        };
        let backup: Arc<dyn DatabaseBackupPort> =
            Arc::new(SqliteBackupAdapter::from_pool(pool, backup_dir));

        Ok(DbRuntime { repos, backup })
    }
}
