#[cfg(test)]
mod tests {
    use crate::test_utils::test_utils::{TestContext, TestEntry};
    use crate::commands::handle_last;
    use chrono::{Local, Duration};
    
    #[test]
    fn test_last_with_entries() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("First task")
                .with_timestamp(now - Duration::hours(2)),
            TestEntry::new("Second task")
                .with_timestamp(now - Duration::hours(1))
                .with_tags(vec!["urgent"]),
            TestEntry::new("Most recent task")
                .with_timestamp(now - Duration::minutes(30))
                .with_note("This is the latest"),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Since we can't capture stdout easily, just verify the command runs without error
        handle_last()?;
        
        Ok(())
    }
    
    #[test]
    fn test_last_with_empty_file() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n")?;
        
        // Should not panic with empty file
        handle_last()?;
        
        Ok(())
    }
    
    #[test]
    fn test_last_with_done_entry() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Completed task")
                .with_timestamp(now - Duration::hours(2))
                .with_done(now - Duration::hours(1)),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Should handle done entries properly
        handle_last()?;
        
        Ok(())
    }
    
    #[test]
    fn test_last_with_multiple_sections() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Task in Currently")
                .with_timestamp(now - Duration::hours(2))
                .with_section("Currently"),
            TestEntry::new("Task in Archive")
                .with_timestamp(now - Duration::hours(1))
                .with_section("Archive"),
            TestEntry::new("Latest in Projects")
                .with_timestamp(now - Duration::minutes(30))
                .with_section("Projects"),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Should find the most recent across all sections
        handle_last()?;
        
        Ok(())
    }
}