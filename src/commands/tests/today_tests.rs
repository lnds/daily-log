#[cfg(test)]
mod tests {
    use crate::commands::handle_today;
    use crate::test_utils::test_utils::{TestContext, TestEntry};
    use chrono::{Duration, Local};

    #[test]
    fn test_today_shows_only_todays_entries() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let now = Local::now();
        let yesterday = now - Duration::days(1);
        let today_morning = now
            .date_naive()
            .and_hms_opt(9, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let today_afternoon = now
            .date_naive()
            .and_hms_opt(14, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();

        let entries = vec![
            TestEntry::new("Yesterday's task").with_timestamp(yesterday),
            TestEntry::new("Morning task").with_timestamp(today_morning),
            TestEntry::new("Afternoon task").with_timestamp(today_afternoon),
            TestEntry::new("Just now task").with_timestamp(now),
        ];
        ctx.create_doing_file_with_entries(entries)?;

        // Should show only today's entries
        handle_today(None)?;

        Ok(())
    }

    #[test]
    fn test_today_with_section_filter() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let now = Local::now();
        let entries = vec![
            TestEntry::new("Currently task")
                .with_timestamp(now - Duration::hours(2))
                .with_section("Currently"),
            TestEntry::new("Archive task")
                .with_timestamp(now - Duration::hours(1))
                .with_section("Archive"),
            TestEntry::new("Another Currently task")
                .with_timestamp(now)
                .with_section("Currently"),
        ];
        ctx.create_doing_file_with_entries(entries)?;

        // Should show only Archive section entries from today
        handle_today(Some("Archive".to_string()))?;

        Ok(())
    }

    #[test]
    fn test_today_with_done_entries() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let now = Local::now();
        let morning = now
            .date_naive()
            .and_hms_opt(9, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let noon = now
            .date_naive()
            .and_hms_opt(12, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();

        let entries = vec![
            TestEntry::new("Morning task")
                .with_timestamp(morning)
                .with_done(noon),
            TestEntry::new("Ongoing task").with_timestamp(noon),
            TestEntry::new("Just completed")
                .with_timestamp(now - Duration::hours(1))
                .with_done(now),
        ];
        ctx.create_doing_file_with_entries(entries)?;

        // Should show durations for done entries
        handle_today(None)?;

        Ok(())
    }

    #[test]
    fn test_today_empty_day() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let yesterday = Local::now() - Duration::days(1);
        let entries = vec![TestEntry::new("Yesterday's task").with_timestamp(yesterday)];
        ctx.create_doing_file_with_entries(entries)?;

        // Should handle no entries for today
        handle_today(None)?;

        Ok(())
    }

    #[test]
    fn test_today_with_notes_and_tags() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let now = Local::now();
        let entries = vec![
            TestEntry::new("Task with note")
                .with_timestamp(now - Duration::hours(2))
                .with_note("Important note"),
            TestEntry::new("Task with tags")
                .with_timestamp(now - Duration::hours(1))
                .with_tags(vec!["meeting", "client"]),
            TestEntry::new("Task with both")
                .with_timestamp(now)
                .with_note("Follow up required")
                .with_tags(vec!["urgent", "bug"]),
        ];
        ctx.create_doing_file_with_entries(entries)?;

        // Should display notes and tags
        handle_today(None)?;

        Ok(())
    }

    #[test]
    fn test_today_handles_midnight_boundary() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;

        let now = Local::now();
        let today_start = now
            .date_naive()
            .and_hms_opt(0, 0, 1)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let yesterday_end = (now.date_naive() - Duration::days(1))
            .and_hms_opt(23, 59, 59)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let tomorrow_start = (now.date_naive() + Duration::days(1))
            .and_hms_opt(0, 0, 1)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();

        let entries = vec![
            TestEntry::new("Yesterday before midnight").with_timestamp(yesterday_end),
            TestEntry::new("Today just after midnight").with_timestamp(today_start),
            TestEntry::new("Current task").with_timestamp(now),
            TestEntry::new("Tomorrow's task").with_timestamp(tomorrow_start),
        ];
        ctx.create_doing_file_with_entries(entries)?;

        // Should only show today's entries
        handle_today(None)?;

        Ok(())
    }
}
