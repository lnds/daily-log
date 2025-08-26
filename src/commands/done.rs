use crate::models::Entry;
use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use chrono::{DateTime, Duration, Local};
use chrono_english::{Dialect, parse_date_string};
use regex::Regex;
use std::io::{self, Write};

#[derive(Debug)]
pub struct DoneOptions {
    pub entry: Vec<String>,
    pub note: Option<String>,
    pub ask: bool,
    pub back: Option<String>,
    pub at: Option<String>,
    pub took: Option<String>,
    pub from: Option<String>,
    pub section: Option<String>,
    pub editor: bool,
    pub archive: bool,
    pub remove: bool,
    pub unfinished: bool,
    pub _date: bool,
    pub _noauto: bool,
}

pub fn handle_done(opts: DoneOptions) -> color_eyre::Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();

    let mut doing_file = parse_taskpaper(&doing_file_path)?;

    // Handle remove flag - remove @done tag from last entry
    if opts.remove {
        let target_section = opts.section.as_deref().unwrap_or("Currently");

        // Find the last done entry in the section
        let last_entry_info = doing_file
            .get_all_entries()
            .into_iter()
            .filter(|e| e.section == target_section && e.is_done())
            .max_by_key(|e| e.timestamp)
            .map(|e| (e.timestamp, e.description.clone()));

        if let Some((timestamp, description)) = last_entry_info {
            // Remove the done tag
            let mut found = false;
            let mut entry_desc = String::new();

            if let Some(entries) = doing_file.sections.get_mut(target_section) {
                for entry in entries.iter_mut() {
                    if entry.timestamp == timestamp && entry.description == description {
                        entry.tags.remove("done");
                        entry_desc = entry.description.clone();
                        found = true;
                        break;
                    }
                }
            }

            if found {
                save_taskpaper(&doing_file)?;
                println!("Removed @done tag from: {entry_desc}");
                return Ok(());
            }
        } else {
            return Err(color_eyre::eyre::eyre!(
                "No completed entries found to remove @done tag from"
            ));
        }
    }

    // If no entry text provided, mark last entry as done
    if opts.entry.is_empty() {
        let target_section = opts.section.as_deref().unwrap_or("Currently");

        // Find the last entry (unfinished if --unfinished flag is set)
        let last_entry_info = doing_file
            .get_all_entries()
            .into_iter()
            .filter(|e| e.section == target_section && (!opts.unfinished || !e.is_done()))
            .max_by_key(|e| e.timestamp)
            .map(|e| (e.timestamp, e.description.clone()));

        if let Some((timestamp, description)) = last_entry_info {
            // Mark it as done
            let mut found = false;
            let mut entry_info = None;

            if let Some(entries) = doing_file.sections.get_mut(target_section) {
                for entry in entries.iter_mut() {
                    if entry.timestamp == timestamp && entry.description == description {
                        if entry.is_done() && !opts.unfinished {
                            return Err(color_eyre::eyre::eyre!(
                                "Last entry is already marked @done"
                            ));
                        }

                        // Calculate done time based on flags
                        let done_time =
                            calculate_done_time(&opts.at, &opts.took, &entry.timestamp)?;
                        entry.tags.insert(
                            "done".to_string(),
                            Some(done_time.format("%Y-%m-%d %H:%M").to_string()),
                        );

                        entry_info = Some((
                            entry.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                            entry.description.clone(),
                            done_time.format("%Y-%m-%d %H:%M").to_string(),
                        ));
                        found = true;
                        break;
                    }
                }
            }

            if found {
                // If archive flag is set, move the entry to Archive section
                if opts.archive
                    && let Some((time_str, desc, _done_time_str)) = &entry_info
                {
                    // Find and remove from current section
                    if let Some(entries) = doing_file.sections.get_mut(target_section)
                        && let Some(pos) = entries.iter().position(|e| {
                            e.timestamp.format("%Y-%m-%d %H:%M").to_string() == *time_str
                                && e.description == *desc
                        })
                    {
                        let mut entry = entries.remove(pos);
                        entry.section = "Archive".to_string();

                        // Add to Archive section
                        doing_file
                            .sections
                            .entry("Archive".to_string())
                            .or_insert_with(Vec::new)
                            .push(entry);
                    }
                }

                save_taskpaper(&doing_file)?;
                if let Some((time_str, desc, done_time_str)) = entry_info {
                    println!("{time_str}: {desc} @done({done_time_str})");
                }
                return Ok(());
            }
        } else {
            return Err(color_eyre::eyre::eyre!("No entries found to mark as done"));
        }
    }

    // Create a new entry and mark it as done
    let entry_text = if opts.editor {
        // TODO: Implement editor support
        return Err(color_eyre::eyre::eyre!(
            "Editor support not yet implemented"
        ));
    } else if opts.entry.is_empty() {
        // Interactive prompt
        print!("What did you finish? ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    } else {
        opts.entry.join(" ")
    };

    if entry_text.is_empty() {
        return Err(color_eyre::eyre::eyre!("Entry text cannot be empty"));
    }

    // Parse parenthetical at end as note
    let paren_regex = Regex::new(r"^(.+?)\s*\(([^)]+)\)\s*$")?;
    let (final_entry_text, parsed_note) = if let Some(captures) = paren_regex.captures(&entry_text)
    {
        (captures[1].to_string(), Some(captures[2].to_string()))
    } else {
        (entry_text.clone(), None)
    };

    // Get final note
    let final_note = if let Some(n) = opts.note {
        Some(n)
    } else if let Some(n) = parsed_note {
        Some(n)
    } else if opts.ask {
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

    // Determine section
    let target_section = if opts.archive {
        "Archive".to_string()
    } else {
        opts.section.unwrap_or_else(|| "Currently".to_string())
    };

    // Extract tags from entry text
    let tag_regex = Regex::new(r"@(\w+)(?:\(([^)]+)\))?")?;
    let mut tags = Vec::new();
    for capture in tag_regex.captures_iter(&final_entry_text) {
        let tag_name = capture[1].to_string();
        let tag_value = capture.get(2).map(|m| m.as_str().to_string());
        tags.push((tag_name, tag_value));
    }

    // Remove tags from description
    let clean_description = tag_regex
        .replace_all(&final_entry_text, "")
        .trim()
        .to_string();

    // Create entry with clean description
    let mut new_entry = Entry::new(clean_description, target_section);

    // Add tags
    for (tag_name, tag_value) in tags {
        new_entry = new_entry.with_tag(tag_name, tag_value);
    }

    // Handle time calculations
    let (start_time, done_time) = if let Some(from_str) = opts.from {
        // Parse "from X to Y" format
        parse_from_range(&from_str)?
    } else {
        // Calculate times based on other flags
        let (start, done) = calculate_times(opts.back, opts.at, opts.took)?;
        (start, done)
    };

    new_entry = new_entry.with_timestamp(start_time);

    // Add @done tag with timestamp
    new_entry.tags.insert(
        "done".to_string(),
        Some(done_time.format("%Y-%m-%d %H:%M").to_string()),
    );

    // Add note if present
    if let Some(note_text) = final_note {
        new_entry = new_entry.with_note(note_text);
    }

    doing_file.add_entry(new_entry.clone());
    save_taskpaper(&doing_file)?;

    println!(
        "{}: {} @done({})",
        new_entry.timestamp.format("%Y-%m-%d %H:%M"),
        new_entry.description,
        done_time.format("%Y-%m-%d %H:%M")
    );

    if !new_entry.tags.is_empty() {
        let tags_str: Vec<String> = new_entry
            .tags
            .iter()
            .filter(|(k, _)| k != &"done") // Don't show done tag again
            .map(|(k, v)| {
                if let Some(val) = v {
                    format!("@{k}({val})")
                } else {
                    format!("@{k}")
                }
            })
            .collect();
        if !tags_str.is_empty() {
            println!("  {}", tags_str.join(" "));
        }
    }

    if let Some(note) = &new_entry.note {
        println!("  Note: {}", note.lines().next().unwrap_or(""));
        for line in note.lines().skip(1) {
            println!("        {line}");
        }
    }

    Ok(())
}

fn calculate_done_time(
    at: &Option<String>,
    took: &Option<String>,
    start_time: &DateTime<Local>,
) -> Result<DateTime<Local>, color_eyre::eyre::Error> {
    if let Some(at_str) = at {
        // Done time is explicitly set
        parse_date_string(at_str, Local::now(), Dialect::Us)
            .map_err(|_| color_eyre::eyre::eyre!("Invalid date string: {}", at_str))
    } else if let Some(took_str) = took {
        // Done time is start time plus duration
        let duration = parse_duration(took_str)?;
        Ok(*start_time + duration)
    } else {
        // Done time is now
        Ok(Local::now())
    }
}

fn calculate_times(
    back: Option<String>,
    at: Option<String>,
    took: Option<String>,
) -> Result<(DateTime<Local>, DateTime<Local>), color_eyre::eyre::Error> {
    let now = Local::now();

    if let Some(at_str) = at {
        // Done time is explicitly set
        let done_time = parse_date_string(&at_str, now, Dialect::Us)
            .map_err(|_| color_eyre::eyre::eyre!("Invalid date string: {}", at_str))?;

        let start_time = if let Some(took_str) = took {
            // Start time is done time minus duration
            let duration = parse_duration(&took_str)?;
            done_time - duration
        } else if let Some(back_str) = back {
            // Start time is explicitly set
            parse_date_string(&back_str, now, Dialect::Us)
                .map_err(|_| color_eyre::eyre::eyre!("Invalid date string: {}", back_str))?
        } else {
            // Start time same as done time
            done_time
        };

        Ok((start_time, done_time))
    } else if let Some(took_str) = took {
        // Done time is now, start time is now minus duration
        let duration = parse_duration(&took_str)?;
        let done_time = now;
        let start_time = if let Some(back_str) = back {
            // Start time is explicitly set
            parse_date_string(&back_str, now, Dialect::Us)
                .map_err(|_| color_eyre::eyre::eyre!("Invalid date string: {}", back_str))?
        } else {
            // Start time is done time minus duration
            done_time - duration
        };

        Ok((start_time, done_time))
    } else if let Some(back_str) = back {
        // Start time is explicitly set, done time is now
        let start_time = parse_date_string(&back_str, now, Dialect::Us)
            .map_err(|_| color_eyre::eyre::eyre!("Invalid date string: {}", back_str))?;
        Ok((start_time, now))
    } else {
        // Both times are now
        Ok((now, now))
    }
}

fn parse_duration(duration_str: &str) -> Result<Duration, color_eyre::eyre::Error> {
    // Try to parse as HH:MM
    if let Ok(re) = Regex::new(r"^(\d+):(\d+)$")
        && let Some(captures) = re.captures(duration_str)
    {
        let hours: i64 = captures[1].parse()?;
        let minutes: i64 = captures[2].parse()?;
        return Ok(Duration::hours(hours) + Duration::minutes(minutes));
    }

    // Try to parse compound durations like 2h30m
    let mut total_duration = Duration::zero();
    let duration_regex = Regex::new(r"(\d+)([dhms])")?;
    let mut matched = false;

    for capture in duration_regex.captures_iter(duration_str) {
        matched = true;
        let value: i64 = capture[1].parse()?;
        let unit = &capture[2];

        let unit_duration = match unit {
            "s" => Duration::seconds(value),
            "m" => Duration::minutes(value),
            "h" => Duration::hours(value),
            "d" => Duration::days(value),
            _ => return Err(color_eyre::eyre::eyre!("Invalid duration unit: {}", unit)),
        };

        total_duration += unit_duration;
    }

    if matched {
        Ok(total_duration)
    } else {
        Err(color_eyre::eyre::eyre!(
            "Invalid duration format: {}. Use XX[dhms] or HH:MM",
            duration_str
        ))
    }
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
