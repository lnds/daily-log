#[cfg(test)]
pub mod utils {
    use crate::storage::Config;
    use chrono::{DateTime, Duration, Local};
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use uuid::Uuid;

    pub struct TestContext {
        pub temp_dir: TempDir,
        pub doing_file_path: PathBuf,
        pub config_path: PathBuf,
        env_var_name: String,
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            // Clean up environment variable
            unsafe {
                env::remove_var(&self.env_var_name);
            }
        }
    }

    impl TestContext {
        pub fn new() -> color_eyre::Result<Self> {
            let temp_dir = TempDir::new()?;
            let doing_file_path = temp_dir.path().join("test_doing.taskpaper");
            let config_path = temp_dir.path().join(".doingrc");

            // Use thread ID for environment variable name
            let env_var_name = format!("DOING_TEST_CONFIG_{:?}", std::thread::current().id());

            // Create test config
            let config = Config {
                doing_file: doing_file_path.clone(),
            };

            let config_content = serde_json::to_string_pretty(&config)?;
            fs::write(&config_path, config_content)?;

            // Set environment variable to use test config
            unsafe {
                env::set_var(&env_var_name, &config_path);
            }

            Ok(Self {
                temp_dir,
                doing_file_path,
                config_path,
                env_var_name,
            })
        }

        pub fn create_test_file(&self, content: &str) -> color_eyre::Result<()> {
            fs::write(&self.doing_file_path, content)?;
            Ok(())
        }

        pub fn read_test_file(&self) -> color_eyre::Result<String> {
            Ok(fs::read_to_string(&self.doing_file_path)?)
        }

        pub fn create_sample_doing_file(&self) -> color_eyre::Result<()> {
            let now = Local::now();
            let yesterday = now - Duration::days(1);
            let two_days_ago = now - Duration::days(2);

            let content = format!(
                r#"Currently:
- {} | First task @tag1 <{}>
- {} | Second task @done({}) <{}>
  This is a note
- {} | Third task @tag2 @flagged <{}>

Archive:
- {} | Archived task @done({}) <{}>
"#,
                format_date(now),
                Uuid::new_v4(),
                format_date(yesterday),
                format_date(yesterday + Duration::hours(2)),
                Uuid::new_v4(),
                format_date(two_days_ago),
                Uuid::new_v4(),
                format_date(two_days_ago),
                format_date(two_days_ago + Duration::hours(1)),
                Uuid::new_v4(),
            );

            self.create_test_file(&content)
        }

        pub fn create_doing_file_with_entries(
            &self,
            entries: Vec<TestEntry>,
        ) -> color_eyre::Result<()> {
            let mut sections: HashMap<String, Vec<String>> = HashMap::new();

            for entry in entries {
                let section = entry.section.unwrap_or("Currently".to_string());
                let mut line = format!(
                    " - {} | {}",
                    format_date(entry.timestamp),
                    entry.description
                );

                for tag in entry.tags {
                    line.push_str(&format!(" @{tag}"));
                }

                if let Some(done_time) = entry.done_time {
                    line.push_str(&format!(" @done({})", format_date(done_time)));
                }

                line.push_str(&format!(" <{}>", entry.uuid.unwrap_or_else(Uuid::new_v4)));

                sections.entry(section.clone()).or_default().push(line);

                if let Some(note) = entry.note {
                    for note_line in note.lines() {
                        sections
                            .get_mut(&section)
                            .unwrap()
                            .push(format!("  {note_line}"));
                    }
                }
            }

            let mut content = String::new();
            for (section, entries) in sections {
                content.push_str(&format!("{section}:\n"));
                for entry in entries {
                    content.push_str(&format!("{entry}\n"));
                }
                content.push('\n');
            }

            self.create_test_file(&content)
        }
    }

    pub struct TestEntry {
        pub timestamp: DateTime<Local>,
        pub description: String,
        pub tags: Vec<String>,
        pub note: Option<String>,
        pub section: Option<String>,
        pub done_time: Option<DateTime<Local>>,
        pub uuid: Option<Uuid>,
    }

    impl TestEntry {
        pub fn new(description: &str) -> Self {
            Self {
                timestamp: Local::now(),
                description: description.to_string(),
                tags: vec![],
                note: None,
                section: None,
                done_time: None,
                uuid: None,
            }
        }

        pub fn with_timestamp(mut self, timestamp: DateTime<Local>) -> Self {
            self.timestamp = timestamp;
            self
        }

        pub fn with_tags(mut self, tags: Vec<&str>) -> Self {
            self.tags = tags.iter().map(|t| t.to_string()).collect();
            self
        }

        pub fn with_note(mut self, note: &str) -> Self {
            self.note = Some(note.to_string());
            self
        }

        pub fn with_section(mut self, section: &str) -> Self {
            self.section = Some(section.to_string());
            self
        }

        pub fn with_done(mut self, done_time: DateTime<Local>) -> Self {
            self.done_time = Some(done_time);
            self
        }

        pub fn with_uuid(mut self, uuid: Uuid) -> Self {
            self.uuid = Some(uuid);
            self
        }
    }

    fn format_date(date: DateTime<Local>) -> String {
        date.format("%Y-%m-%d %H:%M").to_string()
    }

    // Helper to capture stdout
    pub fn capture_stdout<F>(f: F) -> String
    where
        F: FnOnce(),
    {
        // We can't actually redirect stdout in tests easily, so we'll need to refactor
        // commands to accept a writer parameter. For now, let's return empty string
        // and note this limitation.
        f();

        String::new() // This is a limitation - we need to refactor commands to accept writers
    }
}
