#[cfg(test)]
mod tests {
    use crate::test_utils::test_utils::{TestContext, TestEntry};
    use crate::commands::{handle_delete, DeleteOptions};
    use crate::storage::parse_taskpaper;
    use chrono::{Local, Duration};
    
    #[test]
    fn test_delete_last_entry() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Keep this").with_timestamp(now - Duration::hours(2)),
            TestEntry::new("Delete this").with_timestamp(now),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Delete last entry
        handle_delete(DeleteOptions {
            count: 1,
            interactive: false,
            not: false,
            sections: vec![],
            search: None,
            tag: None,
            exact: false,
            force: true,
        })?;
        
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entries = doing_file.sections.get("Currently").unwrap();
        
        // Should only have one entry left
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].description, "Keep this");
        
        Ok(())
    }
    
    #[test]
    fn test_delete_multiple_entries() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Task 1").with_timestamp(now - Duration::hours(3)),
            TestEntry::new("Task 2").with_timestamp(now - Duration::hours(2)),
            TestEntry::new("Task 3").with_timestamp(now - Duration::hours(1)),
            TestEntry::new("Task 4").with_timestamp(now),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Delete last 2 entries
        handle_delete(DeleteOptions {
            count: 2,
            interactive: false,
            not: false,
            sections: vec![],
            search: None,
            tag: None,
            exact: false,
            force: true,
        })?;
        
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entries = doing_file.sections.get("Currently").unwrap();
        
        // Should have 2 entries left
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].description, "Task 1");
        assert_eq!(entries[1].description, "Task 2");
        
        Ok(())
    }
    
    #[test]
    fn test_delete_with_tag_filter() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Keep this").with_timestamp(now - Duration::hours(2)),
            TestEntry::new("Delete this")
                .with_timestamp(now - Duration::hours(1))
                .with_tags(vec!["urgent"]),
            TestEntry::new("Keep this too").with_timestamp(now),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Delete entries with urgent tag
        handle_delete(DeleteOptions {
            count: 10,
            interactive: false,
            not: false,
            sections: vec![],
            search: None,
            tag: Some("urgent".to_string()),
            exact: false,
            force: true,
        })?;
        
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entries = doing_file.sections.get("Currently").unwrap();
        
        // Should have deleted the urgent entry
        assert_eq!(entries.len(), 2);
        assert!(!entries.iter().any(|e| e.tags.contains_key("urgent")));
        
        Ok(())
    }
    
    #[test]
    fn test_delete_with_search() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Keep this task").with_timestamp(now - Duration::hours(2)),
            TestEntry::new("Delete bug fix").with_timestamp(now - Duration::hours(1)),
            TestEntry::new("Keep another task").with_timestamp(now),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Delete entries matching search
        handle_delete(DeleteOptions {
            count: 10,
            interactive: false,
            not: false,
            sections: vec![],
            search: Some("bug".to_string()),
            tag: None,
            exact: false,
            force: true,
        })?;
        
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entries = doing_file.sections.get("Currently").unwrap();
        
        // Should have deleted the bug entry
        assert_eq!(entries.len(), 2);
        assert!(!entries.iter().any(|e| e.description.contains("bug")));
        
        Ok(())
    }
    
    #[test]
    fn test_delete_from_section() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Currently task").with_timestamp(now),
            TestEntry::new("Project task")
                .with_timestamp(now - Duration::minutes(30))
                .with_section("Projects"),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Delete from Projects section
        handle_delete(DeleteOptions {
            count: 1,
            interactive: false,
            not: false,
            sections: vec!["Projects".to_string()],
            search: None,
            tag: None,
            exact: false,
            force: true,
        })?;
        
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        
        // Projects section might be removed if empty, so check if it exists
        if let Some(projects) = doing_file.sections.get("Projects") {
            assert!(projects.is_empty());
        }
        
        // Currently section should still have its entry
        assert_eq!(doing_file.sections.get("Currently").unwrap().len(), 1);
        
        Ok(())
    }
}