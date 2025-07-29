use crate::models::Entry;
use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use chrono::{Local, DateTime};
use chrono_english::{parse_date_string, Dialect};
use regex::Regex;
use std::io::{self, Write};

pub fn handle_now(
    entry: Vec<String>,
    note: Option<String>,
    back: Option<String>,
    section: Option<String>,
    finish_last: bool,
    from: Option<String>,
    editor: bool,
    ask: bool,
    _noauto: bool,
) -> color_eyre::Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    
    let mut doing_file = parse_taskpaper(&doing_file_path)?;
    
    // Handle finish_last option - mark last entry as done
    if finish_last {
        let target_section = section.as_deref().unwrap_or("Currently");
        
        // Find the last undone entry in the section
        let last_entry_info = doing_file.get_all_entries()
            .into_iter()
            .filter(|e| e.section == target_section && !e.is_done())
            .max_by_key(|e| e.timestamp)
            .map(|e| (e.timestamp, e.description.clone()));
            
        if let Some((timestamp, description)) = last_entry_info {
            // Now update the actual entry in the sections
            if let Some(entries) = doing_file.sections.get_mut(target_section) {
                for entry in entries.iter_mut() {
                    if entry.timestamp == timestamp && entry.description == description {
                        entry.mark_done();
                        break;
                    }
                }
            }
        }
    }
    
    // Get entry text
    let entry_text = if entry.is_empty() {
        if editor {
            // TODO: Implement editor support
            return Err(color_eyre::eyre::eyre!("Editor support not yet implemented"));
        } else {
            // Interactive prompt
            print!("What are you doing now? ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    } else {
        entry.join(" ")
    };
    
    if entry_text.is_empty() {
        return Err(color_eyre::eyre::eyre!("Entry text cannot be empty"));
    }
    
    // Parse parenthetical at end as note
    let paren_regex = Regex::new(r"^(.+?)\s*\(([^)]+)\)\s*$")?;
    let (final_entry_text, parsed_note) = if let Some(captures) = paren_regex.captures(&entry_text) {
        (captures[1].to_string(), Some(captures[2].to_string()))
    } else {
        (entry_text.clone(), None)
    };
    
    // Get final note (command line flag takes precedence)
    let final_note = if let Some(n) = note {
        Some(n)
    } else if let Some(n) = parsed_note {
        Some(n)
    } else if ask {
        // Multi-line note input
        println!("Add a note:");
        println!("Enter a blank line (return twice) to end editing and save, CTRL-C to cancel");
        let mut lines = Vec::new();
        let stdin = io::stdin();
        let mut empty_line_count = 0;
        
        loop {
            let mut line = String::new();
            stdin.read_line(&mut line)?;
            
            if line.trim().is_empty() {
                empty_line_count += 1;
                if empty_line_count >= 2 {
                    break;
                }
                lines.push(String::new());
            } else {
                empty_line_count = 0;
                lines.push(line.trim_end().to_string());
            }
        }
        
        // Remove trailing empty lines
        while lines.last().map_or(false, |l| l.is_empty()) {
            lines.pop();
        }
        
        if !lines.is_empty() {
            Some(lines.join("\n"))
        } else {
            None
        }
    } else {
        None
    };
    
    // Determine section
    let target_section = section.unwrap_or_else(|| "Currently".to_string());
    
    // Extract tags from entry text first
    let tag_regex = Regex::new(r"@(\w+)(?:\(([^)]+)\))?")?;
    let mut tags = Vec::new();
    for capture in tag_regex.captures_iter(&final_entry_text) {
        let tag_name = capture[1].to_string();
        let tag_value = capture.get(2).map(|m| m.as_str().to_string());
        tags.push((tag_name, tag_value));
    }
    
    // Remove tags from description
    let clean_description = tag_regex.replace_all(&final_entry_text, "").trim().to_string();
    
    // Create entry with clean description
    let mut new_entry = Entry::new(clean_description, target_section);
    
    // Add tags
    for (tag_name, tag_value) in tags {
        new_entry = new_entry.with_tag(tag_name, tag_value);
    }
    
    // Handle backdating
    let entry_time = if let Some(back_str) = back {
        parse_date_string(&back_str, Local::now(), Dialect::Us)
            .map_err(|_| color_eyre::eyre::eyre!("Invalid date string: {}", back_str))?
    } else if let Some(from_str) = from {
        // Parse "from X to Y" format
        parse_from_range(&from_str, &mut new_entry)?
    } else {
        Local::now()
    };
    
    new_entry = new_entry.with_timestamp(entry_time);
    
    // Add note if present
    if let Some(note_text) = final_note {
        new_entry = new_entry.with_note(note_text);
    }
    
    doing_file.add_entry(new_entry.clone());
    save_taskpaper(&doing_file)?;
    
    println!("{}: {}", 
        new_entry.timestamp.format("%Y-%m-%d %H:%M"),
        new_entry.description
    );
    
    if !new_entry.tags.is_empty() {
        let tags_str: Vec<String> = new_entry.tags.iter()
            .map(|(k, v)| {
                if let Some(val) = v {
                    format!("@{}({})", k, val)
                } else {
                    format!("@{}", k)
                }
            })
            .collect();
        println!("  {}", tags_str.join(" "));
    }
    
    if let Some(note) = &new_entry.note {
        println!("  Note: {}", note.lines().next().unwrap_or(""));
        for line in note.lines().skip(1) {
            println!("        {}", line);
        }
    }
    
    Ok(())
}

fn parse_from_range(from_str: &str, entry: &mut Entry) -> Result<DateTime<Local>, color_eyre::eyre::Error> {
    // Parse "from X to Y" or just "from X"
    let from_regex = Regex::new(r"(?i)from\s+(.+?)(?:\s+to\s+(.+))?$")?;
    
    if let Some(captures) = from_regex.captures(from_str) {
        let start_str = &captures[1];
        let start_time = parse_date_string(start_str, Local::now(), Dialect::Us)
            .map_err(|_| color_eyre::eyre::eyre!("Invalid start time: {}", start_str))?;
        
        if let Some(end_match) = captures.get(2) {
            let end_str = end_match.as_str();
            let end_time = parse_date_string(end_str, start_time, Dialect::Us)
                .map_err(|_| color_eyre::eyre::eyre!("Invalid end time: {}", end_str))?;
            
            // Add @done tag with end time
            entry.tags.insert("done".to_string(), Some(end_time.format("%Y-%m-%d %H:%M").to_string()));
        }
        
        Ok(start_time)
    } else {
        Err(color_eyre::eyre::eyre!("Invalid from format. Use: from TIME [to TIME]"))
    }
}