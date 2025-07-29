use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use chrono::{Local, DateTime};
use regex::Regex;
use std::io::{self, Write};

#[derive(Debug)]
pub struct DeleteOptions {
    pub count: usize,
    pub interactive: bool,
    pub not: bool,
    pub sections: Vec<String>,
    pub search: Option<String>,
    pub tag: Option<String>,
    pub exact: bool,
    pub force: bool,
}

pub fn handle_delete(opts: DeleteOptions) -> color_eyre::Result<()> {
    if opts.interactive {
        return Err(color_eyre::eyre::eyre!("Interactive mode not yet implemented"));
    }

    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    
    let mut doing_file = parse_taskpaper(&doing_file_path)?;
    
    // Determine which sections to work with
    let target_sections: Vec<String> = if opts.sections.is_empty() {
        vec!["Currently".to_string()]
    } else {
        opts.sections
    };
    
    // Collect all entries from target sections
    let mut all_entries: Vec<(String, DateTime<Local>, String)> = Vec::new();
    for section in &target_sections {
        if let Some(entries) = doing_file.sections.get(section) {
            for entry in entries {
                all_entries.push((section.clone(), entry.timestamp, entry.description.clone()));
            }
        }
    }
    
    // Sort by timestamp (newest first)
    all_entries.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Filter entries based on search criteria
    let mut filtered_entries = all_entries;
    
    // Apply search filter
    if let Some(search_query) = &opts.search {
        filtered_entries = filter_by_search(filtered_entries, search_query, opts.exact)?;
    }
    
    // Apply tag filter
    if let Some(tag_query) = &opts.tag {
        filtered_entries = filter_by_tag(&doing_file, filtered_entries, tag_query)?;
    }
    
    // Apply NOT filter if specified
    if opts.not {
        // Get all entries again and remove the filtered ones
        let mut all_entries_again: Vec<(String, DateTime<Local>, String)> = Vec::new();
        for section in &target_sections {
            if let Some(entries) = doing_file.sections.get(section) {
                for entry in entries {
                    all_entries_again.push((section.clone(), entry.timestamp, entry.description.clone()));
                }
            }
        }
        all_entries_again.sort_by(|a, b| b.1.cmp(&a.1));
        
        filtered_entries = all_entries_again.into_iter()
            .filter(|entry| !filtered_entries.contains(entry))
            .collect();
    }
    
    // Take only the requested count
    let entries_to_delete: Vec<_> = filtered_entries.into_iter().take(opts.count).collect();
    
    if entries_to_delete.is_empty() {
        return Err(color_eyre::eyre::eyre!("No matching entries found to delete"));
    }
    
    // Confirm deletion if not forced
    if !opts.force {
        println!("The following entries will be deleted:");
        for (section, timestamp, description) in &entries_to_delete {
            println!("  {} | {} [{}]", timestamp.format("%Y-%m-%d %H:%M"), description, section);
        }
        print!("\nAre you sure you want to delete {} {}? [y/N] ", 
               entries_to_delete.len(),
               if entries_to_delete.len() == 1 { "entry" } else { "entries" });
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Deletion cancelled.");
            return Ok(());
        }
    }
    
    // Delete entries
    let mut deleted_count = 0;
    for (section, timestamp, description) in entries_to_delete {
        if let Some(entries) = doing_file.sections.get_mut(&section) {
            let initial_len = entries.len();
            entries.retain(|entry| !(entry.timestamp == timestamp && entry.description == description));
            
            if entries.len() < initial_len {
                deleted_count += 1;
                println!("Deleted: {} | {}", timestamp.format("%Y-%m-%d %H:%M"), description);
            }
        }
    }
    
    save_taskpaper(&doing_file)?;
    
    if deleted_count == 0 {
        return Err(color_eyre::eyre::eyre!("No entries were deleted"));
    }
    
    println!("\nDeleted {} {}.", deleted_count, if deleted_count == 1 { "entry" } else { "entries" });
    
    Ok(())
}

fn filter_by_search(
    entries: Vec<(String, DateTime<Local>, String)>,
    search_query: &str,
    exact: bool,
) -> Result<Vec<(String, DateTime<Local>, String)>, color_eyre::eyre::Error> {
    let filtered = if search_query.starts_with('/') && search_query.ends_with('/') {
        // Regex search
        let pattern = &search_query[1..search_query.len()-1];
        let regex = Regex::new(pattern)?;
        entries.into_iter()
            .filter(|(_, _, desc)| regex.is_match(desc))
            .collect()
    } else if let Some(query) = search_query.strip_prefix('\'') {
        // Exact match
        entries.into_iter()
            .filter(|(_, _, desc)| desc == query)
            .collect()
    } else if exact {
        // Case-sensitive exact match
        entries.into_iter()
            .filter(|(_, _, desc)| desc.contains(search_query))
            .collect()
    } else {
        // Smart case matching (case-insensitive by default)
        let query_lower = search_query.to_lowercase();
        entries.into_iter()
            .filter(|(_, _, desc)| desc.to_lowercase().contains(&query_lower))
            .collect()
    };
    
    Ok(filtered)
}

fn filter_by_tag(
    doing_file: &crate::models::DoingFile,
    entries: Vec<(String, DateTime<Local>, String)>,
    tag_query: &str,
) -> Result<Vec<(String, DateTime<Local>, String)>, color_eyre::eyre::Error> {
    let tags: Vec<&str> = tag_query.split(',').map(|s| s.trim()).collect();
    
    let filtered = entries.into_iter()
        .filter(|(section, timestamp, description)| {
            // Find the actual entry to check tags
            if let Some(section_entries) = doing_file.sections.get(section) {
                for entry in section_entries {
                    if entry.timestamp == *timestamp && entry.description == *description {
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
                    }
                }
            }
            false
        })
        .collect();
    
    Ok(filtered)
}