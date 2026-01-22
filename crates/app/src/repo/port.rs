use domain::{Repo, RepoId};

use crate::app_error::AppResult;

use crate::common::pagination::{Page, Pagination};

#[async_trait::async_trait]
pub trait RepoRepo: Send + Sync {
    async fn upsert(&self, repo: &Repo) -> AppResult<()>;
    async fn upsert_many(&self, repos: &[Repo]) -> AppResult<()>;
    async fn get(&self, id: &RepoId) -> AppResult<Option<Repo>>;
    async fn list(&self, page: Pagination) -> AppResult<Page<Repo>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubRepoInfo {
    pub id: i64,
    pub full_name: String,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub open_issues_count: i64,
    pub subscribers_count: i64,
}

#[async_trait::async_trait]
pub trait GithubGateway: Send + Sync {
    async fn fetch_repo(&self, full_name: &str) -> AppResult<GithubRepoInfo>;
}
