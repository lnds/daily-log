use crate::models::{DoingFile, Entry};
use chrono::{DateTime, Local, NaiveTime};
use chrono_english::{Dialect, parse_date_string};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct FilterOptions {
    pub search: Option<String>,
    pub tags: Vec<String>,
    pub sections: Vec<String>,
    pub after: Option<DateTime<Local>>,
    pub before: Option<DateTime<Local>>,
    pub from: Option<(DateTime<Local>, Option<DateTime<Local>>)>,
    pub case: CaseSensitivity,
    pub exact: bool,
    pub not: bool,
    pub bool_op: BoolOp,
    pub only_timed: bool,
    pub val: Vec<String>,
}

impl Default for FilterOptions {
    fn default() -> Self {
        Self {
            search: None,
            tags: vec![],
            sections: vec![],
            after: None,
            before: None,
            from: None,
            case: CaseSensitivity::Smart,
            exact: false,
            not: false,
            bool_op: BoolOp::Pattern,
            only_timed: false,
            val: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CaseSensitivity {
    CaseSensitive,
    Ignore,
    Smart,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BoolOp {
    And,
    Or,
    Not,
    Pattern,
}

pub fn filter_entries(
    doing_file: &DoingFile,
    options: &FilterOptions,
) -> color_eyre::Result<Vec<(String, Entry)>> {
    let mut entries: Vec<(String, Entry)> = Vec::new();

    // Determine which sections to include
    let target_sections: Vec<&String> = if options.sections.is_empty() {
        doing_file.sections.keys().collect()
    } else {
        options
            .sections
            .iter()
            .filter(|s| doing_file.sections.contains_key(*s))
            .collect()
    };

    // Collect entries from target sections
    for section in &target_sections {
        if let Some(section_entries) = doing_file.sections.get(*section) {
            for entry in section_entries {
                entries.push(((*section).clone(), entry.clone()));
            }
        }
    }

    // Apply filters
    let mut filtered = entries;

    // Time-based filters
    if let Some(after) = &options.after {
        filtered = filter_by_after(filtered, after);
    }

    if let Some(before) = &options.before {
        filtered = filter_by_before(filtered, before);
    }

    if let Some((from, to)) = &options.from {
        filtered = filter_by_range(filtered, from, to.as_ref());
    }

    // Search filter
    if let Some(search_query) = &options.search {
        filtered = filter_by_search(filtered, search_query, options.exact, &options.case)?;
    }

    // Tag filter
    if !options.tags.is_empty() {
        filtered = filter_by_tags(filtered, &options.tags, &options.bool_op)?;
    }

    // Only timed filter
    if options.only_timed {
        filtered.retain(|(_, entry)| entry.is_done());
    }

    // Value queries
    if !options.val.is_empty() {
        filtered = filter_by_value_queries(filtered, &options.val, &options.bool_op)?;
    }

    // Apply NOT filter if specified
    if options.not {
        let filtered_uuids: HashSet<_> = filtered.iter().map(|(_, e)| e.uuid).collect();

        // Re-collect all entries
        let mut all_entries: Vec<(String, Entry)> = Vec::new();
        for section in &target_sections {
            if let Some(section_entries) = doing_file.sections.get(*section) {
                for entry in section_entries {
                    all_entries.push(((*section).clone(), entry.clone()));
                }
            }
        }

        filtered = all_entries
            .into_iter()
            .filter(|(_, entry)| !filtered_uuids.contains(&entry.uuid))
            .collect();
    }

    Ok(filtered)
}

fn filter_by_after(entries: Vec<(String, Entry)>, after: &DateTime<Local>) -> Vec<(String, Entry)> {
    // If after only has time component, filter by time of day
    if after.date_naive() == Local::now().date_naive() {
        let time = after.time();
        entries
            .into_iter()
            .filter(|(_, entry)| entry.timestamp.time() >= time)
            .collect()
    } else {
        entries
            .into_iter()
            .filter(|(_, entry)| entry.timestamp >= *after)
            .collect()
    }
}

fn filter_by_before(
    entries: Vec<(String, Entry)>,
    before: &DateTime<Local>,
) -> Vec<(String, Entry)> {
    // If before only has time component, filter by time of day
    if before.date_naive() == Local::now().date_naive() {
        let time = before.time();
        entries
            .into_iter()
            .filter(|(_, entry)| entry.timestamp.time() <= time)
            .collect()
    } else {
        entries
            .into_iter()
            .filter(|(_, entry)| entry.timestamp <= *before)
            .collect()
    }
}

fn filter_by_range(
    entries: Vec<(String, Entry)>,
    from: &DateTime<Local>,
    to: Option<&DateTime<Local>>,
) -> Vec<(String, Entry)> {
    if let Some(to) = to {
        entries
            .into_iter()
            .filter(|(_, entry)| entry.timestamp >= *from && entry.timestamp <= *to)
            .collect()
    } else {
        entries
            .into_iter()
            .filter(|(_, entry)| entry.timestamp >= *from)
            .collect()
    }
}

fn filter_by_search(
    entries: Vec<(String, Entry)>,
    search_query: &str,
    exact: bool,
    case: &CaseSensitivity,
) -> Result<Vec<(String, Entry)>, color_eyre::eyre::Error> {
    // Determine case sensitivity
    let case_sensitive = match case {
        CaseSensitivity::CaseSensitive => true,
        CaseSensitivity::Ignore => false,
        CaseSensitivity::Smart => search_query.chars().any(|c| c.is_uppercase()),
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
            .filter(|(_, entry)| {
                regex.is_match(&entry.description)
                    || entry.note.as_ref().is_some_and(|n| regex.is_match(n))
            })
            .collect()
    } else if search_query.starts_with('\'') || exact {
        // Exact match
        let query = if let Some(stripped) = search_query.strip_prefix('\'') {
            stripped
        } else {
            search_query
        };
        entries
            .into_iter()
            .filter(|(_, entry)| {
                let desc_match = if case_sensitive {
                    entry.description == query
                } else {
                    entry.description.eq_ignore_ascii_case(query)
                };
                let note_match = entry.note.as_ref().is_some_and(|n| {
                    if case_sensitive {
                        n == query
                    } else {
                        n.eq_ignore_ascii_case(query)
                    }
                });
                desc_match || note_match
            })
            .collect()
    } else {
        // Fuzzy/substring matching
        entries
            .into_iter()
            .filter(|(_, entry)| {
                let desc_match = if case_sensitive {
                    entry.description.contains(search_query)
                } else {
                    entry
                        .description
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
                };
                let note_match = entry.note.as_ref().is_some_and(|n| {
                    if case_sensitive {
                        n.contains(search_query)
                    } else {
                        n.to_lowercase().contains(&search_query.to_lowercase())
                    }
                });
                desc_match || note_match
            })
            .collect()
    };

    Ok(filtered)
}

fn filter_by_tags(
    entries: Vec<(String, Entry)>,
    tags: &[String],
    bool_op: &BoolOp,
) -> Result<Vec<(String, Entry)>, color_eyre::eyre::Error> {
    let filtered = match bool_op {
        BoolOp::Pattern => {
            // Handle +tag (required) and -tag (excluded) syntax
            let mut required_tags = Vec::new();
            let mut excluded_tags = Vec::new();
            let mut normal_tags = Vec::new();

            for tag in tags {
                if tag.starts_with("+@") || tag.starts_with('+') {
                    let tag_name = tag.trim_start_matches('+').trim_start_matches('@');
                    required_tags.push(tag_name);
                } else if tag.starts_with("-@") || tag.starts_with('-') {
                    let tag_name = tag.trim_start_matches('-').trim_start_matches('@');
                    excluded_tags.push(tag_name);
                } else {
                    let tag_name = tag.trim_start_matches('@');
                    normal_tags.push(tag_name);
                }
            }

            entries
                .into_iter()
                .filter(|(_, entry)| {
                    // Must have all required tags
                    let has_required = required_tags.iter().all(|tag| has_tag(entry, tag));

                    // Must not have any excluded tags
                    let no_excluded = excluded_tags.iter().all(|tag| !has_tag(entry, tag));

                    // Must have at least one normal tag (if any specified)
                    let has_normal =
                        normal_tags.is_empty() || normal_tags.iter().any(|tag| has_tag(entry, tag));

                    has_required && no_excluded && has_normal
                })
                .collect()
        }
        BoolOp::And => {
            // Must have all tags
            entries
                .into_iter()
                .filter(|(_, entry)| {
                    tags.iter().all(|tag| {
                        let tag_name = tag.trim_start_matches('@');
                        has_tag(entry, tag_name)
                    })
                })
                .collect()
        }
        BoolOp::Or => {
            // Must have at least one tag
            entries
                .into_iter()
                .filter(|(_, entry)| {
                    tags.iter().any(|tag| {
                        let tag_name = tag.trim_start_matches('@');
                        has_tag(entry, tag_name)
                    })
                })
                .collect()
        }
        BoolOp::Not => {
            // Must not have any of the tags
            entries
                .into_iter()
                .filter(|(_, entry)| {
                    !tags.iter().any(|tag| {
                        let tag_name = tag.trim_start_matches('@');
                        has_tag(entry, tag_name)
                    })
                })
                .collect()
        }
    };

    Ok(filtered)
}

fn has_tag(entry: &Entry, tag_pattern: &str) -> bool {
    if tag_pattern.contains('*') || tag_pattern.contains('?') {
        // Wildcard matching
        let pattern = tag_pattern.replace('*', ".*").replace('?', ".");
        if let Ok(regex) = Regex::new(&format!("^{pattern}$")) {
            entry.tags.keys().any(|tag| regex.is_match(tag))
        } else {
            false
        }
    } else {
        entry.tags.contains_key(tag_pattern)
    }
}

fn filter_by_value_queries(
    entries: Vec<(String, Entry)>,
    _queries: &[String],
    _bool_op: &BoolOp,
) -> Result<Vec<(String, Entry)>, color_eyre::eyre::Error> {
    // Parse value queries like "@done > 2 hours ago"
    // This is a simplified implementation
    // In a full implementation, we'd parse and evaluate these queries properly
    Ok(entries)
}

pub fn parse_date_filter(date_str: &str) -> color_eyre::Result<DateTime<Local>> {
    // Try to parse as time only (e.g., "8am", "15:00")
    if let Ok(time) = NaiveTime::parse_from_str(date_str, "%H:%M") {
        let today = Local::now().date_naive();
        return Ok(today.and_time(time).and_local_timezone(Local).unwrap());
    }

    // Try common time formats
    let time_patterns = [
        (r"^(\d{1,2})\s*am$", false),
        (r"^(\d{1,2})\s*pm$", true),
        (r"^(\d{1,2}):(\d{2})\s*am$", false),
        (r"^(\d{1,2}):(\d{2})\s*pm$", true),
    ];

    for (pattern, is_pm) in &time_patterns {
        let re = Regex::new(pattern)?;
        if let Some(caps) = re.captures(date_str.to_lowercase().as_str()) {
            let hour = caps[1].parse::<u32>()?;
            let minute = if caps.len() > 2 {
                caps[2].parse::<u32>()?
            } else {
                0
            };

            let hour = if *is_pm && hour != 12 {
                hour + 12
            } else if !is_pm && hour == 12 {
                0
            } else {
                hour
            };

            let today = Local::now().date_naive();
            let time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
            return Ok(today.and_time(time).and_local_timezone(Local).unwrap());
        }
    }

    // Fall back to natural language parsing
    parse_date_string(date_str, Local::now(), Dialect::Us)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to parse date: {}", e))
}

pub fn parse_date_range(
    range_str: &str,
) -> color_eyre::Result<(DateTime<Local>, Option<DateTime<Local>>)> {
    // Split on " to ", " through ", or " - "
    let separators = [" to ", " through ", " - "];

    for sep in &separators {
        if let Some(pos) = range_str.find(sep) {
            let start_str = &range_str[..pos];
            let end_str = &range_str[pos + sep.len()..];

            let start = parse_date_filter(start_str)?;
            let end = parse_date_filter(end_str)?;

            return Ok((start, Some(end)));
        }
    }

    // No range separator found, parse as single date
    let date = parse_date_filter(range_str)?;
    Ok((date, None))
}
