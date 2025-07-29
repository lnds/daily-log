#[cfg(test)]
mod tests {
    use crate::test_utils::test_utils::*;
    use crate::commands::archive::handle_archive;
    use chrono::Local;
    
    #[test]
    fn test_archive_by_section() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Current task <11111111-1111-1111-1111-111111111111>\n\nWork:\n - 2025-07-28 11:00 | Work task <22222222-2222-2222-2222-222222222222>\n"
        ).unwrap();
        
        // Archive Work section
        let result = handle_archive(
            Some("Work".to_string()),
            None, None, "pattern".to_string(), "smart".to_string(),
            None, None, true, false, None, "Archive".to_string(),
            None, vec![], false
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_archive_by_tag() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task 1 @urgent <11111111-1111-1111-1111-111111111111>\n - 2025-07-28 11:00 | Task 2 <22222222-2222-2222-2222-222222222222>\n"
        ).unwrap();
        
        // Archive entries with @urgent tag
        let result = handle_archive(
            Some("@urgent".to_string()),
            None, None, "pattern".to_string(), "smart".to_string(),
            None, None, false, false, None, "Archive".to_string(),
            None, vec![], false
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_archive_with_search() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Fix bug in parser <11111111-1111-1111-1111-111111111111>\n - 2025-07-28 11:00 | Add new feature <22222222-2222-2222-2222-222222222222>\n"
        ).unwrap();
        
        // Archive entries matching search
        let result = handle_archive(
            None,
            None, None, "pattern".to_string(), "smart".to_string(),
            None, None, false, false, Some("bug".to_string()), "Archive".to_string(),
            None, vec![], false
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_archive_with_date_range() {
        let ctx = TestContext::new().unwrap();
        let now = Local::now();
        let yesterday = now - chrono::Duration::days(1);
        
        ctx.create_doing_file_with_entries(vec![
            TestEntry::new("Old task")
                .with_timestamp(yesterday)
                .with_uuid(uuid::Uuid::new_v4()),
            TestEntry::new("New task")
                .with_timestamp(now)
                .with_uuid(uuid::Uuid::new_v4()),
        ]).unwrap();
        
        // Archive entries before today
        let result = handle_archive(
            None,
            None, Some(now.format("%Y-%m-%d").to_string()), 
            "pattern".to_string(), "smart".to_string(),
            None, None, false, false, None, "Archive".to_string(),
            None, vec![], false
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_archive_with_keep() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task 1 <11111111-1111-1111-1111-111111111111>\n - 2025-07-28 11:00 | Task 2 <22222222-2222-2222-2222-222222222222>\n - 2025-07-28 12:00 | Task 3 <33333333-3333-3333-3333-333333333333>\n"
        ).unwrap();
        
        // Archive only 2 entries
        let result = handle_archive(
            Some("Currently".to_string()),
            None, None, "pattern".to_string(), "smart".to_string(),
            None, Some(2), false, false, None, "Archive".to_string(),
            None, vec![], false
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_archive_creates_section() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task <11111111-1111-1111-1111-111111111111>\n"
        ).unwrap();
        
        // Archive to non-existent section
        let result = handle_archive(
            Some("Currently".to_string()),
            None, None, "pattern".to_string(), "smart".to_string(),
            None, None, false, false, None, "Completed".to_string(),
            None, vec![], false
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_archive_empty_section() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n\nWork:\n"
        ).unwrap();
        
        // Try to archive empty section
        let result = handle_archive(
            Some("Work".to_string()),
            None, None, "pattern".to_string(), "smart".to_string(),
            None, None, false, false, None, "Archive".to_string(),
            None, vec![], false
        );
        assert!(result.is_ok());
    }
}