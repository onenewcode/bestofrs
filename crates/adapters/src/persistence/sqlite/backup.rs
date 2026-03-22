use std::path::{Path, PathBuf};
use std::str::FromStr;

use app::app_error::{AppError, AppResult};
use app::backup::{BackupEntry, DatabaseBackupPort};
use async_trait::async_trait;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{query, SqlitePool};

pub struct SqliteBackupAdapter {
    pool: SqlitePool,
    backup_dir: PathBuf,
}

impl SqliteBackupAdapter {
    pub fn from_pool(pool: SqlitePool, backup_dir: &str) -> Self {
        Self {
            pool,
            backup_dir: backup_dir.into(),
        }
    }

    pub async fn new(database_url: &str, backup_dir: &str) -> AppResult<Self> {
        let opts = SqliteConnectOptions::from_str(database_url).map_err(AppError::internal)?;
        let pool = SqlitePool::connect_with(opts)
            .await
            .map_err(AppError::database)?;
        Ok(Self::from_pool(pool, backup_dir))
    }
}

#[async_trait]
impl DatabaseBackupPort for SqliteBackupAdapter {
    async fn create_backup(&self, label: &str) -> AppResult<BackupEntry> {
        std::fs::create_dir_all(&self.backup_dir).map_err(AppError::internal)?;
        let name = format!("{}__{}.sqlite3", now_stamp(), label);
        let path = self.backup_dir.join(&name);

        let escaped = path.to_string_lossy().replace('\'', "''");
        let sql = format!("VACUUM INTO '{escaped}'");
        query(sql.as_str())
            .execute(&self.pool)
            .await
            .map_err(AppError::database)?;

        let metadata = std::fs::metadata(&path).map_err(AppError::internal)?;
        Ok(BackupEntry {
            name,
            created_at_utc: chrono::Utc::now().to_rfc3339(),
            size_bytes: metadata.len(),
        })
    }

    async fn list_backups(&self) -> AppResult<Vec<BackupEntry>> {
        list_backups_from_dir(&self.backup_dir, "sqlite3")
    }

    async fn delete_backup(&self, name: &str) -> AppResult<()> {
        let path = resolve_backup_file(&self.backup_dir, name)?;
        if path.exists() {
            std::fs::remove_file(path).map_err(AppError::internal)?;
        }
        Ok(())
    }

    async fn restore_backup(&self, _name: &str) -> AppResult<()> {
        Err(AppError::internal(
            "SQLite restore is not supported in-process; replace DB file offline",
        ))
    }
}

fn list_backups_from_dir(backup_dir: &Path, expected_ext: &str) -> AppResult<Vec<BackupEntry>> {
    let mut items = Vec::new();
    if !backup_dir.exists() {
        return Ok(items);
    }

    for entry in std::fs::read_dir(backup_dir).map_err(AppError::internal)? {
        let entry = entry.map_err(AppError::internal)?;
        let path = entry.path();
        let ext = path.extension().and_then(|v| v.to_str()).unwrap_or_default();
        if ext != expected_ext {
            continue;
        }
        let Some(name) = path
            .file_name()
            .and_then(|v| v.to_str())
            .map(|v| v.to_string())
        else {
            continue;
        };
        let metadata = entry.metadata().map_err(AppError::internal)?;
        let modified = metadata.modified().map_err(AppError::internal)?;
        let created_at_utc = chrono::DateTime::<chrono::Utc>::from(modified).to_rfc3339();
        items.push(BackupEntry {
            name,
            created_at_utc,
            size_bytes: metadata.len(),
        });
    }

    Ok(items)
}

fn resolve_backup_file(backup_dir: &Path, name: &str) -> AppResult<PathBuf> {
    let path = backup_dir.join(name);
    if path
        .file_name()
        .and_then(|v| v.to_str())
        .filter(|filename| *filename == name)
        .is_none()
    {
        return Err(AppError::internal("Invalid backup file name"));
    }
    Ok(path)
}

fn now_stamp() -> String {
    chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string()
}
