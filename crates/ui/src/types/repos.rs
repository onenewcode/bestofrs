use domain::Repo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepoDto {
    pub id: String,
    pub github_repo_id: Option<i64>,
    pub full_name: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub last_fetched_at: Option<String>,
}

impl From<Repo> for RepoDto {
    fn from(value: Repo) -> Self {
        Self {
            id: value.id.to_string(),
            github_repo_id: value.github_repo_id,
            full_name: value.full_name,
            stars: value.stars,
            forks: value.forks,
            open_issues: value.open_issues,
            watchers: value.watchers,
            last_fetched_at: value.last_fetched_at,
        }
    }
}
