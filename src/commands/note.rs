use crate::models::Entry;
use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use regex::Regex;
use std::io;

#[derive(Debug)]
pub struct NoteFilterOptions {
    pub sections: Vec<String>,
    pub search: Option<String>,
    pub tag: Option<String>,
    pub case: String,
    pub exact: bool,
    pub not: bool,
}

#[derive(Debug)]
pub struct NoteOptions {
    pub note: Vec<String>,
    pub ask: bool,
    pub editor: bool,
    pub remove: bool,
}

pub fn handle_note(
    filter_opts: NoteFilterOptions,
    note_opts: NoteOptions,
    interactive: bool,
) -> color_eyre::Result<()> {
    if interactive {
        return Err(color_eyre::eyre::eyre!("Interactive mode not yet implemented"));
    }

    if note_opts.editor {
        return Err(color_eyre::eyre::eyre!("Editor mode not yet implemented"));
    }

    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    
    let mut doing_file = parse_taskpaper(&doing_file_path)?;
    
    // Find the entry to modify
    let entry_to_modify = find_entry_to_modify(
        &doing_file,
        &filter_opts.sections,
        filter_opts.search.as_deref(),
        filter_opts.tag.as_deref(),
        filter_opts.exact,
        filter_opts.not,
        &filter_opts.case,
    )?;

    // Get the note text
    let note_text = if !note_opts.note.is_empty() {
        Some(note_opts.note.join(" "))
    } else if note_opts.ask {
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
        while lines.last().is_some_and(|l| l.is_empty()) {
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

    // Find and modify the entry
    let mut modified = false;
    let target_uuid = entry_to_modify.uuid;
    let target_section = entry_to_modify.section.clone();
    
    if let Some(entries) = doing_file.sections.get_mut(&target_section) {
        for entry in entries.iter_mut() {
            if entry.uuid == target_uuid {
                if note_opts.remove && note_text.is_none() {
                    // Remove note
                    entry.note = None;
                    println!("Note removed from: {}", entry.description);
                } else if note_opts.remove && note_text.is_some() {
                    // Replace note
                    entry.note = note_text.clone();
                    println!("Note replaced for: {}", entry.description);
                    if let Some(ref new_note) = entry.note {
                        for line in new_note.lines() {
                            println!("  {line}");
                        }
                    }
                } else if let Some(ref new_note) = note_text {
                    // Append to existing note or set new note
                    if let Some(ref existing_note) = entry.note {
                        entry.note = Some(format!("{existing_note}\n{new_note}"));
                    } else {
                        entry.note = Some(new_note.clone());
                    }
                    println!("Note added to: {}", entry.description);
                    if let Some(ref note) = entry.note {
                        for line in note.lines() {
                            println!("  {line}");
                        }
                    }
                }
                modified = true;
                break;
            }
        }
    }
    
    if !modified {
        return Err(color_eyre::eyre::eyre!("Entry not found"));
    }
    
    save_taskpaper(&doing_file)?;
    Ok(())
}

fn find_entry_to_modify(
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
        .ok_or_else(|| color_eyre::eyre::eyre!("No matching entry found"))
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
            Regex::new(&format!("(?i){pattern}"))?
        };
        entries.into_iter()
            .filter(|entry| regex.is_match(&entry.description))
            .collect()
    } else if let Some(query) = search_query.strip_prefix('\'') {
        // Exact match
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
                    if let Ok(regex) = Regex::new(&format!("^{pattern}$")) {
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