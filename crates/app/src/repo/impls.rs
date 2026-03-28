use domain::Repo;

pub trait RepoAvatarUrlsExt {
    fn avatar_urls(&self) -> Vec<String>;
}

impl RepoAvatarUrlsExt for Repo {
    fn avatar_urls(&self) -> Vec<String> {
        build_avatar_urls(
            self.id.as_str(),
            self.avatar_url.as_deref(),
            self.homepage_url.as_deref(),
        )
    }
}

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

pub fn build_avatar_urls(
    repo_id: &str,
    avatar_url: Option<&str>,
    homepage_url: Option<&str>,
) -> Vec<String> {
    let mut urls = Vec::new();

    if let Some(avatar_url) = avatar_url {
        let avatar_url = avatar_url.trim();
        if !avatar_url.is_empty() && !urls.iter().any(|url| url == avatar_url) {
            urls.push(avatar_url.to_string());
        }
    }

    if let Some(homepage_url) = homepage_url {
        let trimmed = homepage_url.trim().trim_end_matches('/');
        if !trimmed.is_empty() {
            urls.push(format!("{trimmed}/favicon.ico"));
        }
    }

    if let Some((owner, _)) = repo_id.split_once('/') {
        let owner = owner.trim();
        if !owner.is_empty() {
            let owner_avatar = format!("https://github.com/{owner}.png");
            if !urls.iter().any(|url| url == &owner_avatar) {
                urls.push(owner_avatar);
            }
        }
    }

    let fallback = "https://github.com/github.png".to_string();
    if !urls.iter().any(|url| url == &fallback) {
        urls.push(fallback);
    }

    urls
}
