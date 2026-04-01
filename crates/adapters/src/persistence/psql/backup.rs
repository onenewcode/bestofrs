use std::path::{Path, PathBuf};
use std::process::Stdio;

use app::app_error::{AppError, AppResult};
use app::backup::{BackupEntry, DatabaseBackupPort};
use async_trait::async_trait;
use tokio::process::Command;

pub struct PostgresBackupAdapter {
    database_url: String,
    backup_dir: PathBuf,
}

impl PostgresBackupAdapter {
    pub fn new(database_url: String, backup_dir: PathBuf) -> Self {
        Self {
            database_url,
            backup_dir,
        }
    }
}

#[async_trait]
impl DatabaseBackupPort for PostgresBackupAdapter {
    async fn create_backup(&self, label: &str) -> AppResult<BackupEntry> {
        std::fs::create_dir_all(&self.backup_dir).map_err(AppError::internal)?;
        let name = format!("{}__{}.dump", now_stamp(), label);
        let path = self.backup_dir.join(&name);

        let output = Command::new("pg_dump")
            .arg("--dbname")
            .arg(&self.database_url)
            .arg("--file")
            .arg(&path)
            .arg("--format=custom")
            .arg("--no-owner")
            .arg("--no-privileges")
            .stdin(Stdio::null())
            .output()
            .await
            .map_err(map_command_error("pg_dump"))?;

        if !output.status.success() {
            return Err(AppError::internal(format!(
                "pg_dump failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let metadata = std::fs::metadata(&path).map_err(AppError::internal)?;

        Ok(BackupEntry {
            name,
            created_at_utc: chrono::Utc::now().to_rfc3339(),
            size_bytes: metadata.len(),
        })
    }

    async fn list_backups(&self) -> AppResult<Vec<BackupEntry>> {
        list_backups_from_dir(&self.backup_dir, &["dump", "sql"])
    }

    async fn delete_backup(&self, name: &str) -> AppResult<()> {
        let path = resolve_backup_file(&self.backup_dir, name)?;
        if path.exists() {
            std::fs::remove_file(path).map_err(AppError::internal)?;
        }
        Ok(())
    }

    async fn restore_backup(&self, name: &str) -> AppResult<()> {
        let path = resolve_backup_file(&self.backup_dir, name)?;
        if !path.exists() {
            return Err(AppError::NotFound(format!("Backup not found: {name}")));
        }

        let ext = path
            .extension()
            .and_then(|v| v.to_str())
            .unwrap_or_default();
        let output = if ext == "dump" {
            Command::new("pg_restore")
                .arg("--dbname")
                .arg(&self.database_url)
                .arg("--clean")
                .arg("--if-exists")
                .arg("--single-transaction")
                .arg("--exit-on-error")
                .arg("--no-owner")
                .arg("--no-privileges")
                .arg(&path)
                .stdin(Stdio::null())
                .output()
                .await
                .map_err(map_command_error("pg_restore"))?
        } else {
            Command::new("psql")
                .arg("--dbname")
                .arg(&self.database_url)
                .arg("-v")
                .arg("ON_ERROR_STOP=1")
                .arg("-1")
                .arg("--file")
                .arg(&path)
                .stdin(Stdio::null())
                .output()
                .await
                .map_err(map_command_error("psql"))?
        };

        if !output.status.success() {
            return Err(AppError::internal(format!(
                "restore failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }
}

fn list_backups_from_dir(backup_dir: &Path, expected_exts: &[&str]) -> AppResult<Vec<BackupEntry>> {
    let mut items = Vec::new();
    if !backup_dir.exists() {
        return Ok(items);
    }

    for entry in std::fs::read_dir(backup_dir).map_err(AppError::internal)? {
        let entry = entry.map_err(AppError::internal)?;
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|v| v.to_str())
            .unwrap_or_default();
        if !expected_exts.contains(&ext) {
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

fn map_command_error(bin: &'static str) -> impl FnOnce(std::io::Error) -> AppError {
    move |err| match err.kind() {
        std::io::ErrorKind::NotFound => {
            AppError::internal(format!("{bin} is not installed or not in PATH"))
        }
        _ => AppError::internal(err),
    }
}
