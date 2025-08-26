use crate::models::Entry;
use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use chrono::{DateTime, Local};
use chrono_english::{Dialect, parse_date_string};
use regex::Regex;

#[derive(Debug)]
pub struct ResetOptions {
    pub date_string: Option<String>,
    pub _bool_op: String,
    pub case: String,
    pub from: Option<String>,
    pub interactive: bool,
    pub no_resume: bool,
    pub not: bool,
    pub resume: bool,
    pub sections: Vec<String>,
    pub search: Option<String>,
    pub took: Option<String>,
    pub tag: Option<String>,
    pub _val: Vec<String>,
    pub exact: bool,
}

pub fn handle_reset(opts: ResetOptions) -> color_eyre::Result<()> {
    if opts.interactive {
        return Err(color_eyre::eyre::eyre!(
            "Interactive mode not yet implemented"
        ));
    }

    let config = Config::load();
    let doing_file_path = config.doing_file_path();

    let mut doing_file = parse_taskpaper(&doing_file_path)?;

    // Find the entry to modify
    let entry_to_modify = find_entry_to_modify(
        &doing_file,
        &opts.sections,
        opts.search.as_deref(),
        opts.tag.as_deref(),
        opts.exact,
        opts.not,
        &opts.case,
    )?;

    if entry_to_modify.is_none() {
        return Err(color_eyre::eyre::eyre!("No matching entry found"));
    }

    let (target_section, target_uuid) = entry_to_modify.unwrap();

    // Parse the new start time
    let new_start_time = if let Some(from_range) = &opts.from {
        // Parse from time range
        parse_from_range(from_range)?
    } else if let Some(date_str) = &opts.date_string {
        // Parse provided date string
        parse_date_string(date_str, Local::now(), Dialect::Us)?
    } else {
        // Use current time
        Local::now()
    };

    // Modify the entry
    if let Some(entries) = doing_file.sections.get_mut(&target_section) {
        for entry in entries.iter_mut() {
            if entry.uuid == target_uuid {
                let old_timestamp = entry.timestamp;
                entry.timestamp = new_start_time;

                // Handle resume (remove @done) unless explicitly disabled
                let should_resume = opts.resume && !opts.no_resume && opts.took.is_none();
                if should_resume {
                    entry.tags.remove("done");
                }

                // Handle --took option
                if let Some(took_str) = &opts.took {
                    let duration = parse_duration(took_str)?;
                    let done_time = new_start_time + duration;
                    entry.tags.insert(
                        "done".to_string(),
                        Some(done_time.format("%Y-%m-%d %H:%M").to_string()),
                    );
                }

                // Print result
                println!("Reset start time for entry:");
                println!(
                    "Old: {}: {}",
                    old_timestamp.format("%Y-%m-%d %H:%M"),
                    entry.description
                );
                println!(
                    "New: {}: {} {}",
                    entry.timestamp.format("%Y-%m-%d %H:%M"),
                    entry.description,
                    format_tags(&entry.tags)
                );

                if should_resume && old_timestamp != entry.timestamp {
                    println!("Entry resumed.");
                }

                break;
            }
        }
    }

    save_taskpaper(&doing_file)?;

    Ok(())
}

fn parse_from_range(from_range: &str) -> Result<DateTime<Local>, color_eyre::eyre::Error> {
    // Parse "from X to Y" format
    let parts: Vec<&str> = from_range.split(" to ").collect();
    if parts.is_empty() {
        return Err(color_eyre::eyre::eyre!("Invalid from range format"));
    }

    // Parse the start time
    let start_str = parts[0].trim();
    let start_time = parse_date_string(start_str, Local::now(), Dialect::Us)?;

    Ok(start_time)
}

fn parse_duration(duration_str: &str) -> Result<chrono::Duration, color_eyre::eyre::Error> {
    // Check for HH:MM format
    if duration_str.contains(':') {
        let parts: Vec<&str> = duration_str.split(':').collect();
        if parts.len() == 2 {
            let hours: i64 = parts[0].parse()?;
            let minutes: i64 = parts[1].parse()?;
            return Ok(chrono::Duration::hours(hours) + chrono::Duration::minutes(minutes));
        }
    }

    // Check for XX[mhd] format
    let re = Regex::new(r"^(\d+)([mhd])$")?;
    if let Some(captures) = re.captures(duration_str) {
        let value: i64 = captures[1].parse()?;
        let unit = &captures[2];

        let duration = match unit {
            "m" => chrono::Duration::minutes(value),
            "h" => chrono::Duration::hours(value),
            "d" => chrono::Duration::days(value),
            _ => return Err(color_eyre::eyre::eyre!("Invalid duration unit")),
        };

        return Ok(duration);
    }

    Err(color_eyre::eyre::eyre!("Invalid duration format"))
}

fn format_tags(tags: &std::collections::HashMap<String, Option<String>>) -> String {
    let mut tag_strs: Vec<String> = tags
        .iter()
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

fn find_entry_to_modify(
    doing_file: &crate::models::DoingFile,
    sections: &[String],
    search: Option<&str>,
    tag: Option<&str>,
    exact: bool,
    not: bool,
    case: &str,
) -> Result<Option<(String, uuid::Uuid)>, color_eyre::eyre::Error> {
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

        let filtered_uuids: std::collections::HashSet<_> =
            filtered_entries.iter().map(|(_, e)| e.uuid).collect();

        filtered_entries = all_entries_again
            .into_iter()
            .filter(|(_, entry)| !filtered_uuids.contains(&entry.uuid))
            .collect();
    }

    // Take the most recent entry
    if let Some((section, entry)) = filtered_entries.into_iter().next() {
        Ok(Some((section, entry.uuid)))
    } else {
        Ok(None)
    }
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
        "s" | "smart" => search_query.chars().any(|c| c.is_uppercase()),
        _ => false,
    };

    let filtered = if search_query.starts_with('/') && search_query.ends_with('/') {
        // Regex search
        let pattern = &search_query[1..search_query.len() - 1];
        let regex = if case_sensitive {
            Regex::new(pattern)?
        } else {
            Regex::new(&format!("(?i){pattern}"))?
        };
        entries
            .into_iter()
            .filter(|(_, entry)| regex.is_match(&entry.description))
            .collect()
    } else if let Some(query) = search_query.strip_prefix('\'') {
        // Exact match
        entries
            .into_iter()
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
        entries
            .into_iter()
            .filter(|(_, entry)| {
                if case_sensitive {
                    entry.description.contains(search_query)
                } else {
                    entry
                        .description
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
                }
            })
            .collect()
    } else {
        // Regular substring matching
        entries
            .into_iter()
            .filter(|(_, entry)| {
                if case_sensitive {
                    entry.description.contains(search_query)
                } else {
                    entry
                        .description
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
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

    let filtered = entries
        .into_iter()
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
