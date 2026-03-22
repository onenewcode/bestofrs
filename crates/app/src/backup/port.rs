use crate::app_error::AppResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupEntry {
    pub name: String,
    pub created_at_utc: String,
    pub size_bytes: u64,
}

#[async_trait]
pub trait DatabaseBackupPort: Send + Sync {
    async fn create_backup(&self, label: &str) -> AppResult<BackupEntry>;
    async fn list_backups(&self) -> AppResult<Vec<BackupEntry>>;
    async fn delete_backup(&self, name: &str) -> AppResult<()>;
    async fn restore_backup(&self, name: &str) -> AppResult<()>;
}
