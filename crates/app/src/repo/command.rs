use std::sync::Arc;

use domain::Repo;

use crate::app_error::AppResult;
use crate::repo::RepoRepo;

#[derive(Clone)]
pub struct RepoCommandHandler {
    repos: Arc<dyn RepoRepo>,
}

impl RepoCommandHandler {
    pub fn new(repos: Arc<dyn RepoRepo>) -> Self {
        Self { repos }
    }
    pub async fn upsert(&self, repo: &Repo) -> AppResult<()> {
        self.upsert_many(std::slice::from_ref(repo)).await
    }

    pub async fn upsert_many(&self, repos: &[Repo]) -> AppResult<()> {
        self.repos.upsert_many(repos).await
    }
}
