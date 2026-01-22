use std::sync::Arc;

use domain::{Repo, RepoId};

use crate::app_error::AppResult;
use crate::common::{Page, Pagination};
use crate::repo::RepoRepo;

#[derive(Clone)]
pub struct RepoQueryHandler {
    repos: Arc<dyn RepoRepo>,
}

impl RepoQueryHandler {
    pub fn new(repos: Arc<dyn RepoRepo>) -> Self {
        Self { repos }
    }

    pub async fn get(&self, repo_id: &RepoId) -> AppResult<Option<Repo>> {
        self.repos.get(repo_id).await
    }

    pub async fn list(&self, page: Pagination) -> AppResult<Page<Repo>> {
        self.repos.list(page).await
    }

    pub async fn get_by_owner_name(&self, owner: &str, name: &str) -> AppResult<Option<Repo>> {
        let full_name = format!("{owner}/{name}");
        let repo_id = RepoId::parse(&full_name)?;
        self.get(&repo_id).await
    }
}
