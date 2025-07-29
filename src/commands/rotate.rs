use crate::storage::{Config, parse_taskpaper, save_taskpaper, DoingFile};
use chrono::Local;
use color_eyre::Result;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RotateOptions {
    pub before: Option<String>,
    pub _bool_op: String,
    pub case: String,
    pub keep: Option<usize>,
    pub not: bool,
    pub section: Option<String>,
    pub search: Option<String>,
    pub tag: Option<String>,
    pub val: Vec<String>,
    pub exact: bool,
}

pub fn handle_rotate(opts: RotateOptions) -> Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let mut doing_file = parse_taskpaper(&doing_file_path)?;

    // Calculate archive file path
    let archive_path = get_archive_path(doing_file_path.as_path());

    // Load or create archive file
    let mut archive_file = if archive_path.exists() {
        parse_taskpaper(&archive_path)?
    } else {
        DoingFile::new(archive_path.clone())
    };

    // Ensure Archive section exists in archive file
    if !archive_file.sections.contains_key("Archive") {
        archive_file.sections.insert("Archive".to_string(), Vec::new());
    }

    let mut entries_to_rotate = Vec::new();

    // Parse before date
    let before_date = opts.before.as_ref().and_then(|s| chrono_english::parse_date_string(
        s, Local::now(), chrono_english::Dialect::Us
    ).ok());

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

    // Determine which sections to process
    let sections_to_process: Vec<String> = if let Some(ref section_name) = opts.section {
        if doing_file.sections.contains_key(section_name) {
            vec![section_name.clone()]
        } else {
            eprintln!("Section '{section_name}' not found");
            return Ok(());
        }
    } else {
        // Process all sections
        doing_file.sections.keys().cloned().collect()
    };

    // Collect entries to rotate from each section
    for section_name in sections_to_process {
        if let Some(entries) = doing_file.sections.get(&section_name) {
            let mut indices_to_rotate = Vec::new();

            for (index, entry) in entries.iter().enumerate() {
                let mut matches = true;

                // Only rotate entries marked as @done by default
                if !entry.tags.contains_key("done") {
                    matches = false;
                }

                // Apply before date filter
                if let Some(before) = &before_date {
                    if entry.timestamp >= *before {
                        matches = false;
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
                    indices_to_rotate.push(index);
                }
            }

            // Apply keep filter - keep only the oldest entries
            if let Some(keep_count) = opts.keep {
                if indices_to_rotate.len() > keep_count {
                    // Keep the first N entries (oldest)
                    indices_to_rotate.truncate(keep_count);
                }
            }

            // Collect entries in reverse order to maintain indices
            for &index in indices_to_rotate.iter().rev() {
                entries_to_rotate.push((section_name.clone(), index));
            }
        }
    }

    // Rotate entries
    let mut rotated_count = 0;
    let mut rotated_entries = Vec::new();

    for (section_name, index) in entries_to_rotate {
        if let Some(entries) = doing_file.sections.get_mut(&section_name) {
            if index < entries.len() {
                let entry = entries.remove(index);
                
                // Add section tag if not from Archive
                let mut entry_to_archive = entry.clone();
                if section_name != "Archive" {
                    entry_to_archive.tags.insert(format!("from_{}", section_name.to_lowercase()), None);
                }
                
                rotated_entries.push(entry_to_archive);
                rotated_count += 1;
            }
        }
    }

    // Add rotated entries to archive file
    if let Some(archive_entries) = archive_file.sections.get_mut("Archive") {
        // Insert at the beginning to maintain chronological order
        for entry in rotated_entries.into_iter().rev() {
            archive_entries.insert(0, entry);
        }
    }

    if rotated_count > 0 {
        // Save both files
        save_taskpaper(&doing_file)?;
        save_taskpaper_to_path(&archive_file, &archive_path)?;
        
        println!("Rotated {} {} to {}", 
            rotated_count,
            if rotated_count == 1 { "entry" } else { "entries" },
            archive_path.display()
        );
    } else {
        println!("No entries found matching the specified criteria");
    }

    Ok(())
}

fn get_archive_path(doing_file_path: &Path) -> PathBuf {
    let file_stem = doing_file_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("doing");
    
    let archive_name = format!("{file_stem}_archive.taskpaper");
    
    if let Some(parent) = doing_file_path.parent() {
        parent.join(archive_name)
    } else {
        PathBuf::from(archive_name)
    }
}

fn save_taskpaper_to_path(doing_file: &DoingFile, path: &PathBuf) -> Result<()> {
    let content = crate::storage::format_taskpaper(doing_file);
    fs::write(path, content)?;
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