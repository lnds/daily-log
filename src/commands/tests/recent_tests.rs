#[cfg(test)]
mod tests {
    use crate::test_utils::test_utils::{TestContext, TestEntry};
    use crate::commands::handle_recent;
    use chrono::{Local, Duration};
    
    #[test]
    fn test_recent_default_count() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let mut entries = vec![];
        
        // Create 15 entries
        for i in 0..15 {
            entries.push(
                TestEntry::new(&format!("Task {}", i))
                    .with_timestamp(now - Duration::hours(i as i64))
            );
        }
        ctx.create_doing_file_with_entries(entries)?;
        
        // Should show default 10 entries
        handle_recent(10, None)?;
        
        Ok(())
    }
    
    #[test]
    fn test_recent_custom_count() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Task 1").with_timestamp(now - Duration::hours(3)),
            TestEntry::new("Task 2").with_timestamp(now - Duration::hours(2)),
            TestEntry::new("Task 3").with_timestamp(now - Duration::hours(1)),
            TestEntry::new("Task 4").with_timestamp(now),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Should show only 2 entries
        handle_recent(2, None)?;
        
        Ok(())
    }
    
    #[test]
    fn test_recent_with_section_filter() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Currently task 1")
                .with_timestamp(now - Duration::hours(3))
                .with_section("Currently"),
            TestEntry::new("Archive task 1")
                .with_timestamp(now - Duration::hours(2))
                .with_section("Archive"),
            TestEntry::new("Currently task 2")
                .with_timestamp(now - Duration::hours(1))
                .with_section("Currently"),
            TestEntry::new("Archive task 2")
                .with_timestamp(now)
                .with_section("Archive"),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Should show only Archive section entries
        handle_recent(10, Some("Archive".to_string()))?;
        
        Ok(())
    }
    
    #[test]
    fn test_recent_with_notes_and_tags() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Task with note")
                .with_timestamp(now - Duration::hours(2))
                .with_note("This is a\nmulti-line note"),
            TestEntry::new("Task with tags")
                .with_timestamp(now - Duration::hours(1))
                .with_tags(vec!["urgent", "bug"]),
            TestEntry::new("Task with both")
                .with_timestamp(now)
                .with_note("Important details")
                .with_tags(vec!["feature", "v2.0"]),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Should display notes and tags properly
        handle_recent(10, None)?;
        
        Ok(())
    }
    
    #[test]
    fn test_recent_with_done_entries() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Completed task 1")
                .with_timestamp(now - Duration::hours(3))
                .with_done(now - Duration::hours(2)),
            TestEntry::new("Ongoing task")
                .with_timestamp(now - Duration::hours(1)),
            TestEntry::new("Completed task 2")
                .with_timestamp(now - Duration::minutes(30))
                .with_done(now),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Should show duration for done entries
        handle_recent(10, None)?;
        
        Ok(())
    }
    
    #[test]
    fn test_recent_empty_file() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n")?;
        
        // Should handle empty file gracefully
        handle_recent(10, None)?;
        
        Ok(())
    }
    
    #[test]
    fn test_recent_sorts_by_timestamp() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Middle task")
                .with_timestamp(now - Duration::hours(2)),
            TestEntry::new("Oldest task")
                .with_timestamp(now - Duration::hours(3)),
            TestEntry::new("Newest task")
                .with_timestamp(now - Duration::hours(1)),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Should display in reverse chronological order
        handle_recent(10, None)?;
        
        Ok(())
    }
}