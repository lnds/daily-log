use super::{Entry, Section};
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DoingFile {
    pub path: PathBuf,
    pub sections: HashMap<String, Vec<Entry>>,
}

impl DoingFile {
    pub fn new(path: PathBuf) -> Self {
        let mut sections = HashMap::new();
        sections.insert(Section::Currently.as_str().to_string(), Vec::new());
        sections.insert(Section::Later.as_str().to_string(), Vec::new());

        Self { path, sections }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.sections
            .entry(entry.section.clone())
            .or_insert_with(Vec::new)
            .push(entry);
    }

    pub fn get_entries(&self, section: &str) -> Option<&Vec<Entry>> {
        self.sections.get(section)
    }

    pub fn get_all_entries(&self) -> Vec<&Entry> {
        self.sections
            .values()
            .flat_map(|entries| entries.iter())
            .collect()
    }

    pub fn get_recent_entries(&self, count: usize) -> Vec<&Entry> {
        let mut all_entries: Vec<&Entry> = self.get_all_entries();
        all_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        all_entries.into_iter().take(count).collect()
    }

    pub fn get_entries_since(&self, since: DateTime<Local>) -> Vec<&Entry> {
        self.get_all_entries()
            .into_iter()
            .filter(|entry| entry.timestamp >= since)
            .collect()
    }

    pub fn get_last_entry(&self) -> Option<&Entry> {
        self.get_all_entries()
            .into_iter()
            .max_by_key(|entry| entry.timestamp)
    }

    pub fn to_taskpaper(&self) -> String {
        let mut result = String::new();

        let section_order = vec!["Currently", "Later"];

        for section_name in section_order {
            if let Some(entries) = self.sections.get(section_name) {
                if !entries.is_empty() {
                    result.push_str(&Section::from_str(section_name).to_taskpaper());
                    result.push('\n');

                    for entry in entries {
                        result.push_str(&entry.to_taskpaper());
                        result.push('\n');
                    }

                    result.push('\n');
                }
            }
        }

        for (section_name, entries) in &self.sections {
            if !["Currently", "Later"].contains(&section_name.as_str()) && !entries.is_empty() {
                result.push_str(&Section::from_str(section_name).to_taskpaper());
                result.push('\n');

                for entry in entries {
                    result.push_str(&entry.to_taskpaper());
                    result.push('\n');
                }

                result.push('\n');
            }
        }

        result.trim_end().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doing_file_new() {
        let file = DoingFile::new(PathBuf::from("test.taskpaper"));
        assert_eq!(file.path, PathBuf::from("test.taskpaper"));
        assert!(file.sections.contains_key("Currently"));
        assert!(file.sections.contains_key("Later"));
    }

    #[test]
    fn test_add_and_get_entries() {
        let mut file = DoingFile::new(PathBuf::from("test.taskpaper"));
        let entry1 = Entry::new("Task 1".to_string(), "Currently".to_string());
        let entry2 = Entry::new("Task 2".to_string(), "Later".to_string());

        file.add_entry(entry1);
        file.add_entry(entry2);

        assert_eq!(file.get_entries("Currently").unwrap().len(), 1);
        assert_eq!(file.get_entries("Later").unwrap().len(), 1);
        assert_eq!(file.get_all_entries().len(), 2);
    }

    #[test]
    fn test_taskpaper_output() {
        let mut file = DoingFile::new(PathBuf::from("test.taskpaper"));
        file.add_entry(Entry::new(
            "Current task".to_string(),
            "Currently".to_string(),
        ));
        file.add_entry(Entry::new("Future task".to_string(), "Later".to_string()));

        let output = file.to_taskpaper();
        assert!(output.contains("Currently:"));
        assert!(output.contains("- Current task"));
        assert!(output.contains("Later:"));
        assert!(output.contains("- Future task"));
    }
}
