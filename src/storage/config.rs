use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub doing_file: PathBuf,
}

impl Config {
    pub fn load() -> Self {
        // Check for test config using thread ID
        let test_var = format!("DOING_TEST_CONFIG_{:?}", std::thread::current().id());
        if let Ok(config_path) = std::env::var(&test_var)
            && let Ok(content) = std::fs::read_to_string(&config_path)
                && let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }

        Self::default()
    }

    pub fn from_path(path: &std::path::Path) -> color_eyre::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
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
