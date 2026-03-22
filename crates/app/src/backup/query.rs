use std::sync::Arc;

use crate::app_error::AppResult;
use crate::backup::{BackupEntry, DatabaseBackupPort};

#[derive(Clone)]
pub struct BackupQueryHandler {
    backup: Arc<dyn DatabaseBackupPort>,
}

impl BackupQueryHandler {
    pub fn new(backup: Arc<dyn DatabaseBackupPort>) -> Self {
        Self { backup }
    }

    pub async fn list_backups(&self) -> AppResult<Vec<BackupEntry>> {
        let mut backups = self.backup.list_backups().await?;
        backups.sort_by(|a, b| b.name.cmp(&a.name));
        Ok(backups)
    }
}
