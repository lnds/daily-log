use crate::models::Entry;
use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use chrono::Local;
use regex::Regex;
use std::io::{self, Write};

pub fn handle_tag(
    tags: Vec<String>,
    _autotag: bool,
    _bool_op: String,
    count: usize,
    case: String,
    date: bool,
    force: bool,
    interactive: bool,
    not: bool,
    remove: bool,
    regex: bool,
    rename: Option<String>,
    sections: Vec<String>,
    search: Option<String>,
    tag: Option<String>,
    unfinished: bool,
    value: Option<String>,
    _val: Vec<String>,
    exact: bool,
) -> color_eyre::Result<()> {
    if interactive {
        return Err(color_eyre::eyre::eyre!("Interactive mode not yet implemented"));
    }

    // Validate count and force
    if count == 0 && !force {
        print!("Are you sure you want to tag all entries? [y/N] ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Tag operation cancelled.");
            return Ok(());
        }
    }

    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    
    let mut doing_file = parse_taskpaper(&doing_file_path)?;
    
    // Find entries to modify
    let entries_to_modify = find_entries_to_modify(
        &doing_file,
        &sections,
        search.as_deref(),
        tag.as_deref(),
        count,
        unfinished,
        exact,
        not,
        &case,
    )?;

    if entries_to_modify.is_empty() {
        return Err(color_eyre::eyre::eyre!("No matching entries found"));
    }

    // Process tags
    let mut tags_to_process: Vec<(String, Option<String>)> = Vec::new();
    for tag_str in &tags {
        let tag_name = if let Some(stripped) = tag_str.strip_prefix('@') {
            stripped.to_string()
        } else {
            tag_str.to_string()
        };
        
        if let Some(val) = &value {
            tags_to_process.push((tag_name, Some(val.clone())));
        } else if date {
            tags_to_process.push((tag_name, Some(Local::now().format("%Y-%m-%d %H:%M").to_string())));
        } else {
            tags_to_process.push((tag_name, None));
        }
    }

    // Modify entries
    let mut modified_count = 0;
    for (target_section, target_uuid) in entries_to_modify {
        if let Some(entries) = doing_file.sections.get_mut(&target_section) {
            for entry in entries.iter_mut() {
                if entry.uuid == target_uuid {
                    if let Some(ref rename_from) = rename {
                        // Rename existing tags
                        rename_tags(entry, rename_from, &tags_to_process[0].0, regex, &case)?;
                    } else if remove {
                        // Remove tags
                        remove_tags(entry, &tags, regex, &case)?;
                    } else {
                        // Add tags
                        for (tag_name, tag_value) in &tags_to_process {
                            entry.tags.insert(tag_name.clone(), tag_value.clone());
                        }
                    }
                    
                    // Print updated entry
                    println!("{}: {} {}", 
                        entry.timestamp.format("%Y-%m-%d %H:%M"),
                        entry.description,
                        format_tags(&entry.tags)
                    );
                    
                    modified_count += 1;
                    break;
                }
            }
        }
    }
    
    save_taskpaper(&doing_file)?;
    
    println!("\nTagged {} {}.", 
        modified_count, 
        if modified_count == 1 { "entry" } else { "entries" }
    );
    
    Ok(())
}

fn format_tags(tags: &std::collections::HashMap<String, Option<String>>) -> String {
    let mut tag_strs: Vec<String> = tags.iter()
        .map(|(tag, value)| {
            if let Some(val) = value {
                format!("@{tag}({val})")
            } else {
                format!("@{tag}")
            }
        })
        .collect();
    tag_strs.sort();
    tag_strs.join(" ")
}

fn remove_tags(
    entry: &mut Entry,
    tags_to_remove: &[String],
    use_regex: bool,
    case: &str,
) -> Result<(), color_eyre::eyre::Error> {
    for tag_pattern in tags_to_remove {
        let pattern = if let Some(stripped) = tag_pattern.strip_prefix('@') {
            stripped
        } else {
            tag_pattern
        };
        
        if use_regex {
            let regex = create_regex(pattern, case)?;
            entry.tags.retain(|tag_name, _| !regex.is_match(tag_name));
        } else {
            // Wildcard matching
            let regex_pattern = pattern
                .replace('*', ".*")
                .replace('?', ".");
            let regex = create_regex(&regex_pattern, case)?;
            entry.tags.retain(|tag_name, _| !regex.is_match(tag_name));
        }
    }
    Ok(())
}

