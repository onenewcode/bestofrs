use std::slice::from_ref;
use std::sync::Arc;
use std::{collections::HashMap, collections::HashSet};

use crate::app_error::AppResult;
use crate::repo::{RepoRepo, RepoTagRepo};
use domain::{Repo, RepoId, Tag, TagLabel, TagValue};

#[derive(Clone)]
pub struct RepoCommandHandler {
    repos: Arc<dyn RepoRepo>,
    repo_tags: Arc<dyn RepoTagRepo>,
}

#[derive(Debug, Clone)]
pub struct TagInput {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ImportTagCommand {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImportTagsCommand {
    pub items: Vec<ImportTagCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportTagsReport {
    pub total: usize,
    pub upserted: usize,
    pub skipped_invalid: usize,
    pub failed_upsert: usize,
    pub invalid_examples: Vec<String>,
    pub error_examples: Vec<String>,
}

impl ImportTagsReport {
    fn new(total: usize) -> Self {
        Self {
            total,
            upserted: 0,
            skipped_invalid: 0,
            failed_upsert: 0,
            invalid_examples: Vec::new(),
            error_examples: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReplaceRepoTagsCommand {
    pub repo_id: String,
    pub tags: Vec<TagInput>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulkTagUpdateAction {
    Add,
    Remove,
}

#[derive(Debug, Clone)]
pub struct BulkUpdateRepoTagCommand {
    pub repo_ids: Vec<String>,
    pub tag: TagInput,
    pub action: BulkTagUpdateAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BulkUpdateRepoTagResult {
    pub total: usize,
    pub updated: usize,
    pub skipped: usize,
}

impl RepoCommandHandler {
    pub fn new(repos: Arc<dyn RepoRepo>, repo_tags: Arc<dyn RepoTagRepo>) -> Self {
        Self { repos, repo_tags }
    }
    pub async fn upsert(&self, repo: &Repo) -> AppResult<()> {
        self.upsert_many(from_ref(repo)).await
    }

    pub async fn upsert_many(&self, repos: &[Repo]) -> AppResult<()> {
        if repos.is_empty() {
            return Ok(());
        }
        self.repos.upsert_many(repos).await?;
        Ok(())
    }

    pub async fn replace_tags(&self, repo_id: &RepoId, tags: &[Tag]) -> AppResult<()> {
        let tags = Self::normalize_business_tags(tags.to_vec());
        self.repo_tags.replace_repo_tags(repo_id, &tags).await
    }

    pub async fn replace_tags_by_repo_id(&self, cmd: ReplaceRepoTagsCommand) -> AppResult<()> {
        let repo_id = RepoId::parse(&cmd.repo_id)?;
        let tags = cmd
            .tags
            .into_iter()
            .map(|tag| Tag {
                label: TagLabel::new(tag.label),
                value: TagValue::new(tag.value),
                description: None,
            })
            .collect::<Vec<_>>();
        self.replace_tags(&repo_id, &tags).await
    }

    pub async fn create_tag(&self, tag: TagInput) -> AppResult<()> {
        let tag = Tag {
            label: TagLabel::new(tag.label),
            value: TagValue::new(tag.value),
            description: None,
        };
        self.repo_tags.upsert_tag(&tag).await
    }

    pub async fn update_tag(
        &self,
        label: String,
        value: String,
        description: Option<String>,
    ) -> AppResult<()> {
        let tag = Tag {
            label: TagLabel::new(label),
            value: TagValue::new(value),
            description,
        };
        self.repo_tags.update_tag(&tag).await
    }

    pub async fn delete_tag(&self, tag: TagInput) -> AppResult<()> {
        let tag = Tag {
            label: TagLabel::new(tag.label),
            value: TagValue::new(tag.value),
            description: None,
        };
        self.repo_tags.delete_tag(&tag).await
    }

    pub async fn import_tags(&self, cmd: ImportTagsCommand) -> AppResult<ImportTagsReport> {
        const MAX_INVALID_EXAMPLES: usize = 20;
        const MAX_ERROR_EXAMPLES: usize = 20;

        let mut report = ImportTagsReport::new(cmd.items.len());
        for item in cmd.items {
            let label = item.label.trim().to_string();
            let value = item.value.trim().to_string();
            if label.is_empty() || value.is_empty() {
                report.skipped_invalid += 1;
                if report.invalid_examples.len() < MAX_INVALID_EXAMPLES {
                    report
                        .invalid_examples
                        .push(format!("{}:{}", item.label, item.value));
                }
                continue;
            }

            let create_result = self
                .create_tag(TagInput {
                    label: label.clone(),
                    value: value.clone(),
                })
                .await;
            if let Err(err) = create_result {
                report.failed_upsert += 1;
                if report.error_examples.len() < MAX_ERROR_EXAMPLES {
                    report.error_examples.push(err.to_string());
                }
                continue;
            }

            let description = item
                .description
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty());
            let update_result = self.update_tag(label, value, description).await;
            if let Err(err) = update_result {
                report.failed_upsert += 1;
                if report.error_examples.len() < MAX_ERROR_EXAMPLES {
                    report.error_examples.push(err.to_string());
                }
                continue;
            }

            report.upserted += 1;
        }

        Ok(report)
    }

    pub async fn bulk_update_tag_for_repos(
        &self,
        cmd: BulkUpdateRepoTagCommand,
    ) -> AppResult<BulkUpdateRepoTagResult> {
        if cmd.repo_ids.is_empty() {
            return Ok(BulkUpdateRepoTagResult {
                total: 0,
                updated: 0,
                skipped: 0,
            });
        }

        let target_tag = Tag {
            label: TagLabel::new(cmd.tag.label),
            value: TagValue::new(cmd.tag.value),
            description: None,
        };

        let mut dedup = HashSet::new();
        let mut repo_ids = Vec::new();
        for raw_repo_id in cmd.repo_ids {
            let repo_id = RepoId::parse(&raw_repo_id)?;
            let key = repo_id.as_str().to_string();
            if dedup.insert(key) {
                repo_ids.push(repo_id);
            }
        }

        let existing_pairs = self.repo_tags.list_by_repo_ids(&repo_ids).await?;
        let mut tags_by_repo: HashMap<String, Vec<Tag>> = HashMap::new();
        for (repo_id, tag) in existing_pairs {
            tags_by_repo
                .entry(repo_id.as_str().to_string())
                .or_default()
                .push(tag);
        }

        let mut updated = 0usize;
        let mut skipped = 0usize;

        for repo_id in &repo_ids {
            let key = repo_id.as_str().to_string();
            let current_tags =
                Self::normalize_business_tags(tags_by_repo.remove(&key).unwrap_or_default());
            let mut next_tags = current_tags.clone();

            match cmd.action {
                BulkTagUpdateAction::Add => {
                    if next_tags.contains(&target_tag) {
                        skipped += 1;
                        continue;
                    }
                    next_tags.push(target_tag.clone());
                }
                BulkTagUpdateAction::Remove => {
                    let before = next_tags.len();
                    next_tags.retain(|tag| tag != &target_tag);
                    if before == next_tags.len() {
                        skipped += 1;
                        continue;
                    }
                }
            }

            next_tags = Self::normalize_business_tags(next_tags);
            self.repo_tags
                .replace_repo_tags(repo_id, &next_tags)
                .await?;
            updated += 1;
        }

        Ok(BulkUpdateRepoTagResult {
            total: repo_ids.len(),
            updated,
            skipped,
        })
    }

    fn normalize_business_tags(tags: Vec<Tag>) -> Vec<Tag> {
        let mut dedup = HashSet::new();
        let mut out = Vec::new();
        for tag in tags {
            let key = format!("{}:{}", tag.label.as_str(), tag.value.as_str());
            if dedup.insert(key) {
                out.push(tag);
            }
        }
        out
    }
}
