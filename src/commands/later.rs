use crate::models::Entry;
use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use regex::Regex;

pub fn handle_later(
    task: Vec<String>,
    tags: Vec<String>,
    note: Option<String>,
) -> color_eyre::Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();

    let mut doing_file = parse_taskpaper(&doing_file_path)?;

    let description = task.join(" ");
    if description.is_empty() {
        return Err(color_eyre::eyre::eyre!("Task description cannot be empty"));
    }

    let mut entry = Entry::new(description, "Later".to_string());

    let tag_regex = Regex::new(r"^@?(\w+)(?:\(([^)]+)\))?$")?;
    for tag in tags {
        if let Some(captures) = tag_regex.captures(&tag) {
            let tag_name = captures[1].to_string();
            let tag_value = captures.get(2).map(|m| m.as_str().to_string());
            entry = entry.with_tag(tag_name, tag_value);
        }
    }

    if let Some(note_text) = note {
        entry = entry.with_note(note_text);
    }

    doing_file.add_entry(entry.clone());
    save_taskpaper(&doing_file)?;

    println!("Added to Later: {}", entry.description);

    Ok(())
}
