use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub doing_file: PathBuf,
}

impl Config {
    pub fn load() -> Self {
        Self::default()
    }

    pub fn doing_file_path(&self) -> PathBuf {
        if self.doing_file.is_absolute() {
            self.doing_file.clone()
        } else {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(&self.doing_file)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            doing_file: PathBuf::from(".doing.taskpaper"),
        }
    }
}
