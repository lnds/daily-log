use crate::storage::{Config, parse_taskpaper};
use chrono::Local;

pub fn handle_last() -> color_eyre::Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();

    let doing_file = parse_taskpaper(&doing_file_path)?;

    if let Some(entry) = doing_file.get_last_entry() {
        let time_ago = Local::now().signed_duration_since(entry.timestamp);
        let time_str = format_duration(time_ago);

        println!(
            "{} - {} ({})",
            entry.timestamp.format("%Y-%m-%d %H:%M"),
            entry.description,
            time_str
        );

        if !entry.tags.is_empty() {
            let tags_str: Vec<String> = entry
                .tags
                .iter()
                .map(|(k, v)| {
                    if let Some(val) = v {
                        format!("@{k}({val})")
                    } else {
                        format!("@{k}")
                    }
                })
                .collect();
            println!("  Tags: {}", tags_str.join(" "));
        }

        if let Some(note) = &entry.note {
            println!("  Note: {note}");
        }
    } else {
        println!("No entries found");
    }

    Ok(())
}

fn format_duration(duration: chrono::Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;

    if hours > 24 {
        let days = hours / 24;
        format!("{days} days ago")
    } else if hours > 0 {
        format!("{} hours {} minutes ago", hours, minutes.abs())
    } else if minutes > 0 {
        format!("{minutes} minutes ago")
    } else {
        "just now".to_string()
    }
}
