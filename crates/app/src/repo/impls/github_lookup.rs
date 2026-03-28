use domain::Repo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoGithubLookupKey {
    GithubRepoId(i64),
    RepoFullName(String),
}

impl RepoGithubLookupKey {
    pub fn from_repo_id(repo_id: &str) -> Self {
        Self::RepoFullName(repo_id.to_string())
    }
}

pub trait RepoGithubLookupExt {
    fn github_lookup_key(&self) -> RepoGithubLookupKey;
}

impl RepoGithubLookupExt for Repo {
    fn github_lookup_key(&self) -> RepoGithubLookupKey {
        if let Some(github_repo_id) = self.github_repo_id {
            RepoGithubLookupKey::GithubRepoId(github_repo_id)
        } else {
            RepoGithubLookupKey::RepoFullName(self.id.as_str().to_string())
        }
    }
}