fn rename_tags(
    entry: &mut Entry,
    from_pattern: &str,
    to_tag: &str,
    use_regex: bool,
    case: &str,
) -> Result<(), color_eyre::eyre::Error> {
    let pattern = if let Some(stripped) = from_pattern.strip_prefix('@') {
        stripped
    } else {
        from_pattern
    };
    
    let regex = if use_regex {
        create_regex(pattern, case)?
    } else {
        // Wildcard matching
        let regex_pattern = pattern
            .replace('*', ".*")
            .replace('?', ".");
        create_regex(&regex_pattern, case)?
    };
    
    let mut tags_to_rename = Vec::new();
    for (tag_name, tag_value) in &entry.tags {
        if regex.is_match(tag_name) {
            tags_to_rename.push((tag_name.clone(), tag_value.clone()));
        }
    }
    
    for (old_tag, value) in tags_to_rename {
        entry.tags.remove(&old_tag);
        entry.tags.insert(to_tag.to_string(), value);
    }
    
    Ok(())
}

fn create_regex(pattern: &str, case: &str) -> Result<Regex, color_eyre::eyre::Error> {
    let case_sensitive = match case {
        "c" | "case-sensitive" => true,
        "i" | "ignore" => false,
        "s" | "smart" => {
            // Smart case: case-sensitive if pattern contains uppercase
            pattern.chars().any(|c| c.is_uppercase())
        }
        _ => false,
    };
    
    let regex = if case_sensitive {
        Regex::new(&format!("^{pattern}$"))?
    } else {
        Regex::new(&format!("(?i)^{pattern}$"))?
    };
    
    Ok(regex)
}

fn find_entries_to_modify(
    doing_file: &crate::models::DoingFile,
    sections: &[String],
    search: Option<&str>,
    tag: Option<&str>,
    count: usize,
    unfinished: bool,
    exact: bool,
    not: bool,
    case: &str,
) -> Result<Vec<(String, uuid::Uuid)>, color_eyre::eyre::Error> {
    // Determine which sections to search
    let target_sections: Vec<String> = if sections.is_empty() {
        doing_file.sections.keys().cloned().collect()
    } else {
        sections.to_vec()
    };
    
    // Collect all entries from target sections
    let mut all_entries: Vec<(String, Entry)> = Vec::new();
    for section in &target_sections {
        if let Some(entries) = doing_file.sections.get(section) {
            for entry in entries {
                all_entries.push((section.clone(), entry.clone()));
            }
        }
    }
    
    // Sort by timestamp (newest first)
    all_entries.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
    
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
    
    // Apply unfinished filter
    if unfinished {
        filtered_entries.retain(|(_, entry)| !entry.is_done());
    }
    
    // Apply NOT filter if specified
    if not {
        // Get all entries again
        let mut all_entries_again: Vec<(String, Entry)> = Vec::new();
        for section in &target_sections {
            if let Some(entries) = doing_file.sections.get(section) {
                for entry in entries {
                    all_entries_again.push((section.clone(), entry.clone()));
                }
            }
        }
        all_entries_again.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
        
        let filtered_uuids: std::collections::HashSet<_> = filtered_entries.iter()
            .map(|(_, e)| e.uuid)
            .collect();
        
        filtered_entries = all_entries_again.into_iter()
            .filter(|(_, entry)| !filtered_uuids.contains(&entry.uuid))
            .collect();
    }
    
    // Take only the requested count (0 means all)
    let entries = if count == 0 {
        filtered_entries
    } else {
        filtered_entries.into_iter().take(count).collect()
    };
    
    Ok(entries.into_iter()
        .map(|(section, entry)| (section, entry.uuid))
        .collect())
}

fn filter_by_search(
    entries: Vec<(String, Entry)>,
    search_query: &str,
    exact: bool,
    case: &str,
) -> Result<Vec<(String, Entry)>, color_eyre::eyre::Error> {
    // Determine case sensitivity
    let case_sensitive = match case {
        "c" | "case-sensitive" => true,
        "i" | "ignore" => false,
        "s" | "smart" => {
            search_query.chars().any(|c| c.is_uppercase())
        }
        _ => false,
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
            .filter(|(_, entry)| regex.is_match(&entry.description))
            .collect()
    } else if let Some(query) = search_query.strip_prefix('\'') {
        // Exact match
        entries.into_iter()
            .filter(|(_, entry)| {
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
            .filter(|(_, entry)| {
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
            .filter(|(_, entry)| {
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
    entries: Vec<(String, Entry)>,
    tag_query: &str,
) -> Result<Vec<(String, Entry)>, color_eyre::eyre::Error> {
    let tags: Vec<&str> = tag_query.split(',').map(|s| s.trim()).collect();
    
    let filtered = entries.into_iter()
        .filter(|(_, entry)| {
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