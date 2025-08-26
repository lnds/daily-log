use crate::models::Entry;
use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use chrono::Local;
use color_eyre::Result;
use color_eyre::eyre::eyre;
use uuid::Uuid;

pub struct EntryService;

impl EntryService {
    /// Toggle the @done status of an entry by its UUID
    /// Returns the updated entry if successful
    pub fn toggle_done_by_uuid(uuid: &Uuid) -> Result<Entry> {
        let config = Config::load();
        let doing_file_path = config.doing_file_path();

        let mut doing_file = parse_taskpaper(&doing_file_path)?;

        // Find and toggle the entry
        let mut found_entry = None;

        for (_section_name, entries) in doing_file.sections.iter_mut() {
            for entry in entries.iter_mut() {
                if &entry.uuid == uuid {
                    // Toggle the done status
                    if entry.is_done() {
                        entry.tags.remove("done");
                    } else {
                        let now = Local::now();
                        entry.tags.insert(
                            "done".to_string(),
                            Some(now.format("%Y-%m-%d %H:%M").to_string()),
                        );
                    }
                    found_entry = Some(entry.clone());
                    break;
                }
            }
            if found_entry.is_some() {
                break;
            }
        }

        if let Some(updated_entry) = found_entry {
            // Save the file
            save_taskpaper(&doing_file)?;
            Ok(updated_entry)
        } else {
            Err(eyre!("Entry with UUID {} not found", uuid))
        }
    }

    /// Update an entry's description by its UUID
    pub fn update_entry_description(uuid: &Uuid, new_description: String) -> Result<Entry> {
        let config = Config::load();
        let doing_file_path = config.doing_file_path();

        let mut doing_file = parse_taskpaper(&doing_file_path)?;

        // Find and update the entry
        let mut found_entry = None;

        for (_section_name, entries) in doing_file.sections.iter_mut() {
            for entry in entries.iter_mut() {
                if &entry.uuid == uuid {
                    entry.description = new_description.clone();
                    found_entry = Some(entry.clone());
                    break;
                }
            }
            if found_entry.is_some() {
                break;
            }
        }

        if let Some(updated_entry) = found_entry {
            // Save the file
            save_taskpaper(&doing_file)?;
            Ok(updated_entry)
        } else {
            Err(eyre!("Entry with UUID {} not found", uuid))
        }
    }

    /// Update an entry's note by its UUID
    pub fn update_entry_note(uuid: &Uuid, new_note: Option<String>) -> Result<Entry> {
        let config = Config::load();
        let doing_file_path = config.doing_file_path();

        let mut doing_file = parse_taskpaper(&doing_file_path)?;

        // Find and update the entry
        let mut found_entry = None;

        for (_section_name, entries) in doing_file.sections.iter_mut() {
            for entry in entries.iter_mut() {
                if &entry.uuid == uuid {
                    entry.note = new_note.clone();
                    found_entry = Some(entry.clone());
                    break;
                }
            }
            if found_entry.is_some() {
                break;
            }
        }

        if let Some(updated_entry) = found_entry {
            // Save the file
            save_taskpaper(&doing_file)?;
            Ok(updated_entry)
        } else {
            Err(eyre!("Entry with UUID {} not found", uuid))
        }
    }

    /// Delete an entry by its UUID
    pub fn delete_by_uuid(uuid: &Uuid) -> Result<()> {
        let config = Config::load();
        let doing_file_path = config.doing_file_path();

        let mut doing_file = parse_taskpaper(&doing_file_path)?;

        // Find and remove the entry
        let mut deleted = false;

        for (_section_name, entries) in doing_file.sections.iter_mut() {
            let initial_len = entries.len();
            entries.retain(|e| &e.uuid != uuid);
            if entries.len() < initial_len {
                deleted = true;
                break;
            }
        }

        if deleted {
            save_taskpaper(&doing_file)?;
            Ok(())
        } else {
            Err(eyre!("Entry with UUID {} not found", uuid))
        }
    }

    /// Get recent entries across all sections
    pub fn get_recent_entries(count: usize) -> Result<Vec<Entry>> {
        let config = Config::load();
        let doing_file_path = config.doing_file_path();

        let doing_file = parse_taskpaper(&doing_file_path)?;
        let all_entries = doing_file.get_all_entries();
        let mut owned_entries: Vec<Entry> = all_entries.into_iter().cloned().collect();

        // Sort by timestamp, newest first
        owned_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Take the requested count
        owned_entries.truncate(count);

        Ok(owned_entries)
    }

    /// Get entries for a specific section
    pub fn get_section_entries(section: &str) -> Result<Vec<Entry>> {
        let config = Config::load();
        let doing_file_path = config.doing_file_path();

        let doing_file = parse_taskpaper(&doing_file_path)?;

        Ok(doing_file
            .sections
            .get(section)
            .cloned()
            .unwrap_or_default())
    }

    /// Get entries for display in TUI, optionally filtered by section
    pub fn get_tui_entries(section_filter: Option<&str>, limit: usize) -> Result<Vec<Entry>> {
        let config = Config::load();
        let doing_file_path = config.doing_file_path();

        let doing_file = parse_taskpaper(&doing_file_path)?;

        let mut entries: Vec<Entry> = if let Some(section) = section_filter {
            // Get entries from specific section
            doing_file
                .sections
                .get(section)
                .cloned()
                .unwrap_or_default()
        } else {
            // Get all entries
            doing_file.get_all_entries().into_iter().cloned().collect()
        };

        // Sort by timestamp descending (most recent first)
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Limit to requested count
        entries.truncate(limit);

        Ok(entries)
    }
}
