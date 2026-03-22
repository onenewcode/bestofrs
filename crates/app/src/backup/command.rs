use std::sync::Arc;

use crate::app_error::{AppError, AppResult};
use crate::backup::{BackupEntry, DatabaseBackupPort};

#[derive(Clone)]
pub struct BackupCommandHandler {
    backup: Arc<dyn DatabaseBackupPort>,
    allow_restore: bool,
    retain_last: usize,
}

impl BackupCommandHandler {
    pub fn new(
        backup: Arc<dyn DatabaseBackupPort>,
        allow_restore: bool,
        retain_last: usize,
    ) -> Self {
        Self {
            backup,
            allow_restore,
            retain_last,
        }
    }

    pub async fn create_backup(&self, label: Option<String>) -> AppResult<BackupEntry> {
        let label = sanitize_label(label.as_deref().unwrap_or_default())?;
        let created = self.backup.create_backup(&label).await?;

        if self.retain_last > 0 {
            let mut backups = self.backup.list_backups().await?;
            backups.sort_by(|a, b| b.name.cmp(&a.name));
            let stale = backups
                .into_iter()
                .skip(self.retain_last)
                .collect::<Vec<_>>();
            for item in stale {
                self.backup.delete_backup(&item.name).await?;
            }
        }

        Ok(created)
    }

    pub async fn delete_backup(&self, name: String) -> AppResult<()> {
        validate_backup_name(&name)?;
        self.backup.delete_backup(&name).await
    }

    pub async fn restore_backup(&self, name: String) -> AppResult<()> {
        if !self.allow_restore {
            return Err(AppError::internal(
                "Backup restore is disabled by configuration",
            ));
        }
        validate_backup_name(&name)?;
        self.backup.restore_backup(&name).await
    }
}

fn sanitize_label(raw: &str) -> AppResult<String> {
    let candidate = raw.trim();
    if candidate.is_empty() {
        return Ok("manual".to_string());
    }
    if candidate.len() > 64 {
        return Err(AppError::Domain(
            "Backup label must be <= 64 characters".to_string(),
        ));
    }
    if candidate
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        return Ok(candidate.to_string());
    }

    Err(AppError::Domain(
        "Backup label only supports [A-Za-z0-9_-]".to_string(),
    ))
}

fn validate_backup_name(name: &str) -> AppResult<()> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(AppError::Domain("Backup name is required".to_string()));
    }
    if trimmed.len() > 200 {
        return Err(AppError::Domain("Backup name is too long".to_string()));
    }
    if trimmed.contains('/') || trimmed.contains('\\') || trimmed.contains("..") {
        return Err(AppError::Domain("Invalid backup name".to_string()));
    }
    Ok(())
}
