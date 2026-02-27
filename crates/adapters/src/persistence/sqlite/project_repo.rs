use super::db_err;
use app::app_error::AppResult;
use app::common::pagination::{Page, Pagination};
use app::project::ProjectRepo;
use domain::{Project, RepoId};
use sqlx::{QueryBuilder, Sqlite};

#[derive(Debug, sqlx::FromRow)]
struct ProjectDb {
    repo_id: String,
    name: String,
    slug: String,
    description: String,
    url: Option<String>,
    avatar_url: Option<String>,
    status: Option<String>,
    logo: Option<String>,
    twitter: Option<String>,
}

impl From<ProjectDb> for Project {
    fn from(db: ProjectDb) -> Self {
        Self {
            id: RepoId::new_unchecked(db.repo_id),
            name: db.name,
            slug: db.slug,
            description: db.description,
            url: db.url,
            avatar_url: db.avatar_url,
            status: db.status,
            logo: db.logo,
            twitter: db.twitter,
        }
    }
}

#[derive(Clone)]
pub struct SqliteProjectRepo {
    pool: sqlx::SqlitePool,
}

impl SqliteProjectRepo {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ProjectRepo for SqliteProjectRepo {
    async fn upsert(&self, project: &Project) -> AppResult<()> {
        self.upsert_many(std::slice::from_ref(project)).await
    }

    async fn upsert_many(&self, items: &[Project]) -> AppResult<()> {
        if items.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await.map_err(db_err)?;

        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            INSERT INTO projects (
              repo_id,
              name, slug, description,
              url, avatar_url,
              status, logo, twitter,
              updated_at
            )
            "#,
        );

        builder.push_values(items, |mut b, p| {
            b.push_bind(p.id.as_str())
                .push_bind(&p.name)
                .push_bind(&p.slug)
                .push_bind(&p.description)
                .push_bind(&p.url)
                .push_bind(&p.avatar_url)
                .push_bind(&p.status)
                .push_bind(&p.logo)
                .push_bind(&p.twitter)
                .push("datetime('now')");
        });

        builder.push(
            r#"
            ON CONFLICT(repo_id) DO UPDATE SET
              name = excluded.name,
              slug = excluded.slug,
              description = excluded.description,
              url = excluded.url,
              avatar_url = excluded.avatar_url,
              status = excluded.status,
              logo = excluded.logo,
              twitter = excluded.twitter,
              updated_at = excluded.updated_at
            "#,
        );

        builder.build().execute(&mut *tx).await.map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;

        Ok(())
    }

    async fn list(&self, page: Pagination) -> AppResult<Page<Project>> {
        let limit = page.limit();
        let offset = page.offset();
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects")
            .fetch_one(&self.pool)
            .await
            .map_err(db_err)?;

        let rows: Vec<ProjectDb> = sqlx::query_as(
            r#"
            SELECT
              repo_id,
              name, slug, description,
              url, avatar_url,
              status, logo, twitter
            FROM projects
            ORDER BY name ASC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        let items = rows.into_iter().map(Into::into).collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn search_by_key(&self, key: String, page: Pagination) -> AppResult<Page<Project>> {
        let limit = page.limit();
        let offset = page.offset();
        let key = key.trim();
        if key.is_empty() {
            return Ok(page.to_page(Vec::new(), 0));
        }
        let pattern = format!("%{key}%");

        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM projects
            WHERE repo_id LIKE ? OR name LIKE ? OR slug LIKE ? OR description LIKE ?
            "#,
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;

        let rows: Vec<ProjectDb> = sqlx::query_as(
            r#"
            SELECT
              repo_id,
              name, slug, description,
              url, avatar_url,
              status, logo, twitter
            FROM projects
            WHERE repo_id LIKE ? OR name LIKE ? OR slug LIKE ? OR description LIKE ?
            ORDER BY name ASC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let items = rows.into_iter().map(Into::into).collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn remove(&self, repo_id: String) -> AppResult<()> {
        sqlx::query("DELETE FROM projects WHERE repo_id = ?")
            .bind(repo_id)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }
}
