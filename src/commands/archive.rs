use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use chrono::Local;
use color_eyre::Result;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ArchiveOptions {
    pub target: Option<String>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub _bool_op: String,
    pub case: String,
    pub from: Option<String>,
    pub keep: Option<usize>,
    pub label: bool,
    pub not: bool,
    pub search: Option<String>,
    pub to: String,
    pub tag: Option<String>,
    pub val: Vec<String>,
    pub exact: bool,
}

pub fn handle_archive(opts: ArchiveOptions) -> Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let mut doing_file = parse_taskpaper(&doing_file_path)?;

    // Ensure destination section exists
    if !doing_file.sections.contains_key(&opts.to) {
        doing_file.sections.insert(opts.to.clone(), Vec::new());
    }

    let mut entries_to_move = Vec::new();
    let mut source_sections = Vec::new();

    // Parse date filters
    let after_date = opts.after.as_ref().and_then(|s| chrono_english::parse_date_string(
        s, Local::now(), chrono_english::Dialect::Us
    ).ok());
    let before_date = opts.before.as_ref().and_then(|s| chrono_english::parse_date_string(
        s, Local::now(), chrono_english::Dialect::Us
    ).ok());
    let date_range = opts.from.as_ref().and_then(|s| {
        // Simple date range parsing - expects "YYYY-MM-DD to YYYY-MM-DD" format
        let parts: Vec<&str> = s.split(" to ").collect();
        if parts.len() == 2 {
            let start = chrono_english::parse_date_string(
                parts[0], Local::now(), chrono_english::Dialect::Us
            ).ok();
            let end = chrono_english::parse_date_string(
                parts[1], Local::now(), chrono_english::Dialect::Us
            ).ok();
            match (start, end) {
                (Some(s), Some(e)) => Some((s, e)),
                _ => None,
            }
        } else {
            None
        }
    });

    // Determine which sections to process
    let sections_to_process: Vec<String> = if let Some(target_str) = &opts.target {
        if target_str.starts_with('@') {
            // If target is a tag, process all sections
            doing_file.sections.keys().cloned().collect()
        } else {
            // If target is a section name
            if doing_file.sections.contains_key(target_str) {
                vec![target_str.clone()]
            } else {
                eprintln!("Section '{target_str}' not found");
                return Ok(());
            }
        }
    } else {
        // No target specified, process all sections except destination
        doing_file.sections.keys()
            .filter(|k| *k != &opts.to)
            .cloned()
            .collect()
    };

    // Compile search patterns
    let search_regex = if let Some(ref pattern) = opts.search {
        Some(compile_search_regex(pattern, &opts.case, opts.exact)?)
    } else {
        None
    };

    let tag_regex = if let Some(ref tag_pattern) = opts.tag {
        Some(compile_tag_regex(tag_pattern)?)
    } else {
        None
    };

    let tag_value_queries = compile_tag_value_queries(&opts.val)?;

    // Collect entries to move from each section
    for section_name in sections_to_process {
        if section_name == opts.to {
            continue; // Don't move from destination to itself
        }

        if let Some(entries) = doing_file.sections.get(&section_name) {
            let mut indices_to_move = Vec::new();

            for (index, entry) in entries.iter().enumerate() {
                let mut matches = true;

                // Apply date filters
                if let Some(after) = &after_date {
                    if entry.timestamp <= *after {
                        matches = false;
                    }
                }
                if let Some(before) = &before_date {
                    if entry.timestamp >= *before {
                        matches = false;
                    }
                }
                if let Some((start, end)) = &date_range {
                    if entry.timestamp < *start || entry.timestamp > *end {
                        matches = false;
                    }
                }

                // Apply target filter (tag)
                if let Some(target_str) = &opts.target {
                    if target_str.starts_with('@') {
                        let tag_name = target_str.trim_start_matches('@');
                        if !entry.tags.contains_key(tag_name) {
                            matches = false;
                        }
                    }
                }

                // Apply search filter
                if let Some(ref regex) = search_regex {
                    if !regex.is_match(&entry.description) && 
                       !entry.note.as_ref().is_some_and(|n| regex.is_match(n)) {
                        matches = false;
                    }
                }

                // Apply tag filter
                if let Some(ref regex) = tag_regex {
                    let has_matching_tag = entry.tags.keys().any(|t| regex.is_match(t));
                    if !has_matching_tag {
                        matches = false;
                    }
                }

                // Apply tag value queries
                for (tag_name, value_pattern) in &tag_value_queries {
                    match entry.tags.get(tag_name) {
                        Some(Some(value)) => {
                            if !value_pattern.is_match(value) {
                                matches = false;
                            }
                        }
                        _ => matches = false,
                    }
                }

                // Apply not filter
                if opts.not {
                    matches = !matches;
                }

                if matches {
                    indices_to_move.push(index);
                }
            }

            // Apply keep filter - keep only the most recent N entries
            if let Some(keep_count) = opts.keep {
                if indices_to_move.len() > keep_count {
                    // Keep the last N entries (most recent)
                    indices_to_move = indices_to_move.into_iter()
                        .rev()
                        .take(keep_count)
                        .rev()
                        .collect();
                }
            }

            // Collect entries in reverse order to maintain indices
            for &index in indices_to_move.iter().rev() {
                entries_to_move.push((section_name.clone(), index));
            }
        }
    }

    // Move entries
    let mut moved_count = 0;
    for (section_name, index) in entries_to_move {
        if let Some(entries) = doing_file.sections.get_mut(&section_name) {
            if index < entries.len() {
                let mut entry = entries.remove(index);
                
                // Add label if requested
                if opts.label && section_name != "Currently" {
                    entry.tags.insert(format!("from_{}", section_name.to_lowercase()), None);
                }
                
                // Add to destination section
                if let Some(dest_entries) = doing_file.sections.get_mut(&opts.to) {
                    dest_entries.insert(0, entry);
                    moved_count += 1;
                    if !source_sections.contains(&section_name) {
                        source_sections.push(section_name);
                    }
                }
            }
        }
    }

    if moved_count > 0 {
        save_taskpaper(&doing_file)?;
        println!("Moved {} {} from {} to {}", 
            moved_count,
            if moved_count == 1 { "entry" } else { "entries" },
            source_sections.join(", "),
            opts.to
        );
    } else {
        println!("No entries found matching the specified criteria");
    }

    Ok(())
}

