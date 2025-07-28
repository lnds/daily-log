use crate::models::{DoingFile, Entry};
use chrono::{DateTime, Local, NaiveDateTime};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
    let task_regex = Regex::new(r"^- (.+)$")?;
    let tag_regex = Regex::new(r"@(\w+)(?:\(([^)]+)\))?")?;

    for line in content.lines() {
        if let Some(captures) = project_regex.captures(line) {
            if let Some(entry) = current_entry.take() {
                doing_file.add_entry(entry);
            }
            current_section = captures[1].to_string();
        } else if let Some(captures) = task_regex.captures(line) {
            if let Some(entry) = current_entry.take() {
                doing_file.add_entry(entry);
            }

            let task_line = &captures[1];
            let mut description = task_line.to_string();
            let mut tags = HashMap::new();

            for tag_capture in tag_regex.captures_iter(task_line) {
                let tag_name = tag_capture[1].to_string();
                let tag_value = tag_capture.get(2).map(|m| m.as_str().to_string());
                tags.insert(tag_name, tag_value);

                description = description.replace(&tag_capture[0], "").trim().to_string();
            }

            let mut entry = Entry::new(description, current_section.clone());
            entry.tags = tags;

            if let Some(Some(done_time)) = entry.tags.get("done") {
                if let Ok(timestamp) = parse_done_timestamp(done_time) {
                    entry = entry.with_timestamp(timestamp);
                }
            }

            current_entry = Some(entry);
        } else if line.starts_with("    ") && current_entry.is_some() {
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

fn parse_done_timestamp(timestamp: &str) -> Result<DateTime<Local>, chrono::ParseError> {
    let formats = [
        "%Y-%m-%d %H:%M",
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M",
        "%Y/%m/%d %H:%M:%S",
    ];

    for format in &formats {
        if let Ok(naive) = NaiveDateTime::parse_from_str(timestamp, format) {
            return Ok(DateTime::from_naive_utc_and_offset(
                naive,
                *Local::now().offset(),
            ));
        }
    }

    let naive = NaiveDateTime::parse_from_str("invalid", "%Y-%m-%d");
    Err(naive.unwrap_err())
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
        assert_eq!(result.sections.len(), 2);
    }

    #[test]
    fn test_parse_taskpaper_content() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Currently:").unwrap();
        writeln!(temp_file, "- Working on parser @in_progress").unwrap();
        writeln!(temp_file, "    This is a note").unwrap();
        writeln!(temp_file, "- Completed task @done(2024-01-15 14:30)").unwrap();
        writeln!(temp_file, "").unwrap();
        writeln!(temp_file, "Later:").unwrap();
        writeln!(temp_file, "- Future task @priority(high)").unwrap();

        let result = parse_taskpaper(temp_file.path()).unwrap();

        let currently = result.get_entries("Currently").unwrap();
        assert_eq!(currently.len(), 2);
        assert_eq!(currently[0].description, "Working on parser");
        assert!(currently[0].tags.contains_key("in_progress"));
        assert_eq!(currently[0].note, Some("This is a note".to_string()));

        let later = result.get_entries("Later").unwrap();
        assert_eq!(later.len(), 1);
        assert_eq!(
            later[0].tags.get("priority"),
            Some(&Some("high".to_string()))
        );
    }
}
