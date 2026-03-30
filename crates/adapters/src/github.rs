use app::app_error::{AppError, AppResult};
use app::repo::{
    GithubGateway, GithubLatestPushedRepoInfo, GithubLatestPushedRepoSearchResult, GithubReadme,
    GithubRepoInfo,
};
use base64::Engine;
use std::cmp::Reverse;
use std::collections::HashSet;

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

    fn authorized_get(&self, url: String) -> reqwest::RequestBuilder {
        let mut req = self.client.get(url);
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("token {token}"));
        }
        req
    }

    fn decode_readme_content(content: &str, encoding: &str) -> AppResult<String> {
        if !encoding.eq_ignore_ascii_case("base64") {
            return Err(AppError::upstream(format!(
                "Unsupported README encoding: {encoding}"
            )));
        }

        let cleaned = content.replace(['\n', '\r'], "");
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(cleaned)
            .map_err(AppError::upstream)?;

        String::from_utf8(decoded).map_err(AppError::upstream)
    }

    fn into_repo_info(repo: GithubRepoResponse) -> GithubRepoInfo {
        GithubRepoInfo {
            id: repo.id,
            full_name: repo.full_name,
            description: repo
                .description
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty()),
            homepage: repo
                .homepage
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty()),
            owner_avatar_url: repo
                .owner
                .avatar_url
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty()),
            stargazers_count: repo.stargazers_count,
            forks_count: repo.forks_count,
            open_issues_count: repo.open_issues_count,
            subscribers_count: repo.subscribers_count,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GithubRepoResponse {
    id: i64,
    full_name: String,
    description: Option<String>,
    homepage: Option<String>,
    owner: GithubRepoOwner,
    stargazers_count: i64,
    forks_count: i64,
    open_issues_count: i64,
    subscribers_count: i64,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GithubRepoOwner {
    avatar_url: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GithubReadmeResponse {
    content: String,
    encoding: String,
    html_url: Option<String>,
    download_url: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GithubSearchRepositoriesResponse {
    total_count: u64,
    items: Vec<GithubSearchRepoItemResponse>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GithubSearchRepoItemResponse {
    id: i64,
    full_name: String,
    stargazers_count: i64,
    created_at: String,
    pushed_at: String,
}

#[async_trait::async_trait]
impl GithubGateway for GithubClient {
    async fn fetch_repo(&self, full_name: &str) -> AppResult<GithubRepoInfo> {
        let url = format!("https://api.github.com/repos/{full_name}");

        let resp = self
            .authorized_get(url)
            .send()
            .await
            .map_err(AppError::upstream)?
            .error_for_status()
            .map_err(AppError::upstream)?;
        let repo: GithubRepoResponse = resp.json().await.map_err(AppError::upstream)?;

        Ok(Self::into_repo_info(repo))
    }

    async fn fetch_repo_by_github_id(&self, github_repo_id: i64) -> AppResult<GithubRepoInfo> {
        let url = format!("https://api.github.com/repositories/{github_repo_id}");

        let resp = self
            .authorized_get(url)
            .send()
            .await
            .map_err(AppError::upstream)?
            .error_for_status()
            .map_err(AppError::upstream)?;
        let repo: GithubRepoResponse = resp.json().await.map_err(AppError::upstream)?;

        Ok(Self::into_repo_info(repo))
    }

    async fn search_recently_pushed_repos(
        &self,
        limit: usize,
    ) -> AppResult<GithubLatestPushedRepoSearchResult> {
        if limit == 0 {
            return Ok(GithubLatestPushedRepoSearchResult {
                requested_limit: limit.min(1000),
                upstream_total_count: Some(0),
                fetched_raw_count: 0,
                unique_count: 0,
                items: Vec::new(),
            });
        }

        let capped_limit = limit.min(1000);
        let per_page = 100usize;
        let total_pages = capped_limit.div_ceil(per_page).min(10);
        let query = "language:rust".to_string();

        let mut out = Vec::with_capacity(capped_limit);
        let mut seen_ids = HashSet::with_capacity(capped_limit);
        let mut fetched_raw_count = 0usize;
        let mut upstream_total_count = None;

        for page in 1..=total_pages {
            let page_str = page.to_string();
            let url = reqwest::Url::parse_with_params(
                "https://api.github.com/search/repositories",
                &[
                    ("q", query.as_str()),
                    ("sort", "updated"),
                    ("order", "desc"),
                    ("per_page", "100"),
                    ("page", page_str.as_str()),
                ],
            )
            .map_err(AppError::internal)?
            .to_string();

            let resp = self
                .authorized_get(url)
                .send()
                .await
                .map_err(AppError::upstream)?
                .error_for_status()
                .map_err(AppError::upstream)?;

            let body: GithubSearchRepositoriesResponse =
                resp.json().await.map_err(AppError::upstream)?;
            if upstream_total_count.is_none() {
                upstream_total_count = Some(body.total_count);
            }
            if body.items.is_empty() {
                break;
            }
            fetched_raw_count += body.items.len();

            for item in body.items {
                if seen_ids.insert(item.id) {
                    out.push(GithubLatestPushedRepoInfo {
                        id: item.id,
                        full_name: item.full_name,
                        stargazers_count: item.stargazers_count,
                        created_at: item.created_at,
                        pushed_at: item.pushed_at,
                    });
                }
            }

            if out.len() >= capped_limit {
                break;
            }
        }

        out.sort_by_key(|item| Reverse(item.created_at.clone()));
        let unique_count = out.len();
        out.truncate(capped_limit);
        Ok(GithubLatestPushedRepoSearchResult {
            requested_limit: capped_limit,
            upstream_total_count,
            fetched_raw_count,
            unique_count,
            items: out,
        })
    }

    async fn fetch_readme(&self, full_name: &str) -> AppResult<Option<GithubReadme>> {
        let url = format!("https://api.github.com/repos/{full_name}/readme");

        let resp = self
            .authorized_get(url)
            .send()
            .await
            .map_err(AppError::upstream)?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let body: GithubReadmeResponse = resp
            .error_for_status()
            .map_err(AppError::upstream)?
            .json()
            .await
            .map_err(AppError::upstream)?;

        let content = Self::decode_readme_content(&body.content, &body.encoding)?;

        Ok(Some(GithubReadme {
            content,
            html_url: body.html_url,
            download_url: body.download_url,
        }))
    }
}
