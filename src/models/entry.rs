use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    pub description: String,
    pub timestamp: DateTime<Local>,
    pub section: String,
    pub tags: HashMap<String, Option<String>>,
    pub note: Option<String>,
    pub uuid: Uuid,
}

impl Entry {
    pub fn new(description: String, section: String) -> Self {
        Self {
            description,
            timestamp: Local::now(),
            section,
            tags: HashMap::new(),
            note: None,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn with_timestamp(mut self, timestamp: DateTime<Local>) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn with_tag(mut self, key: String, value: Option<String>) -> Self {
        self.tags.insert(key, value);
        self
    }

    pub fn with_note(mut self, note: String) -> Self {
        self.note = Some(note);
        self
    }

    pub fn is_done(&self) -> bool {
        self.tags.contains_key("done")
    }

    pub fn mark_done(&mut self) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M").to_string();
        self.tags.insert("done".to_string(), Some(timestamp));
    }

    pub fn to_taskpaper(&self) -> String {
        // Build description with inline tags
        let mut desc_with_tags = self.description.clone();

        // Add tags to the description
        for (tag, value) in &self.tags {
            desc_with_tags.push_str(&format!(" @{tag}"));
            if let Some(v) = value {
                desc_with_tags.push_str(&format!("({v})"));
            }
        }

        // Format: - YYYY-MM-DD HH:MM | description @tags <uuid>
        let mut result = format!(
            " - {} | {} <{}>",
            self.timestamp.format("%Y-%m-%d %H:%M"),
            desc_with_tags,
            self.uuid.as_hyphenated()
        );

        // Add note with proper indentation (2 spaces)
        if let Some(note) = &self.note {
            for line in note.lines() {
                result.push_str(&format!("\n  {line}"));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_entry() {
        let entry = Entry::new("Test task".to_string(), "Currently".to_string());
        assert_eq!(entry.description, "Test task");
        assert_eq!(entry.section, "Currently");
        assert!(entry.tags.is_empty());
        assert!(entry.note.is_none());
    }

    #[test]
    fn test_taskpaper_format() {
        let mut entry = Entry::new("Write tests".to_string(), "Currently".to_string())
            .with_tag("priority".to_string(), Some("high".to_string()))
            .with_note("This is important".to_string());

        let output = entry.to_taskpaper();
        // Check format: - YYYY-MM-DD HH:MM | description @tags <uuid>
        assert!(output.starts_with(" - "));
        assert!(output.contains(" | Write tests @priority(high) <"));
        assert!(output.contains("\n  This is important"));

        entry.mark_done();
        let output = entry.to_taskpaper();
        assert!(output.contains("@done("));
    }
}
