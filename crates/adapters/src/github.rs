use app::app_error::{AppError, AppResult};
use app::repo::{GithubGateway, GithubRepoInfo};

#[derive(Clone)]
pub struct GithubClient {
    client: reqwest::Client,
    token: Option<String>,
}

impl GithubClient {
    pub fn new(token: Option<String>) -> AppResult<Self> {
        let client = reqwest::Client::builder()
            .user_agent("bestofrs")
            .build()
            .map_err(AppError::internal)?;
        let token = token
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        Ok(Self { client, token })
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GithubRepoResponse {
    id: i64,
    full_name: String,
    stargazers_count: i64,
    forks_count: i64,
    open_issues_count: i64,
    subscribers_count: i64,
}

#[async_trait::async_trait]
impl GithubGateway for GithubClient {
    async fn fetch_repo(&self, full_name: &str) -> AppResult<GithubRepoInfo> {
        let url = format!("https://api.github.com/repos/{full_name}");

        let mut req = self.client.get(url);
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("token {token}"));
        }

        let resp = req
            .send()
            .await
            .map_err(AppError::upstream)?
            .error_for_status()
            .map_err(AppError::upstream)?;
        let repo: GithubRepoResponse = resp.json().await.map_err(AppError::upstream)?;

        Ok(GithubRepoInfo {
            id: repo.id,
            full_name: repo.full_name,
            stargazers_count: repo.stargazers_count,
            forks_count: repo.forks_count,
            open_issues_count: repo.open_issues_count,
            subscribers_count: repo.subscribers_count,
        })
    }
}
