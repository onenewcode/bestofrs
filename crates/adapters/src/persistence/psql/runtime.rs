use std::sync::Arc;

use app::app_error::AppResult;
use app::backup::DatabaseBackupPort;
use app::project::ProjectRepo;
use app::repo::{RepoRepo, RepoTagRepo};
use app::snapshot::{SnapshotDeltaRepo, SnapshotRepo};

use crate::persistence::{DbRepos, DbRuntime, PersistenceBackend};

use super::{
    connect_and_migrate, PostgresBackupAdapter, PostgresProjectRepo, PostgresRepoRepo,
    PostgresRepoTagRepo, PostgresSnapshotRepo,
};

pub struct PostgresBackend;

#[async_trait::async_trait]
impl PersistenceBackend for PostgresBackend {
    fn name(&self) -> &'static str {
        "postgres"
    }

    fn can_handle(&self, url: &str) -> bool {
        url.starts_with("postgres://") || url.starts_with("postgresql://")
    }

    async fn build_runtime(&self, database_url: &str, backup_dir: &str) -> AppResult<DbRuntime> {
        let pool = connect_and_migrate(database_url).await?;

        let project: Arc<dyn ProjectRepo> = Arc::new(PostgresProjectRepo::new(pool.clone()));
        let repo: Arc<dyn RepoRepo> = Arc::new(PostgresRepoRepo::new(pool.clone()));
        let repo_tag: Arc<dyn RepoTagRepo> = Arc::new(PostgresRepoTagRepo::new(pool.clone()));

        let snapshot_repo = Arc::new(PostgresSnapshotRepo::new(pool));
        let snapshot: Arc<dyn SnapshotRepo> = snapshot_repo.clone();
        let snapshot_delta: Arc<dyn SnapshotDeltaRepo> = snapshot_repo;

        let repos = DbRepos {
            project,
            repo,
            repo_tag,
            snapshot,
            snapshot_delta,
        };
        let backup: Arc<dyn DatabaseBackupPort> = Arc::new(PostgresBackupAdapter::new(
            database_url.to_string(),
            backup_dir.into(),
        ));

        Ok(DbRuntime { repos, backup })
    }
}
