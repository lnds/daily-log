use crate::storage::{Config, parse_taskpaper};

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

    let mut current_section = "";

    for entry in entries {
        if current_section != entry.section {
            println!("\n{}:", entry.section);
            current_section = &entry.section;
        }

        print!("  {} - ", entry.timestamp.format("%Y-%m-%d %H:%M"));

        if entry.is_done() {
            println!("✓ {}", entry.description);
        } else {
            println!("{}", entry.description);
        }

        if !entry.tags.is_empty() {
            let tags_str: Vec<String> = entry
                .tags
                .iter()
                .filter(|(k, _)| k.as_str() != "done")
                .map(|(k, v)| {
                    if let Some(val) = v {
                        format!("@{}({})", k, val)
                    } else {
                        format!("@{}", k)
                    }
                })
                .collect();
            if !tags_str.is_empty() {
                println!("      Tags: {}", tags_str.join(" "));
            }
        }

        if let Some(note) = &entry.note {
            for line in note.lines() {
                println!("      {}", line);
            }
        }
    }

    Ok(())
}
