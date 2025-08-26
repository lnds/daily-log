use crate::models::{DoingFile, Entry};
use chrono::{Local, TimeZone};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn parse_taskpaper(path: &Path) -> color_eyre::Result<DoingFile> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(DoingFile::new(path.to_path_buf()));
        }
        Err(e) => return Err(e.into()),
    };

    let mut doing_file = DoingFile::new(path.to_path_buf());
    let mut current_section = "Currently".to_string();
    let mut current_entry: Option<Entry> = None;

    let project_regex = Regex::new(r"^(.+):$")?;
    // Updated regex to match: - YYYY-MM-DD HH:MM | description <uuid>
    let task_regex =
        Regex::new(r"^ - (\d{4}-\d{2}-\d{2} \d{2}:\d{2}) \| (.+?) <([a-f0-9-]{36})>$")?;
    let tag_regex = Regex::new(r"@(\w+)(?:\(([^)]+)\))?")?;

    for line in content.lines() {
        if let Some(captures) = project_regex.captures(line) {
            if let Some(entry) = current_entry.take() {
                doing_file.add_entry(entry);
            }
            current_section = captures[1].to_string();
            // Ensure the section exists even if it's empty
            doing_file
                .sections
                .entry(current_section.clone())
                .or_default();
        } else if let Some(captures) = task_regex.captures(line) {
            if let Some(entry) = current_entry.take() {
                doing_file.add_entry(entry);
            }

            let timestamp_str = &captures[1];
            let task_line = &captures[2];
            let uuid_str = &captures[3];

            // Parse timestamp as local time
            let timestamp = chrono::NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M")
                .ok()
                .and_then(|naive| Local.from_local_datetime(&naive).single())
                .unwrap_or_else(Local::now);

            // Parse UUID
            let uuid = Uuid::parse_str(uuid_str).unwrap_or_else(|_| Uuid::new_v4());

            let mut description = task_line.to_string();
            let mut tags = HashMap::new();

            // Extract tags from the description
            for tag_capture in tag_regex.captures_iter(task_line) {
                let tag_name = tag_capture[1].to_string();
                let tag_value = tag_capture.get(2).map(|m| m.as_str().to_string());
                tags.insert(tag_name, tag_value);

                description = description.replace(&tag_capture[0], "").trim().to_string();
            }

            let mut entry = Entry::new(description, current_section.clone());
            entry.tags = tags;
            entry.timestamp = timestamp;
            entry.uuid = uuid;

            current_entry = Some(entry);
        } else if line.starts_with("  ") && !line.starts_with(" -") && current_entry.is_some() {
            if let Some(ref mut entry) = current_entry {
                let note_line = line.trim_start();
                if let Some(existing_note) = &mut entry.note {
                    existing_note.push('\n');
                    existing_note.push_str(note_line);
                } else {
                    entry.note = Some(note_line.to_string());
                }
            }
        } else if line.trim().is_empty() {
            if let Some(entry) = current_entry.take() {
                doing_file.add_entry(entry);
            }
        }
    }

    if let Some(entry) = current_entry {
        doing_file.add_entry(entry);
    }

    Ok(doing_file)
}

pub fn save_taskpaper(doing_file: &DoingFile) -> color_eyre::Result<()> {
    let content = doing_file.to_taskpaper();
    fs::write(&doing_file.path, content)?;
    Ok(())
}

pub fn format_taskpaper(doing_file: &DoingFile) -> String {
    doing_file.to_taskpaper()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_empty_file() {
        let path = Path::new("nonexistent.taskpaper");
        let result = parse_taskpaper(path).unwrap();
        assert_eq!(result.sections.len(), 1);
    }

    #[test]
    fn test_parse_taskpaper_content() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Currently:").unwrap();
        writeln!(temp_file, " - 2025-07-28 16:24 | Working on parser @in_progress <7a1185c6-0241-52ac-0771-83f31c40acdd>").unwrap();
        writeln!(temp_file, "  This is a note").unwrap();
        writeln!(temp_file, " - 2025-07-28 12:28 | Completed task @done(2025-07-28 13:58) <e520e775-3401-241c-b8d2-ef3ad6ba3fa7>").unwrap();
        writeln!(temp_file, "").unwrap();
        writeln!(temp_file, "Archive:").unwrap();
        writeln!(temp_file, " - 2025-07-28 09:37 | Archived task @priority(high) <ff20b760-b698-fe0b-d7a8-00213cb6cc3f>").unwrap();

        let result = parse_taskpaper(temp_file.path()).unwrap();

        let currently = result.get_entries("Currently").unwrap();
        assert_eq!(currently.len(), 2);
        assert_eq!(currently[0].description, "Working on parser");
        assert!(currently[0].tags.contains_key("in_progress"));
        assert_eq!(currently[0].note, Some("This is a note".to_string()));

        let archive = result.get_entries("Archive").unwrap();
        assert_eq!(archive.len(), 1);
        assert_eq!(archive[0].description, "Archived task");
        assert_eq!(
            archive[0].tags.get("priority"),
            Some(&Some("high".to_string()))
        );
    }
}
