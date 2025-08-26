#[cfg(test)]
mod tests {
    use crate::models::{DoingFile, Entry};
    use crate::services::EntryService;
    use crate::storage::{Config, save_taskpaper};
    use crate::test_utils::utils::*;
    use chrono::Local;

    #[test]
    fn test_toggle_done_by_uuid() {
        let _ctx = TestContext::new().unwrap();

        // Create an entry
        let entry = Entry::new("Test entry for toggle".to_string(), "Currently".to_string());

        // Save it
        let config = Config::load();
        let mut doing_file = DoingFile::new(config.doing_file_path());
        doing_file
            .sections
            .entry("Currently".to_string())
            .or_default()
            .push(entry.clone());
        save_taskpaper(&doing_file).unwrap();

        // Toggle to done
        let updated = EntryService::toggle_done_by_uuid(&entry.uuid).unwrap();
        assert!(updated.is_done(), "Entry should be marked as done");

        // Toggle back to undone
        let updated2 = EntryService::toggle_done_by_uuid(&entry.uuid).unwrap();
        assert!(!updated2.is_done(), "Entry should not be marked as done");
    }

    #[test]
    fn test_delete_by_uuid() {
        let _ctx = TestContext::new().unwrap();

        // Create entries
        let entry1 = Entry::new("Entry to delete".to_string(), "Currently".to_string());
        let entry2 = Entry::new("Entry to keep".to_string(), "Currently".to_string());

        let config = Config::load();
        let mut doing_file = DoingFile::new(config.doing_file_path());
        doing_file
            .sections
            .entry("Currently".to_string())
            .or_default()
            .extend(vec![entry1.clone(), entry2.clone()]);
        save_taskpaper(&doing_file).unwrap();

        // Delete entry1
        EntryService::delete_by_uuid(&entry1.uuid).unwrap();

        // Verify it's gone
        let entries = EntryService::get_recent_entries(10).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].uuid, entry2.uuid);
    }

    #[test]
    fn test_get_recent_entries() {
        let _ctx = TestContext::new().unwrap();

        // Create entries with different timestamps
        let mut entries = vec![];
        for i in 0..5 {
            let mut entry = Entry::new(format!("Entry {i}"), "Currently".to_string());
            entry.timestamp = Local::now() - chrono::Duration::hours(i as i64);
            entries.push(entry);
        }

        let config = Config::load();
        let mut doing_file = DoingFile::new(config.doing_file_path());
        doing_file
            .sections
            .entry("Currently".to_string())
            .or_default()
            .extend(entries);
        save_taskpaper(&doing_file).unwrap();

        // Get recent 3
        let recent = EntryService::get_recent_entries(3).unwrap();
        assert_eq!(recent.len(), 3);

        // Should be sorted newest first
        assert!(recent[0].timestamp > recent[1].timestamp);
        assert!(recent[1].timestamp > recent[2].timestamp);
    }
}
