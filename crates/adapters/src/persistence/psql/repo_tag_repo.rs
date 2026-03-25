use app::app_error::AppResult;
use app::common::pagination::{Page, Pagination};
use app::repo::{build_avatar_urls, RepoTagFacet, RepoTagListItem, RepoTagRepo, RepoTagTopRepo};
use async_trait::async_trait;
use domain::{RepoId, Tag, TagLabel, TagValue};
use sqlx::{Postgres, QueryBuilder};
use std::collections::{HashMap, HashSet};

use super::db_err;

fn tag_id(label: &str, value: &str) -> String {
    format!("tag:{label}:{value}")
}

#[derive(Debug, sqlx::FromRow)]
struct RepoTagRow {
    repo_id: String,
    label: String,
    value: String,
}

impl RepoTagRow {
    fn into_pair(self) -> (RepoId, Tag) {
        let repo_id = RepoId::new_unchecked(self.repo_id);
        let tag = Tag {
            label: TagLabel::new(self.label),
            value: TagValue::new(self.value),
            description: None,
        };
        (repo_id, tag)
    }
}

#[derive(Debug, sqlx::FromRow)]
struct TagRow {
    id: String,
    label: String,
    value: String,
    description: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct TagTopRepoRow {
    tag_id: String,
    repo_id: String,
    avatar_url: Option<String>,
    homepage_url: Option<String>,
}

#[derive(Clone)]
pub struct PostgresRepoTagRepo {
    pool: sqlx::PgPool,
}

impl PostgresRepoTagRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    async fn repo_totals_by_tag_ids(&self, tag_ids: &[String]) -> AppResult<HashMap<String, u64>> {
        if tag_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT m.tag_id, COUNT(DISTINCT m.repo_id) AS total \
             FROM repo_tag_map m WHERE m.tag_id IN (",
        );
        let mut first = true;
        for tag_id in tag_ids {
            if !first {
                builder.push(", ");
            }
            first = false;
            builder.push_bind(tag_id);
        }
        builder.push(") GROUP BY m.tag_id");

        let rows: Vec<(String, i64)> = builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;

        let mut totals = HashMap::new();
        for (tag_id, total) in rows {
            totals.insert(tag_id, total.max(0) as u64);
        }
        Ok(totals)
    }
}

#[async_trait]
impl RepoTagRepo for PostgresRepoTagRepo {
    async fn replace_repo_tags(&self, repo_id: &RepoId, tags: &[Tag]) -> AppResult<()> {
        self.replace_repo_tags_bulk(&[(repo_id.clone(), tags.to_vec())]).await
    }

    async fn replace_repo_tags_bulk(&self, items: &[(RepoId, Vec<Tag>)]) -> AppResult<()> {
        if items.is_empty() {
            return Ok(());
        }
        let mut tx = self.pool.begin().await.map_err(db_err)?;
        let repo_ids = items.iter().map(|(repo_id, _)| repo_id).collect::<Vec<_>>();
        let mut delete_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("DELETE FROM repo_tag_map WHERE repo_id IN (");
        let mut delete_separated = delete_builder.separated(", ");
        for repo_id in &repo_ids {
            delete_separated.push_bind(repo_id.as_str());
        }
        delete_separated.push_unseparated(")");
        delete_builder
            .build()
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        let mut unique_tags = Vec::new();
        let mut seen_tag_ids = HashSet::new();
        let mut mappings: Vec<(String, String)> = Vec::new();
        let mut seen_mappings = HashSet::new();
        for (repo_id, tags) in items {
            for tag in tags {
                let id = tag_id(tag.label.as_str(), tag.value.as_str());
                if seen_tag_ids.insert(id.clone()) {
                    unique_tags.push((id.clone(), tag.clone()));
                }
                let mapping_key = format!("{}|{}", repo_id.as_str(), id);
                if seen_mappings.insert(mapping_key) {
                    mappings.push((repo_id.as_str().to_string(), id));
                }
            }
        }

        if !unique_tags.is_empty() {
            let mut tag_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO tags (id, label, value, description) ");
            tag_builder.push_values(&unique_tags, |mut b, (id, tag)| {
                b.push_bind(id)
                    .push_bind(tag.label.as_str())
                    .push_bind(tag.value.as_str())
                    .push_bind(tag.description.clone());
            });
            tag_builder.push(" ON CONFLICT(id) DO NOTHING");
            tag_builder
                .build()
                .execute(&mut *tx)
                .await
                .map_err(db_err)?;
        }

        if !mappings.is_empty() {
            let mut map_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO repo_tag_map (repo_id, tag_id, source) ");
            map_builder.push_values(&mappings, |mut b, (repo_id, tag_id)| {
                b.push_bind(repo_id).push_bind(tag_id).push_bind("manual");
            });
            map_builder.push(" ON CONFLICT(repo_id, tag_id) DO NOTHING");
            map_builder
                .build()
                .execute(&mut *tx)
                .await
                .map_err(db_err)?;
        }

        tx.commit().await.map_err(db_err)?;
        Ok(())
    }

