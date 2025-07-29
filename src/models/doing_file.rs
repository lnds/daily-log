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

        Self { path, sections }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.sections
            .entry(entry.section.clone())
            .or_insert_with(Vec::new)
            .push(entry);
    }

    pub fn add_entry_to_section(&mut self, mut entry: Entry, section: String) {
        entry.section = section.clone();
        self.sections
            .entry(section)
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
        
        // Sort sections to ensure consistent output
        let mut sorted_sections: Vec<(&String, &Vec<Entry>)> = self.sections.iter().collect();
        sorted_sections.sort_by_key(|(name, _)| {
            // Currently always comes first, then alphabetical
            if *name == "Currently" {
                ("", name.as_str())
            } else {
                ("z", name.as_str())
            }
        });

        for (section_name, entries) in sorted_sections {
            result.push_str(&Section::from_str(section_name).to_taskpaper());
            result.push('\n');

            for entry in entries {
                result.push_str(&entry.to_taskpaper());
                result.push('\n');
            }

            // Add blank line between sections
            if !result.ends_with("\n\n") {
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
    }

    #[test]
    fn test_add_and_get_entries() {
        let mut file = DoingFile::new(PathBuf::from("test.taskpaper"));
        let entry1 = Entry::new("Task 1".to_string(), "Currently".to_string());
        let entry2 = Entry::new("Task 2".to_string(), "Archive".to_string());

        file.add_entry(entry1);
        file.add_entry(entry2);

        assert_eq!(file.get_entries("Currently").unwrap().len(), 1);
        assert_eq!(file.get_entries("Archive").unwrap().len(), 1);
        assert_eq!(file.get_all_entries().len(), 2);
    }

    #[test]
    fn test_taskpaper_output() {
        let mut file = DoingFile::new(PathBuf::from("test.taskpaper"));
        file.add_entry(Entry::new(
            "Current task".to_string(),
            "Currently".to_string(),
        ));
        file.add_entry(Entry::new("Archived task".to_string(), "Archive".to_string()));

        let output = file.to_taskpaper();
        assert!(output.contains("Currently:"));
        // Updated format includes timestamp and UUID
        assert!(output.contains(" | Current task <"));
        assert!(output.contains("Archive:"));
        assert!(output.contains(" | Archived task <"));
    }
}
