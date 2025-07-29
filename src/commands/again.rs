use crate::models::Entry;
use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use chrono::Local;
use chrono_english::{parse_date_string, Dialect};
use regex::Regex;
use std::io;

pub fn handle_again(
    noauto: bool,
    ask: bool,
    back: Option<String>,
    _bool_op: String,
    case: String,
    editor: bool,
    interactive: bool,
    in_section: Option<String>,
    note: Option<String>,
    not: bool,
    sections: Vec<String>,
    search: Option<String>,
    tag: Option<String>,
    _val: Vec<String>,
    exact: bool,
) -> color_eyre::Result<()> {
    if interactive {
        return Err(color_eyre::eyre::eyre!("Interactive mode not yet implemented"));
    }

    if editor {
        return Err(color_eyre::eyre::eyre!("Editor mode not yet implemented"));
    }

    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    
    let mut doing_file = parse_taskpaper(&doing_file_path)?;
    
    // Find the entry to duplicate
    let entry_to_duplicate = find_entry_to_duplicate(
        &doing_file,
        &sections,
        search.as_deref(),
        tag.as_deref(),
        exact,
        not,
        &case,
    )?;

    // Create new entry based on the found one
    let new_start_time = if let Some(back_str) = &back {
        parse_date_string(back_str, Local::now(), Dialect::Us)
            .map_err(|_| color_eyre::eyre::eyre!("Invalid date string: {}", back_str))?
    } else {
        Local::now()
    };

    // Create new entry with same description and tags (minus @done)
    let mut new_entry = Entry::new(
        entry_to_duplicate.description.clone(),
        in_section.clone().unwrap_or_else(|| entry_to_duplicate.section.clone()),
    );
    
    // Set the new timestamp
    new_entry.timestamp = new_start_time;
    
    // Copy tags except @done
    for (tag_name, tag_value) in &entry_to_duplicate.tags {
        if tag_name != "done" {
            new_entry.tags.insert(tag_name.clone(), tag_value.clone());
        }
    }

    // Add or update note
    if let Some(new_note) = note {
        new_entry.note = Some(new_note);
    } else if ask {
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
            new_entry.note = Some(lines.join("\n"));
        }
    } else if let Some(existing_note) = &entry_to_duplicate.note {
        new_entry.note = Some(existing_note.clone());
    }

    // Add auto tags unless disabled
    if !noauto {
        // Add any default tags from config if implemented
    }

    // Add the new entry
    doing_file.add_entry(new_entry.clone());
    save_taskpaper(&doing_file)?;

    // Show confirmation
    println!(
        "{}: {}",
        new_entry.timestamp.format("%Y-%m-%d %H:%M"),
        new_entry.description
    );
    
    if let Some(note_text) = &new_entry.note {
        for line in note_text.lines() {
            println!("  {}", line);
        }
    }

    Ok(())
}

fn find_entry_to_duplicate(
    doing_file: &crate::models::DoingFile,
    sections: &[String],
    search: Option<&str>,
    tag: Option<&str>,
    exact: bool,
    not: bool,
    case: &str,
) -> Result<Entry, color_eyre::eyre::Error> {
    // Determine which sections to search
    let target_sections: Vec<String> = if sections.is_empty() {
        // Get all section names
        doing_file.sections.keys().cloned().collect()
    } else {
        sections.to_vec()
    };
    
    // Collect all entries from target sections
    let mut all_entries: Vec<Entry> = Vec::new();
    for section in &target_sections {
        if let Some(entries) = doing_file.sections.get(section) {
            for entry in entries {
                all_entries.push(entry.clone());
            }
        }
    }
    
    // Sort by timestamp (newest first)
    all_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    // Apply filters
    let mut filtered_entries = all_entries;
    
    // Apply search filter
    if let Some(search_query) = search {
        filtered_entries = filter_by_search(filtered_entries, search_query, exact, case)?;
    }
    
    // Apply tag filter
    if let Some(tag_query) = tag {
        filtered_entries = filter_by_tag(filtered_entries, tag_query)?;
    }
    
    // Apply NOT filter if specified
    if not {
        // Get all entries again
        let mut all_entries_again: Vec<Entry> = Vec::new();
        for section in &target_sections {
            if let Some(entries) = doing_file.sections.get(section) {
                for entry in entries {
                    all_entries_again.push(entry.clone());
                }
            }
        }
        all_entries_again.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        filtered_entries = all_entries_again.into_iter()
            .filter(|entry| !filtered_entries.iter().any(|e| e.uuid == entry.uuid))
            .collect();
    }
    
    // Get the most recent entry after filtering
    filtered_entries
        .into_iter()
        .next()
        .ok_or_else(|| color_eyre::eyre::eyre!("No matching entry found to duplicate"))
}

fn filter_by_search(
    entries: Vec<Entry>,
    search_query: &str,
    exact: bool,
    case: &str,
) -> Result<Vec<Entry>, color_eyre::eyre::Error> {
    // Determine case sensitivity
    let case_sensitive = match case {
        "c" | "case-sensitive" => true,
        "i" | "ignore" => false,
        "s" | "smart" => {
            // Smart case: case-sensitive if search contains uppercase
            search_query.chars().any(|c| c.is_uppercase())
        }
        _ => false, // Default to case-insensitive
    };

    let filtered = if search_query.starts_with('/') && search_query.ends_with('/') {
        // Regex search
        let pattern = &search_query[1..search_query.len()-1];
        let regex = if case_sensitive {
            Regex::new(pattern)?
        } else {
            Regex::new(&format!("(?i){}", pattern))?
        };
        entries.into_iter()
            .filter(|entry| regex.is_match(&entry.description))
            .collect()
    } else if search_query.starts_with('\'') {
        // Exact match
        let query = &search_query[1..];
        entries.into_iter()
            .filter(|entry| {
                if case_sensitive {
                    entry.description == query
                } else {
                    entry.description.eq_ignore_ascii_case(query)
                }
            })
            .collect()
    } else if exact {
        // Exact substring match
        entries.into_iter()
            .filter(|entry| {
                if case_sensitive {
                    entry.description.contains(search_query)
                } else {
                    entry.description.to_lowercase().contains(&search_query.to_lowercase())
                }
            })
            .collect()
    } else {
        // Regular substring matching
        entries.into_iter()
            .filter(|entry| {
                if case_sensitive {
                    entry.description.contains(search_query)
                } else {
                    entry.description.to_lowercase().contains(&search_query.to_lowercase())
                }
            })
            .collect()
    };
    
    Ok(filtered)
}

fn filter_by_tag(
    entries: Vec<Entry>,
    tag_query: &str,
) -> Result<Vec<Entry>, color_eyre::eyre::Error> {
    let tags: Vec<&str> = tag_query.split(',').map(|s| s.trim()).collect();
    
    let filtered = entries.into_iter()
        .filter(|entry| {
            // Check if entry has any of the requested tags
            for tag in &tags {
                if tag.contains('*') || tag.contains('?') {
                    // Wildcard matching
                    let pattern = tag.replace('*', ".*").replace('?', ".");
                    if let Ok(regex) = Regex::new(&format!("^{}$", pattern)) {
                        for entry_tag in entry.tags.keys() {
                            if regex.is_match(entry_tag) {
                                return true;
                            }
                        }
                    }
                } else if entry.tags.contains_key(*tag) {
                    return true;
                }
            }
            false
        })
        .collect();
    
    Ok(filtered)
}