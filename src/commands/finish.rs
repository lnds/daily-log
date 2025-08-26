use crate::models::Entry;
use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use chrono::{DateTime, Duration, Local};
use chrono_english::{Dialect, parse_date_string};
use regex::Regex;

#[derive(Debug)]
pub struct FinishOptions {
    pub count: usize,
    pub archive: bool,
    pub at: Option<String>,
    pub auto: bool,
    pub back: Option<String>,
    pub from: Option<String>,
    pub interactive: bool,
    pub not: bool,
    pub remove: bool,
    pub sections: Vec<String>,
    pub search: Option<String>,
    pub took: Option<String>,
    pub tag: Option<String>,
    pub unfinished: bool,
    pub update: bool,
    pub exact: bool,
    pub date: bool,
}

pub fn handle_finish(opts: FinishOptions) -> color_eyre::Result<()> {
    if opts.interactive {
        return Err(color_eyre::eyre::eyre!(
            "Interactive mode not yet implemented"
        ));
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

    // Apply unfinished filter
    if opts.unfinished {
        filtered_entries = filter_unfinished(&doing_file, filtered_entries)?;
    }

    // Apply NOT filter if specified
    if opts.not {
        // Get all entries again and remove the filtered ones
        let mut all_entries_again: Vec<(String, DateTime<Local>, String)> = Vec::new();
        for section in &target_sections {
            if let Some(entries) = doing_file.sections.get(section) {
                for entry in entries {
                    all_entries_again.push((
                        section.clone(),
                        entry.timestamp,
                        entry.description.clone(),
                    ));
                }
            }
        }
        all_entries_again.sort_by(|a, b| b.1.cmp(&a.1));

        filtered_entries = all_entries_again
            .into_iter()
            .filter(|entry| !filtered_entries.contains(entry))
            .collect();
    }

    // Take only the requested count
    let entries_to_finish: Vec<_> = filtered_entries.into_iter().take(opts.count).collect();

    if entries_to_finish.is_empty() {
        return Err(color_eyre::eyre::eyre!(
            "No matching entries found to finish"
        ));
    }

    // Process each entry
    let mut finished_count = 0;
    type EntryUpdate = (String, DateTime<Local>, String, Option<DateTime<Local>>);
    let mut updates: Vec<EntryUpdate> = Vec::new();

    // First pass: collect updates without mutating
    for (section, timestamp, description) in entries_to_finish {
        if let Some(entries) = doing_file.sections.get(&section) {
            for entry in entries {
                if entry.timestamp == timestamp && entry.description == description {
                    // Skip if already done and not updating
                    if entry.is_done() && !opts.update && !opts.remove {
                        continue;
                    }

                    if opts.remove {
                        updates.push((section.clone(), timestamp, description.clone(), None));
                    } else {
                        // Calculate done time
                        let done_time = if opts.auto {
                            calculate_auto_done_time(&doing_file, &entry.timestamp)?
                        } else if let Some(from_str) = &opts.from {
                            let (_, end_time) = parse_from_range(from_str)?;
                            end_time
                        } else {
                            calculate_done_time(&opts.at, &opts.back, &opts.took, &entry.timestamp)?
                        };

                        if opts.date {
                            updates.push((
                                section.clone(),
                                timestamp,
                                description.clone(),
                                Some(done_time),
                            ));
                        } else {
                            // For cancel command - no timestamp
                            updates.push((
                                section.clone(),
                                timestamp,
                                description.clone(),
                                Some(Local::now()),
                            ));
                        }
                    }
                    break;
                }
            }
        }
    }

    // Second pass: apply updates
    for (section, timestamp, description, done_time) in updates {
        if let Some(entries) = doing_file.sections.get_mut(&section) {
            for entry in entries.iter_mut() {
                if entry.timestamp == timestamp && entry.description == description {
                    if done_time.is_none() {
                        // Remove the done tag
                        entry.tags.remove("done");
                        println!("Removed @done tag from: {}", entry.description);
                    } else if let Some(dt) = done_time {
                        // Add or update done tag
                        if opts.date {
                            entry.tags.insert(
                                "done".to_string(),
                                Some(dt.format("%Y-%m-%d %H:%M").to_string()),
                            );

                            println!(
                                "{}: {} @done({})",
                                entry.timestamp.format("%Y-%m-%d %H:%M"),
                                entry.description,
                                dt.format("%Y-%m-%d %H:%M")
                            );
                        } else {
                            // No date - just add @done without timestamp
                            entry.tags.insert("done".to_string(), None);

                            println!(
                                "{}: {} @done",
                                entry.timestamp.format("%Y-%m-%d %H:%M"),
                                entry.description
                            );
                        }
                    }

                    finished_count += 1;
                    break;
                }
            }
        }
    }

    // Archive entries if requested
    if opts.archive && !opts.remove {
        let mut entries_to_archive = Vec::new();

        for section in &target_sections {
            if let Some(entries) = doing_file.sections.get_mut(section) {
                let mut remaining = Vec::new();

                for entry in entries.drain(..) {
                    if entry.is_done() {
                        entries_to_archive.push(entry);
                    } else {
                        remaining.push(entry);
                    }
                }

                *entries = remaining;
            }
        }

        // Add to Archive section
        for entry in entries_to_archive {
            doing_file.add_entry_to_section(entry, "Archive".to_string());
        }
    }

    save_taskpaper(&doing_file)?;

    if finished_count == 0 {
        return Err(color_eyre::eyre::eyre!("No entries were finished"));
    }

    Ok(())
}

fn filter_by_search(
    entries: Vec<(String, DateTime<Local>, String)>,
    search_query: &str,
    exact: bool,
) -> Result<Vec<(String, DateTime<Local>, String)>, color_eyre::eyre::Error> {
    let filtered = if search_query.starts_with('/') && search_query.ends_with('/') {
        // Regex search
        let pattern = &search_query[1..search_query.len() - 1];
        let regex = Regex::new(pattern)?;
        entries
            .into_iter()
            .filter(|(_, _, desc)| regex.is_match(desc))
            .collect()
    } else if let Some(query) = search_query.strip_prefix('\'') {
        // Exact match
        entries
            .into_iter()
            .filter(|(_, _, desc)| desc == query)
            .collect()
    } else if exact {
        // Case-sensitive exact match
        entries
            .into_iter()
            .filter(|(_, _, desc)| desc.contains(search_query))
            .collect()
    } else {
        // Smart case matching (case-insensitive by default)
        let query_lower = search_query.to_lowercase();
        entries
            .into_iter()
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

    let filtered = entries
        .into_iter()
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

fn filter_unfinished(
    doing_file: &crate::models::DoingFile,
    entries: Vec<(String, DateTime<Local>, String)>,
) -> Result<Vec<(String, DateTime<Local>, String)>, color_eyre::eyre::Error> {
    let filtered = entries
        .into_iter()
        .filter(|(section, timestamp, description)| {
            // Find the actual entry to check if done
            if let Some(section_entries) = doing_file.sections.get(section) {
                for entry in section_entries {
                    if entry.timestamp == *timestamp && entry.description == *description {
                        return !entry.is_done();
                    }
                }
            }
            false
        })
        .collect();

    Ok(filtered)
}

fn calculate_auto_done_time(
    doing_file: &crate::models::DoingFile,
    entry_time: &DateTime<Local>,
) -> Result<DateTime<Local>, color_eyre::eyre::Error> {
    // Find the next entry after this one
    let mut all_entries: Vec<&Entry> = Vec::new();

    for entries in doing_file.sections.values() {
        for entry in entries {
            all_entries.push(entry);
        }
    }

    // Sort by timestamp
    all_entries.sort_by_key(|e| e.timestamp);

    // Find the entry after our target entry
    for i in 0..all_entries.len() {
        if all_entries[i].timestamp == *entry_time {
            if i + 1 < all_entries.len() {
                // Return 1 minute before the next entry
                return Ok(all_entries[i + 1].timestamp - Duration::minutes(1));
            }
            break;
        }
    }

    // If no next entry found, use current time
    Ok(Local::now())
}

fn calculate_done_time(
    at: &Option<String>,
    back: &Option<String>,
    took: &Option<String>,
    start_time: &DateTime<Local>,
) -> Result<DateTime<Local>, color_eyre::eyre::Error> {
    if let Some(at_str) = at {
        // Done time is explicitly set
        parse_date_string(at_str, Local::now(), Dialect::Us)
            .map_err(|_| color_eyre::eyre::eyre!("Invalid date string: {}", at_str))
    } else if let Some(back_str) = back {
        // Done time is backdated from now
        let back_from = Local::now();
        parse_date_string(back_str, back_from, Dialect::Us)
            .map_err(|_| color_eyre::eyre::eyre!("Invalid date string: {}", back_str))
    } else if let Some(took_str) = took {
        // Done time is start time plus duration
        let duration = parse_duration(took_str)?;
        Ok(*start_time + duration)
    } else {
        // Done time is now
        Ok(Local::now())
    }
}

fn parse_duration(duration_str: &str) -> Result<Duration, color_eyre::eyre::Error> {
    // Try to parse as HH:MM
    if let Ok(re) = Regex::new(r"^(\d+):(\d+)$")
        && let Some(captures) = re.captures(duration_str) {
            let hours: i64 = captures[1].parse()?;
            let minutes: i64 = captures[2].parse()?;
            return Ok(Duration::hours(hours) + Duration::minutes(minutes));
        }

    // Try to parse as XX[mhd]
    if let Ok(re) = Regex::new(r"^(\d+)([mhd])$")
        && let Some(captures) = re.captures(duration_str) {
            let value: i64 = captures[1].parse()?;
            let unit = &captures[2];

            return match unit {
                "m" => Ok(Duration::minutes(value)),
                "h" => Ok(Duration::hours(value)),
                "d" => Ok(Duration::days(value)),
                _ => Err(color_eyre::eyre::eyre!("Invalid duration unit: {}", unit)),
            };
        }

    Err(color_eyre::eyre::eyre!(
        "Invalid duration format: {}. Use XX[mhd] or HH:MM",
        duration_str
    ))
}

fn parse_from_range(
    from_str: &str,
) -> Result<(DateTime<Local>, DateTime<Local>), color_eyre::eyre::Error> {
    // Parse "from X to Y" or just "X to Y"
    let from_regex = Regex::new(r"(?i)(?:from\s+)?(.+?)\s+to\s+(.+)$")?;

    if let Some(captures) = from_regex.captures(from_str) {
        let start_str = &captures[1];
        let end_str = &captures[2];

        let start_time = parse_date_string(start_str, Local::now(), Dialect::Us)
            .map_err(|_| color_eyre::eyre::eyre!("Invalid start time: {}", start_str))?;

        let end_time = parse_date_string(end_str, start_time, Dialect::Us)
            .map_err(|_| color_eyre::eyre::eyre!("Invalid end time: {}", end_str))?;

        Ok((start_time, end_time))
    } else {
        Err(color_eyre::eyre::eyre!("Invalid from format. Use: X to Y"))
    }
}
