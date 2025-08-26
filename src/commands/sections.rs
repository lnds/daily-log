use crate::cli::SectionsAction;
use crate::storage::{Config, DoingFile, parse_taskpaper, save_taskpaper};
use color_eyre::Result;
use std::collections::BTreeMap;

pub fn handle_sections(action: Option<SectionsAction>) -> Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let mut doing_file = parse_taskpaper(&doing_file_path)?;

    match action {
        None | Some(SectionsAction::List { .. }) => {
            list_sections(
                &doing_file,
                action
                    .as_ref()
                    .is_some_and(|a| matches!(a, SectionsAction::List { column: true, .. })),
            );
        }
        Some(SectionsAction::Add { section_name }) => {
            add_section(&mut doing_file, &section_name)?;
            save_taskpaper(&doing_file)?;
            println!("Added section: {section_name}");
        }
        Some(SectionsAction::Remove {
            section_name,
            archive,
        }) => {
            remove_section(&mut doing_file, &section_name, archive)?;
            save_taskpaper(&doing_file)?;
            println!("Removed section: {section_name}");
        }
    }

    Ok(())
}

fn list_sections(doing_file: &DoingFile, column: bool) {
    // Use BTreeMap to maintain alphabetical order
    let sections: BTreeMap<&String, usize> = doing_file
        .sections
        .iter()
        .map(|(name, entries)| (name, entries.len()))
        .collect();

    if column {
        // Display in column format
        for name in sections.keys() {
            println!("{name}");
        }
    } else {
        // Display with entry counts
        let max_name_len = sections.keys().map(|name| name.len()).max().unwrap_or(0);

        for (name, count) in &sections {
            println!(
                "{:width$} ({} {})",
                name,
                count,
                if *count == 1 { "entry" } else { "entries" },
                width = max_name_len
            );
        }
    }
}

fn add_section(doing_file: &mut DoingFile, section_name: &str) -> Result<()> {
    if doing_file.sections.contains_key(section_name) {
        return Err(color_eyre::eyre::eyre!(
            "Section '{}' already exists",
            section_name
        ));
    }

    doing_file
        .sections
        .insert(section_name.to_string(), Vec::new());
    Ok(())
}

fn remove_section(doing_file: &mut DoingFile, section_name: &str, archive: bool) -> Result<()> {
    if !doing_file.sections.contains_key(section_name) {
        return Err(color_eyre::eyre::eyre!(
            "Section '{}' does not exist",
            section_name
        ));
    }

    // Don't allow removing the default section
    if section_name == "Currently" {
        return Err(color_eyre::eyre::eyre!(
            "Cannot remove the 'Currently' section"
        ));
    }

    if archive {
        // Move entries to Archive section if requested
        if let Some(entries) = doing_file.sections.remove(section_name)
            && !entries.is_empty()
        {
            let archive_entries = doing_file
                .sections
                .entry("Archive".to_string())
                .or_default();

            // Add entries to the beginning of Archive section
            for entry in entries.into_iter().rev() {
                archive_entries.insert(0, entry);
            }
        }
    } else {
        // Just remove the section
        doing_file.sections.remove(section_name);
    }

    Ok(())
}
