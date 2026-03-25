use serde::{Deserialize, Serialize};

use crate::types::auth::MeDto;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Preference {
    pub locale: String,
    pub theme: Option<String>,
    pub grid_theme: String,
    pub privacy: String,
    pub user: Option<MeDto>,
}

impl Default for Preference {
    fn default() -> Self {
        Self {
            locale: "en-US".to_string(),
            theme: None,
            grid_theme: "green".to_string(),
            privacy: "public".to_string(),
            user: None,
        }
    }
}
