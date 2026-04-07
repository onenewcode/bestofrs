use app::prelude::Page;
use app::repo::{RepoSearchResult, RepoSearchTagItem};
use domain::Repo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchRepoDto {
    pub id: String,
    pub full_name: Option<String>,
    pub description: Option<String>,
    pub github_repo_id: Option<i64>,
}

impl From<Repo> for SearchRepoDto {
    fn from(value: Repo) -> Self {
        Self {
            id: value.id.to_string(),
            full_name: value.full_name,
            description: value.description,
            github_repo_id: value.github_repo_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchTagDto {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub repos_total: u64,
}

impl From<RepoSearchTagItem> for SearchTagDto {
    fn from(value: RepoSearchTagItem) -> Self {
        Self {
            label: value.label,
            value: value.value,
            description: value.description,
            repos_total: value.repos_total,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResultDto {
    pub repos: Page<SearchRepoDto>,
    pub tags: Page<SearchTagDto>,
}

impl From<RepoSearchResult> for SearchResultDto {
    fn from(value: RepoSearchResult) -> Self {
        Self {
            repos: value.repos.map(SearchRepoDto::from),
            tags: value.tags.map(SearchTagDto::from),
        }
    }
}
