use crate::storage::{Config, parse_taskpaper};
use color_eyre::Result;
use regex::Regex;
use std::collections::HashMap;

pub fn handle_tags(
    max_count: Option<usize>,
    _bool_op: String,
    counts: bool,
    case: String,
    interactive: bool,
    line: bool,
    not: bool,
    order: String,
    section: Vec<String>,
    search: Option<String>,
    sort: String,
    tag: Option<String>,
    val: Vec<String>,
    exact: bool,
) -> Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let doing_file = parse_taskpaper(&doing_file_path)?;

    // Compile search patterns
    let search_regex = if let Some(ref pattern) = search {
        Some(compile_search_regex(pattern, &case, exact)?)
    } else {
        None
    };

    let tag_regex = if let Some(ref tag_pattern) = tag {
        Some(compile_tag_regex(tag_pattern)?)
    } else {
        None
    };

    let tag_value_queries = compile_tag_value_queries(&val)?;

    // Determine which sections to process
    let sections_to_process: Vec<&String> = if section.is_empty() {
        doing_file.sections.keys().collect()
    } else {
        section.iter()
            .filter(|s| doing_file.sections.contains_key(*s))
            .collect()
    };

    // Collect tags with their counts
    let mut tag_counts: HashMap<String, usize> = HashMap::new();

    for section_name in sections_to_process {
        if let Some(entries) = doing_file.sections.get(section_name) {
            for entry in entries {
                let mut matches = true;

                // Apply search filter
                if let Some(ref regex) = search_regex {
                    if !regex.is_match(&entry.description) && 
                       !entry.note.as_ref().map_or(false, |n| regex.is_match(n)) {
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
                if not {
                    matches = !matches;
                }

                if matches {
                    // Count all tags in this entry
                    for (tag_name, _) in &entry.tags {
                        *tag_counts.entry(tag_name.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    if tag_counts.is_empty() {
        println!("No tags found");
        return Ok(());
    }

    // Sort tags
    let mut sorted_tags: Vec<(String, usize)> = tag_counts.into_iter().collect();
    
    match sort.as_str() {
        "name" => {
            sorted_tags.sort_by(|a, b| a.0.cmp(&b.0));
        }
        "count" | "time" => {
            sorted_tags.sort_by(|a, b| b.1.cmp(&a.1));
        }
        _ => {
            sorted_tags.sort_by(|a, b| a.0.cmp(&b.0));
        }
    }

    // Apply order (asc/desc)
    if order == "desc" {
        sorted_tags.reverse();
    }

    // Apply max count
    if let Some(max) = max_count {
        sorted_tags.truncate(max);
    }

    // Display tags
    if interactive {
        // Interactive mode would require a menu system - simplified for now
        println!("Interactive mode not yet implemented");
        return Ok(());
    }

    if line {
        // Display all tags on one line
        let tag_list: Vec<String> = if counts {
            sorted_tags.iter()
                .map(|(tag, count)| format!("@{}({})", tag, count))
                .collect()
        } else {
            sorted_tags.iter()
                .map(|(tag, _)| format!("@{}", tag))
                .collect()
        };
        println!("{}", tag_list.join(" "));
    } else {
        // Display one tag per line
        let max_tag_len = sorted_tags.iter()
            .map(|(tag, _)| tag.len())
            .max()
            .unwrap_or(0);

        for (tag, count) in sorted_tags {
            if counts {
                println!("{:width$} ({})", tag, count, width = max_tag_len);
            } else {
                println!("{}", tag);
            }
        }
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
        Regex::new(&format!("(?i){}", pattern))?
    } else {
        Regex::new(&pattern)?
    };

    Ok(regex)
}

fn compile_tag_regex(pattern: &str) -> Result<Regex> {
    let pattern = pattern.trim_start_matches('@');
    Ok(Regex::new(&format!("^{}$", pattern))?)
}

fn parse_smart_case(case: &str, pattern: &str) -> bool {
    match case {
        "i" | "ignore" => true,
        "c" | "case-sensitive" => false,
        "s" | "smart" | _ => {
            // Smart case: case-insensitive unless pattern contains uppercase
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