fn compile_search_regex(pattern: &str, case: &str, exact: bool) -> Result<Regex> {
    let pattern = if exact {
        regex::escape(pattern)
    } else {
        pattern.to_string()
    };

    let case_insensitive = parse_smart_case(case, &pattern);
    
    let regex = if case_insensitive {
        Regex::new(&format!("(?i){pattern}"))?
    } else {
        Regex::new(&pattern)?
    };

    Ok(regex)
}

fn compile_tag_regex(pattern: &str) -> Result<Regex> {
    let pattern = pattern.trim_start_matches('@');
    Ok(Regex::new(&format!("^{pattern}$"))?)
}

fn parse_smart_case(case: &str, pattern: &str) -> bool {
    match case {
        "i" | "ignore" => true,
        "c" | "case-sensitive" => false,
        "s" | "smart" => {
            // Smart case: case-insensitive unless pattern contains uppercase
            !pattern.chars().any(|c| c.is_uppercase())
        }
        _ => {
            // Default to smart case
            !pattern.chars().any(|c| c.is_uppercase())
        }
    }
}

fn compile_tag_value_queries(val: &[String]) -> Result<HashMap<String, Regex>> {
    let mut queries = HashMap::new();
    
    for query in val {
        // Parse tag value queries in format "tag_name=pattern"
        if let Some(eq_pos) = query.find('=') {
            let tag_name = query[..eq_pos].trim().to_string();
            let pattern = query[eq_pos + 1..].trim();
            let regex = Regex::new(pattern)?;
            queries.insert(tag_name, regex);
        }
    }
    
    Ok(queries)
}