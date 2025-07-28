use crate::storage::{Config, parse_taskpaper};
use chrono::{Local, Duration, TimeZone};

pub fn handle_recent(count: usize, section: Option<String>) -> color_eyre::Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    
    let doing_file = parse_taskpaper(&doing_file_path)?;
    
    let entries = if let Some(section_name) = section {
        doing_file
            .get_entries(&section_name)
            .map(|entries| entries.iter().collect::<Vec<_>>())
            .unwrap_or_default()
    } else {
        doing_file.get_recent_entries(count)
    };
    
    if entries.is_empty() {
        println!("No entries found");
        return Ok(());
    }
    
    let now = Local::now();
    let max_desc_width = 80;
    let section_width = 12;
    
    for entry in entries {
        // Format date/time
        let date_str = format_date(&entry.timestamp, &now);
        let time_str = entry.timestamp.format("%H:%M").to_string();
        
        // Format section
        let section_str = format!("[{:<width$}]", entry.section, width = section_width - 2);
        
        // Calculate duration if done with a timestamp
        let duration_str = if let Some(done_value) = entry.tags.get("done") {
            if let Some(done_time_str) = done_value {
                // Parse done timestamp
                if let Ok(done_time) = chrono::NaiveDateTime::parse_from_str(done_time_str, "%Y-%m-%d %H:%M") {
                    let done_local = Local.from_local_datetime(&done_time).single().unwrap_or_else(|| Local::now());
                    let duration = done_local.timestamp() - entry.timestamp.timestamp();
                    if duration > 0 {
                        format!(" {}", format_duration(Duration::seconds(duration)))
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        // Build description with tags
        let mut desc = entry.description.clone();
        for (tag, value) in &entry.tags {
            desc.push_str(&format!(" @{}", tag));
            if let Some(v) = value {
                desc.push_str(&format!("({})", v));
            }
        }
        
        // Truncate description if too long
        let desc = if desc.len() > max_desc_width {
            format!("{}...", &desc[..max_desc_width-3])
        } else {
            desc
        };
        
        // Print main line
        print!("{:>10} {:>5} ║ ", date_str, time_str);
        
        // Print description, section and duration
        if duration_str.is_empty() {
            println!("{:<width$} {}", desc, section_str, width = max_desc_width);
        } else {
            println!("{:<width$} {}{}", desc, section_str, duration_str, width = max_desc_width);
        }
        
        // Print notes if any
        if let Some(note) = &entry.note {
            for line in note.lines() {
                println!("{:>10} {:>5} ┃ {}", "", "", line);
            }
        }
    }
    
    Ok(())
}

fn format_date(timestamp: &chrono::DateTime<chrono::Local>, now: &chrono::DateTime<chrono::Local>) -> String {
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

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}