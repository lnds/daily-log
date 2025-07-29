use crate::storage::{Config, parse_taskpaper};
use chrono::{Local, Timelike};

pub fn handle_today(section: Option<String>) -> color_eyre::Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();

    let doing_file = parse_taskpaper(&doing_file_path)?;

    let today_start = Local::now()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();

    let mut entries = doing_file.get_entries_since(today_start);

    if let Some(section_name) = section {
        entries.retain(|entry| entry.section == section_name);
    }

    if entries.is_empty() {
        println!("No entries for today");
        return Ok(());
    }

    println!("Today's entries:");

    let mut current_section = "";

    for entry in entries {
        if current_section != entry.section {
            println!("\n{}:", entry.section);
            current_section = &entry.section;
        }

        print!("  {} - ", entry.timestamp.format("%H:%M"));

        if entry.is_done() {
            println!("✓ {}", entry.description);
        } else {
            println!("{}", entry.description);
        }

        if let Some(note) = &entry.note {
            for line in note.lines() {
                println!("      {line}");
            }
        }
    }

    Ok(())
}
