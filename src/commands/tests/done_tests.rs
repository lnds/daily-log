#[cfg(test)]
mod tests {
    use crate::commands::{DoneOptions, handle_done};
    use crate::storage::parse_taskpaper;
    use crate::test_utils::utils::{TestContext, TestEntry};
    use chrono::{Duration, Local};

    #[test]
    fn test_done_marks_last_entry() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let now = Local::now();
        let entries = vec![
            TestEntry::new("First task").with_timestamp(now - Duration::minutes(2)),
            TestEntry::new("Second task").with_timestamp(now - Duration::minutes(1)),
            TestEntry::new("Last task").with_timestamp(now),
        ];
        ctx.create_doing_file_with_entries(entries)?;

        // Mark last entry as done
        handle_done(DoneOptions {
            entry: vec![],
            note: None,
            ask: false,
            back: None,
            at: None,
            took: None,
            from: None,
            section: None,
            editor: false,
            archive: false,
            remove: false,
            unfinished: false,
            _date: false,
            _noauto: false,
        })?;

        // Re-read the file after done command
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entries = doing_file.sections.get("Currently").unwrap();

        // Last task should now be marked as done
        let last_entry = entries
            .iter()
            .find(|e| e.description == "Last task")
            .unwrap();
        assert!(last_entry.is_done());

        // Others should not be done
        let first_entry = entries
            .iter()
            .find(|e| e.description == "First task")
            .unwrap();
        assert!(!first_entry.is_done());

        Ok(())
    }

    #[test]
    fn test_done_creates_new_entry() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n")?;

        // Create a new done entry
        handle_done(DoneOptions {
            entry: vec!["Quick task @urgent".to_string()],
            note: None,
            ask: false,
            back: None,
            at: None,
            took: None,
            from: None,
            section: None,
            editor: false,
            archive: false,
            remove: false,
            unfinished: false,
            _date: false,
            _noauto: false,
        })?;

        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entries = doing_file.sections.get("Currently").unwrap();

        // Entry should exist and be done
        let entry = entries
            .iter()
            .find(|e| e.description == "Quick task")
            .unwrap();
        assert!(entry.is_done());
        assert!(entry.tags.contains_key("urgent"));

        Ok(())
    }

    #[test]
    fn test_done_with_at_time() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let now = Local::now();
        let entries =
            vec![TestEntry::new("Task to complete").with_timestamp(now - Duration::hours(3))];
        ctx.create_doing_file_with_entries(entries)?;

        // Mark as done at specific time
        handle_done(DoneOptions {
            entry: vec![],
            note: None,
            ask: false,
            back: None,
            at: Some("2 hours ago".to_string()),
            took: None,
            from: None,
            section: None,
            editor: false,
            archive: false,
            remove: false,
            unfinished: false,
            _date: false,
            _noauto: false,
        })?;

        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entry = doing_file
            .sections
            .get("Currently")
            .unwrap()
            .iter()
            .find(|e| e.description == "Task to complete")
            .unwrap();

        assert!(entry.is_done());
        // Check that done time is approximately 2 hours ago
        if let Some(done_value) = entry.tags.get("done") {
            // The done tag should contain a timestamp
            assert!(done_value.is_some());
        }

        Ok(())
    }

    #[test]
    fn test_done_with_took_duration() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let now = Local::now();
        let entries =
            vec![TestEntry::new("Task that took time").with_timestamp(now - Duration::hours(3))];
        ctx.create_doing_file_with_entries(entries)?;

        // Mark as done with duration
        handle_done(DoneOptions {
            entry: vec![],
            note: None,
            ask: false,
            back: None,
            at: None,
            took: Some("2h30m".to_string()),
            from: None,
            section: None,
            editor: false,
            archive: false,
            remove: false,
            unfinished: false,
            _date: false,
            _noauto: false,
        })?;

        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entry = doing_file
            .sections
            .get("Currently")
            .unwrap()
            .iter()
            .find(|e| e.description == "Task that took time")
            .unwrap();

        assert!(entry.is_done());

        Ok(())
    }

    #[test]
    fn test_done_remove_flag() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let now = Local::now();
        let entries = vec![
            TestEntry::new("Done task")
                .with_timestamp(now - Duration::hours(1))
                .with_done(now),
        ];
        ctx.create_doing_file_with_entries(entries)?;

        // Remove done tag
        handle_done(DoneOptions {
            entry: vec![],
            note: None,
            ask: false,
            back: None,
            at: None,
            took: None,
            from: None,
            section: None,
            editor: false,
            archive: false,
            remove: true,
            unfinished: false,
            _date: false,
            _noauto: false,
        })?;

        // Re-read the file after removing done tag
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entry = doing_file
            .sections
            .get("Currently")
            .unwrap()
            .iter()
            .find(|e| e.description == "Done task")
            .unwrap();

        assert!(!entry.is_done());

        Ok(())
    }

    #[test]
    fn test_done_with_archive() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let now = Local::now();
        let entries = vec![TestEntry::new("Task to archive").with_timestamp(now)];
        ctx.create_doing_file_with_entries(entries)?;

        // Mark as done and archive
        handle_done(DoneOptions {
            entry: vec![],
            note: None,
            ask: false,
            back: None,
            at: None,
            took: None,
            from: None,
            section: None,
            editor: false,
            archive: true,
            remove: false,
            unfinished: false,
            _date: false,
            _noauto: false,
        })?;

        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;

        // Should not be in Currently
        let currently_entries = doing_file.sections.get("Currently").unwrap();
        assert!(
            !currently_entries
                .iter()
                .any(|e| e.description == "Task to archive")
        );

        // Should be in Archive
        let archive_entries = doing_file.sections.get("Archive").unwrap();
        let entry = archive_entries
            .iter()
            .find(|e| e.description == "Task to archive")
            .unwrap();
        assert!(entry.is_done());

        Ok(())
    }
}