    async fn upsert_tag(&self, tag: &Tag) -> AppResult<()> {
        let id = tag_id(tag.label.as_str(), tag.value.as_str());
        sqlx::query(
            "INSERT INTO tags (id, label, value, description) VALUES ($1, $2, $3, $4) \
             ON CONFLICT(id) DO UPDATE SET label = excluded.label, value = excluded.value, description = excluded.description",
        )
        .bind(id)
        .bind(tag.label.as_str())
        .bind(tag.value.as_str())
        .bind(tag.description.clone())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn update_tag(&self, tag: &Tag) -> AppResult<()> {
        let id = tag_id(tag.label.as_str(), tag.value.as_str());
        sqlx::query("UPDATE tags SET description = $1 WHERE id = $2")
            .bind(tag.description.clone())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_tag(&self, tag: &Tag) -> AppResult<()> {
        let id = tag_id(tag.label.as_str(), tag.value.as_str());
        let mut tx = self.pool.begin().await.map_err(db_err)?;
        sqlx::query("DELETE FROM repo_tag_map WHERE tag_id = $1")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
        sqlx::query("DELETE FROM tags WHERE id = $1")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
        tx.commit().await.map_err(db_err)?;
        Ok(())
    }

    async fn list_by_repo_ids(&self, repo_ids: &[RepoId]) -> AppResult<Vec<(RepoId, Tag)>> {
        if repo_ids.is_empty() {
            return Ok(Vec::new());
        }
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT m.repo_id, t.label, t.value FROM repo_tag_map m \
             JOIN tags t ON t.id = m.tag_id WHERE m.repo_id IN (",
        );
        let mut first = true;
        for repo_id in repo_ids {
            if !first {
                builder.push(", ");
            }
            first = false;
            builder.push_bind(repo_id.as_str());
        }
        builder.push(")");
        let rows: Vec<RepoTagRow> = builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(rows.into_iter().map(RepoTagRow::into_pair).collect())
    }

    async fn find_tags_by_values(&self, values: &[String]) -> AppResult<Vec<Tag>> {
        if values.is_empty() {
            return Ok(Vec::new());
        }
        let mut builder: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT id, label, value, description FROM tags WHERE value IN (");
        let mut separated = builder.separated(", ");
        for value in values {
            separated.push_bind(value);
        }
        separated.push_unseparated(")");
        let rows: Vec<TagRow> = builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        let tags = rows
            .into_iter()
            .map(|row| Tag {
                label: TagLabel::new(row.label),
                value: TagValue::new(row.value),
                description: row.description,
            })
            .collect();
        Ok(tags)
    }

    async fn list_repo_ids_without_tags(&self, page: Pagination) -> AppResult<Page<RepoId>> {
        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM repos r
            WHERE NOT EXISTS (
              SELECT 1 FROM repo_tag_map m WHERE m.repo_id = r.id
            )
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;

        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
            SELECT r.id
            FROM repos r
            WHERE NOT EXISTS (
              SELECT 1 FROM repo_tag_map m WHERE m.repo_id = r.id
            )
            ORDER BY r.id
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(page.limit() as i64)
        .bind(page.offset() as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let items = rows
            .into_iter()
            .map(|(repo_id,)| RepoId::new_unchecked(repo_id))
            .collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn list_repo_ids_by_label(
        &self,
        label: &str,
        value: Option<&str>,
        page: Pagination,
    ) -> AppResult<Page<RepoId>> {
        let mut count_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT COUNT(DISTINCT m.repo_id) FROM repo_tag_map m \
             JOIN tags t ON t.id = m.tag_id WHERE t.label = ",
        );
        count_builder.push_bind(label);
        if let Some(value) = value {
            count_builder.push(" AND t.value = ");
            count_builder.push_bind(value);
        }
        let total: i64 = count_builder
            .build_query_scalar()
            .fetch_one(&self.pool)
            .await
            .map_err(db_err)?;

        let limit = page.limit();
        let offset = page.offset();
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT DISTINCT m.repo_id FROM repo_tag_map m \
             JOIN tags t ON t.id = m.tag_id WHERE t.label = ",
        );
        builder.push_bind(label);
        if let Some(value) = value {
            builder.push(" AND t.value = ");
            builder.push_bind(value);
        }
        builder.push(" ORDER BY m.repo_id LIMIT ");
        builder.push_bind(limit as i64);
        builder.push(" OFFSET ");
        builder.push_bind(offset as i64);
        let rows: Vec<(String,)> = builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        let items = rows
            .into_iter()
            .map(|(repo_id,)| RepoId::new_unchecked(repo_id))
            .collect();
        Ok(page.to_page(items, total as u64))
    }
    async fn list_tags(&self, page: Pagination) -> AppResult<Page<Tag>> {
        let limit = page.limit();
        let offset = page.offset();
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tags")
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;

        let rows: Vec<(String, String, Option<String>)> = sqlx::query_as(
            r#"
            SELECT label, value, description
            FROM tags
            ORDER BY label, value
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let items = rows
            .into_iter()
            .map(|(label, value, description)| Tag {
                label: TagLabel::new(label),
                value: TagValue::new(value),
                description,
            })
            .collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn list_tags_with_meta(
        &self,
        page: Pagination,
        top_n: u32,
    ) -> AppResult<Page<RepoTagListItem>> {
        let limit = page.limit();
        let offset = page.offset();
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tags")
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;

        let tag_rows: Vec<TagRow> = sqlx::query_as(
            r#"
            SELECT id, label, value, description
            FROM tags
            ORDER BY label, value
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let mut repo_total_by_tag: HashMap<String, u64> = HashMap::new();
        if !tag_rows.is_empty() {
            let tag_ids = tag_rows.iter().map(|row| row.id.clone()).collect::<Vec<_>>();
            repo_total_by_tag = self.repo_totals_by_tag_ids(&tag_ids).await?;
        }

        let mut top_by_tag: HashMap<String, Vec<RepoTagTopRepo>> = HashMap::new();
        if !tag_rows.is_empty() && top_n > 0 {
            let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "WITH ranked AS (\
                 SELECT m.tag_id, r.id AS repo_id, r.avatar_url, r.homepage_url, \
                 ROW_NUMBER() OVER (PARTITION BY m.tag_id ORDER BY r.stars DESC, r.id ASC) AS rn \
                 FROM repo_tag_map m \
                 JOIN repos r ON r.id = m.repo_id \
                 WHERE m.tag_id IN (",
            );
            let mut first = true;
            for row in &tag_rows {
                if !first {
                    builder.push(", ");
                }
                first = false;
                builder.push_bind(row.id.clone());
            }
            builder.push(
                ") ) SELECT tag_id, repo_id, avatar_url, homepage_url FROM ranked WHERE rn <= ",
            );
            builder.push_bind(top_n as i64);
            builder.push(" ORDER BY tag_id, rn");
            let rows: Vec<TagTopRepoRow> = builder
                .build_query_as()
                .fetch_all(&self.pool)
                .await
                .map_err(db_err)?;
            for row in rows {
                top_by_tag
                    .entry(row.tag_id)
                    .or_default()
                    .push(RepoTagTopRepo {
                        avatar_urls: build_avatar_urls(
                            &row.repo_id,
                            row.avatar_url.as_deref(),
                            row.homepage_url.as_deref(),
                        ),
                        repo_id: row.repo_id,
                    });
            }
        }

        let items = tag_rows
            .into_iter()
            .map(|row| RepoTagListItem {
                label: row.label,
                value: row.value,
                description: row.description,
                repos_total: repo_total_by_tag.remove(&row.id).unwrap_or(0),
                top_repos: top_by_tag.remove(&row.id).unwrap_or_default(),
            })
            .collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn list_tags_with_meta_by_values(
        &self,
        values: &[String],
        top_n: u32,
    ) -> AppResult<Vec<RepoTagListItem>> {
        if values.is_empty() {
            return Ok(Vec::new());
        }

        let mut builder: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT id, label, value, description FROM tags WHERE value IN (");
        let mut separated = builder.separated(", ");
        for value in values {
            separated.push_bind(value);
        }
        separated.push_unseparated(")");
        builder.push(" ORDER BY label, value");
        let tag_rows: Vec<TagRow> = builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;

        let mut repo_total_by_tag: HashMap<String, u64> = HashMap::new();
        if !tag_rows.is_empty() {
            let tag_ids = tag_rows.iter().map(|row| row.id.clone()).collect::<Vec<_>>();
            repo_total_by_tag = self.repo_totals_by_tag_ids(&tag_ids).await?;
        }

        let mut top_by_tag: HashMap<String, Vec<RepoTagTopRepo>> = HashMap::new();
        if !tag_rows.is_empty() && top_n > 0 {
            let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "WITH ranked AS (\
                 SELECT m.tag_id, r.id AS repo_id, r.avatar_url, r.homepage_url, \
                 ROW_NUMBER() OVER (PARTITION BY m.tag_id ORDER BY r.stars DESC, r.id ASC) AS rn \
                 FROM repo_tag_map m \
                 JOIN repos r ON r.id = m.repo_id \
                 WHERE m.tag_id IN (",
            );
            let mut first = true;
            for row in &tag_rows {
                if !first {
                    builder.push(", ");
                }
                first = false;
                builder.push_bind(row.id.clone());
            }
            builder.push(
                ") ) SELECT tag_id, repo_id, avatar_url, homepage_url FROM ranked WHERE rn <= ",
            );
            builder.push_bind(top_n as i64);
            builder.push(" ORDER BY tag_id, rn");
            let rows: Vec<TagTopRepoRow> = builder
                .build_query_as()
                .fetch_all(&self.pool)
                .await
                .map_err(db_err)?;
            for row in rows {
                top_by_tag
                    .entry(row.tag_id)
                    .or_default()
                    .push(RepoTagTopRepo {
                        avatar_urls: build_avatar_urls(
                            &row.repo_id,
                            row.avatar_url.as_deref(),
                            row.homepage_url.as_deref(),
                        ),
                        repo_id: row.repo_id,
                    });
            }
        }

        Ok(tag_rows
            .into_iter()
            .map(|row| RepoTagListItem {
                label: row.label,
                value: row.value,
                description: row.description,
                repos_total: repo_total_by_tag.remove(&row.id).unwrap_or(0),
                top_repos: top_by_tag.remove(&row.id).unwrap_or_default(),
            })
            .collect())
    }

    async fn search_tags_by_key(&self, key: &str, page: Pagination) -> AppResult<Page<Tag>> {
        let key = format!("%{key}%");
        let limit = page.limit();
        let offset = page.offset();
        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM tags
            WHERE label ILIKE $1 OR value ILIKE $1
            "#,
        )
        .bind(&key)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;

        let rows: Vec<(String, String, Option<String>)> = sqlx::query_as(
            r#"
            SELECT label, value, description
            FROM tags
            WHERE label ILIKE $1 OR value ILIKE $1
            ORDER BY label, value
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&key)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let items = rows
            .into_iter()
            .map(|(label, value, description)| Tag {
                label: TagLabel::new(label),
                value: TagValue::new(value),
                description,
            })
            .collect();
        Ok(page.to_page(items, total as u64))
    }

    async fn count_repos_by_tags(
        &self,
        tags: &[Tag],
    ) -> AppResult<HashMap<(String, String), u64>> {
        if tags.is_empty() {
            return Ok(HashMap::new());
        }

        let mut unique_pairs: Vec<(String, String)> = Vec::new();
        let mut seen = HashSet::new();
        let mut pair_by_id: HashMap<String, (String, String)> = HashMap::new();
        for tag in tags {
            let label = tag.label.as_str().to_string();
            let value = tag.value.as_str().to_string();
            if seen.insert((label.clone(), value.clone())) {
                unique_pairs.push((label, value));
            }
        }
        let tag_ids = unique_pairs
            .iter()
            .map(|(label, value)| {
                let id = tag_id(label, value);
                pair_by_id.insert(id.clone(), (label.clone(), value.clone()));
                id
            })
            .collect::<Vec<_>>();
        let by_tag_id = self.repo_totals_by_tag_ids(&tag_ids).await?;
        let mut totals = HashMap::new();
        for (tag_id, total) in by_tag_id {
            if let Some((label, value)) = pair_by_id.remove(&tag_id) {
                totals.insert((label, value), total);
            }
        }
        Ok(totals)
    }

    async fn list_tag_facets_by_active_values(
        &self,
        active_values: &[String],
        limit: Option<u32>,
    ) -> AppResult<Vec<RepoTagFacet>> {
        let mut builder: QueryBuilder<Postgres>;
        if active_values.is_empty() {
            builder = QueryBuilder::new(
                "SELECT t.value, COUNT(DISTINCT m.repo_id) AS cnt \
                 FROM repo_tag_map m \
                 JOIN tags t ON t.id = m.tag_id \
                 GROUP BY t.value ORDER BY cnt DESC, t.value ASC",
            );
        } else {
            builder = QueryBuilder::new(
                "WITH matched_repos AS (\
                 SELECT m.repo_id \
                 FROM repo_tag_map m \
                 JOIN tags t ON t.id = m.tag_id \
                 WHERE t.value IN (",
            );
            let mut first = true;
            for value in active_values {
                if !first {
                    builder.push(", ");
                }
                first = false;
                builder.push_bind(value);
            }
            builder.push(") GROUP BY m.repo_id HAVING COUNT(DISTINCT t.value) = ");
            builder.push_bind(active_values.len() as i64);
            builder.push(
                ") SELECT t.value, COUNT(DISTINCT m.repo_id) AS cnt \
                 FROM repo_tag_map m \
                 JOIN tags t ON t.id = m.tag_id \
                 JOIN matched_repos mr ON mr.repo_id = m.repo_id \
                 WHERE t.value NOT IN (",
            );
            let mut first = true;
            for value in active_values {
                if !first {
                    builder.push(", ");
                }
                first = false;
                builder.push_bind(value);
            }
            builder.push(") GROUP BY t.value ORDER BY cnt DESC, t.value ASC");
        }
        if let Some(limit) = limit {
            builder.push(" LIMIT ");
            builder.push_bind(limit as i64);
        }
        let rows: Vec<(String, i64)> = builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|(value, count)| RepoTagFacet {
                value,
                count: count.max(0) as u64,
            })
            .collect())
    }
}
