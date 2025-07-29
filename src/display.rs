use crate::models::Entry;
use chrono::{DateTime, Local};
use std::collections::HashMap;

pub struct DisplayOptions {
    pub times: bool,
    pub duration: bool,
    pub totals: bool,
    pub hilite: bool,
    pub search_query: Option<String>,
    pub output_format: OutputFormat,
    pub tag_sort: TagSort,
    pub tag_order: SortOrder,
    pub section_filter: Vec<String>,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            times: true,
            duration: false,
            totals: false,
            hilite: false,
            search_query: None,
            output_format: OutputFormat::Default,
            tag_sort: TagSort::Name,
            tag_order: SortOrder::Asc,
            section_filter: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Default,
    Json,
    Csv,
    Markdown,
    Html,
    TaskPaper,
    Timeline,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagSort {
    Name,
    Time,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

pub fn display_entries(entries: &[(String, Entry)], options: &DisplayOptions) -> color_eyre::Result<()> {
    match options.output_format {
        OutputFormat::Default => display_default(entries, options),
        OutputFormat::Json => display_json(entries),
        OutputFormat::Csv => display_csv(entries),
        OutputFormat::Markdown => display_markdown(entries, options),
        OutputFormat::Html => display_html(entries, options),
        OutputFormat::TaskPaper => display_taskpaper(entries),
        OutputFormat::Timeline => display_timeline(entries, options),
    }
}

fn display_default(entries: &[(String, Entry)], options: &DisplayOptions) -> color_eyre::Result<()> {
    if entries.is_empty() {
        println!("No entries found");
        return Ok(());
    }

    let now = Local::now();
    let mut total_duration = chrono::Duration::zero();

    // Calculate maximum width for description
    let max_desc_width = 50;

    for (section, entry) in entries.iter() {

        // Format date
        let date_str = format_date(&entry.timestamp, &now);
        let time_str = entry.timestamp.format("%H:%M").to_string();

        // Calculate duration if done
        let (duration_str, duration) = if options.times && entry.is_done() {
            calculate_duration(entry)
        } else if options.duration && !entry.is_done() {
            let duration = now - entry.timestamp;
            let duration_str = format!(" ({})", format_duration(&duration));
            (duration_str, Some(duration))
        } else {
            (String::new(), None)
        };

        if let Some(d) = duration {
            total_duration += d;
        }

        // Build section string
        let section_str = if section != "Currently" {
            format!("[{section}]")
        } else {
            String::new()
        };

        // Build description with tags
        let mut desc = entry.description.clone();
        let tags = format_tags(&entry.tags, options);
        if !tags.is_empty() {
            desc.push(' ');
            desc.push_str(&tags);
        }

        // Highlight search matches if requested
        if options.hilite && options.search_query.is_some() {
            desc = highlight_matches(&desc, options.search_query.as_ref().unwrap());
        }

        // Truncate description if too long
        let desc = if desc.len() > max_desc_width {
            format!("{}...", &desc[..max_desc_width-3])
        } else {
            desc
        };

        // Print main line
        print!("{date_str:>10} {time_str:>5} ║ ");

        // Print description, section and duration
        if duration_str.is_empty() {
            println!("{desc:<max_desc_width$} {section_str}");
        } else {
            println!("{desc:<max_desc_width$} {section_str}{duration_str}");
        }

        // Print notes if any
        if let Some(note) = &entry.note {
            for line in note.lines() {
                println!("{:>10} {:>5} ┃ {}", "", "", line);
            }
        }
    }

    // Show totals if requested
    if options.totals && !total_duration.is_zero() {
        println!("\n{:>10} {:>5} ═══════════════════════════════════════════════════════════", "", "");
        println!("{:>10} {:>5} Total: {}", "", "", format_duration(&total_duration));
    }

    Ok(())
}

fn display_json(entries: &[(String, Entry)]) -> color_eyre::Result<()> {
    let json_entries: Vec<serde_json::Value> = entries.iter()
        .map(|(section, entry)| {
            serde_json::json!({
                "section": section,
                "timestamp": entry.timestamp.to_rfc3339(),
                "description": entry.description,
                "tags": entry.tags,
                "note": entry.note,
                "uuid": entry.uuid.to_string(),
            })
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&json_entries)?);
    Ok(())
}

fn display_csv(entries: &[(String, Entry)]) -> color_eyre::Result<()> {
    // CSV header
    println!("timestamp,description,section,tags,note,uuid");

    for (section, entry) in entries {
        let tags_str = entry.tags.iter()
            .map(|(k, v)| {
                if let Some(val) = v {
                    format!("@{k}({val})")
                } else {
                    format!("@{k}")
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        let note_str = entry.note.as_deref().unwrap_or("");

        println!(
            "{},{},{},{},{},{}",
            entry.timestamp.to_rfc3339(),
            escape_csv(&entry.description),
            escape_csv(section),
            escape_csv(&tags_str),
            escape_csv(note_str),
            entry.uuid
        );
    }

    Ok(())
}

fn display_markdown(entries: &[(String, Entry)], options: &DisplayOptions) -> color_eyre::Result<()> {
    println!("# Doing Entries\n");

    let mut current_date = None;

    for (section, entry) in entries {
        let entry_date = entry.timestamp.date_naive();

        // Add date header if changed
        if current_date != Some(entry_date) {
            println!("\n## {}\n", entry_date.format("%A, %B %d, %Y"));
            current_date = Some(entry_date);
        }

        // Time and description
        print!("- **{}** - {}", entry.timestamp.format("%H:%M"), entry.description);

        // Tags
        let tags = format_tags(&entry.tags, options);
        if !tags.is_empty() {
            print!(" {tags}");
        }

        // Section
        if section != "Currently" {
            print!(" _[{section}]_");
        }

        println!();

        // Note
        if let Some(note) = &entry.note {
            for line in note.lines() {
                println!("  {line}");
            }
        }
    }

    Ok(())
}

fn display_html(entries: &[(String, Entry)], options: &DisplayOptions) -> color_eyre::Result<()> {
    println!(r#"<!DOCTYPE html>
<html>
<head>
    <title>Doing Entries</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }}
        .entry {{ margin: 10px 0; padding: 10px; border-left: 3px solid #007acc; }}
        .timestamp {{ font-weight: bold; color: #666; }}
        .tags {{ color: #007acc; }}
        .section {{ color: #999; font-style: italic; }}
        .note {{ margin-left: 20px; color: #666; font-style: italic; }}
    </style>
</head>
<body>
    <h1>Doing Entries</h1>"#);

    for (section, entry) in entries {
        println!(r#"    <div class="entry">"#);
        print!(r#"        <span class="timestamp">{}</span> - {}"#, 
            entry.timestamp.format("%Y-%m-%d %H:%M"), 
            html_escape(&entry.description)
        );

        let tags = format_tags(&entry.tags, options);
        if !tags.is_empty() {
            print!(r#" <span class="tags">{}</span>"#, html_escape(&tags));
        }

        if section != "Currently" {
            print!(r#" <span class="section">[{}]</span>"#, html_escape(section));
        }

        if let Some(note) = &entry.note {
            println!(r#"        <div class="note">{}</div>"#, html_escape(note));
        }

        println!(r#"    </div>"#);
    }

    println!("</body>\n</html>");
    Ok(())
}

fn display_taskpaper(entries: &[(String, Entry)]) -> color_eyre::Result<()> {
    let mut sections: HashMap<String, Vec<&Entry>> = HashMap::new();

    // Group by section
    for (section, entry) in entries {
        sections.entry(section.clone()).or_default().push(entry);
    }

    // Output each section
    for (section, entries) in sections {
        println!("{section}:");
        for entry in entries {
            print!("- {} | {}", 
                entry.timestamp.format("%Y-%m-%d %H:%M"), 
                entry.description
            );

            // Add tags
            for (tag, value) in &entry.tags {
                print!(" @{tag}");
                if let Some(v) = value {
                    print!("({v})");
                }
            }

            println!(" <{}>", entry.uuid);

            // Add note
            if let Some(note) = &entry.note {
                for line in note.lines() {
                    println!("  {line}");
                }
            }
        }
        println!();
    }

    Ok(())
}

fn display_timeline(entries: &[(String, Entry)], options: &DisplayOptions) -> color_eyre::Result<()> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut current_date = None;

    for (section, entry) in entries {
        let entry_date = entry.timestamp.date_naive();

        // Add date separator if changed
        if current_date != Some(entry_date) {
            println!("\n══════════════════ {} ══════════════════", 
                entry_date.format("%A, %B %d, %Y"));
            current_date = Some(entry_date);
        }

        // Time
        print!("{} ", entry.timestamp.format("%H:%M"));

        // Duration if done
        if options.times && entry.is_done() {
            let (duration_str, _) = calculate_duration(entry);
            if !duration_str.is_empty() {
                print!("{} ", duration_str.trim());
            }
        }

        // Description and tags
        print!("│ {}", entry.description);
        let tags = format_tags(&entry.tags, options);
        if !tags.is_empty() {
            print!(" {tags}");
        }

        // Section
        if section != "Currently" {
            print!(" [{section}]");
        }

        println!();

        // Note
        if let Some(note) = &entry.note {
            for line in note.lines() {
                println!("      │   {line}");
            }
        }
    }

    Ok(())
}

fn format_date(timestamp: &DateTime<Local>, now: &DateTime<Local>) -> String {
    let days_diff = (now.date_naive() - timestamp.date_naive()).num_days();

    if days_diff == 0 {
        "Today".to_string()
    } else if days_diff == 1 {
        "Yesterday".to_string()
    } else if days_diff < 7 {
        // This week - show day name
        timestamp.format("%a").to_string()
    } else {
        // Older - show month/day
        timestamp.format("%m/%d").to_string()
    }
}

fn calculate_duration(entry: &Entry) -> (String, Option<chrono::Duration>) {
    if let Some(Some(done_str)) = entry.tags.get("done") {
        if let Ok(done_time) = chrono::DateTime::parse_from_str(
            &format!("{done_str} +0000"),
            "%Y-%m-%d %H:%M %z",
        ) {
            let duration = done_time.with_timezone(&Local) - entry.timestamp;
            let duration_str = format!(" ({})", format_duration(&duration));
            return (duration_str, Some(duration));
        }
    }
    (String::new(), None)
}

fn format_duration(duration: &chrono::Duration) -> String {
    let total_seconds = duration.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{hours}h{minutes}m")
    } else if minutes > 0 {
        format!("{minutes}m{seconds}s")
    } else {
        format!("{seconds}s")
    }
}

fn format_tags(tags: &HashMap<String, Option<String>>, options: &DisplayOptions) -> String {
    let mut tag_vec: Vec<(String, Option<String>)> = tags.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    // Sort tags
    match options.tag_sort {
        TagSort::Name => {
            tag_vec.sort_by(|a, b| match options.tag_order {
                SortOrder::Asc => a.0.cmp(&b.0),
                SortOrder::Desc => b.0.cmp(&a.0),
            });
        }
        TagSort::Time => {
            // For time sorting, we'd need to parse date values in tags
            // For now, fall back to name sorting
            tag_vec.sort_by(|a, b| match options.tag_order {
                SortOrder::Asc => a.0.cmp(&b.0),
                SortOrder::Desc => b.0.cmp(&a.0),
            });
        }
    }

    tag_vec.iter()
        .map(|(tag, value)| {
            if let Some(v) = value {
                format!("@{tag}({v})")
            } else {
                format!("@{tag}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn highlight_matches(text: &str, query: &str) -> String {
    // Simple highlighting with ANSI codes
    // In a real implementation, this would handle regex and case sensitivity
    text.replace(query, &format!("\x1b[33m{query}\x1b[0m"))
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}