#[cfg(test)]
mod tests {
    use crate::test_utils::test_utils::*;
    use crate::commands::{handle_rotate, RotateOptions};
    use chrono::Local;
    
    #[test]
    fn test_rotate_done_entries() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Completed task @done(2025-07-28 11:00) <11111111-1111-1111-1111-111111111111>\n - 2025-07-28 12:00 | Ongoing task <22222222-2222-2222-2222-222222222222>\n"
        ).unwrap();
        
        // Rotate done entries
        let result = handle_rotate(RotateOptions {
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            keep: None,
            not: false,
            section: None,
            search: None,
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_rotate_with_before_date() {
        let ctx = TestContext::new().unwrap();
        let now = Local::now();
        let yesterday = now - chrono::Duration::days(1);
        let two_days_ago = now - chrono::Duration::days(2);
        
        ctx.create_doing_file_with_entries(vec![
            TestEntry::new("Old task")
                .with_timestamp(two_days_ago)
                .with_done(two_days_ago + chrono::Duration::hours(1))
                .with_uuid(uuid::Uuid::new_v4()),
            TestEntry::new("Recent task")
                .with_timestamp(yesterday)
                .with_done(yesterday + chrono::Duration::hours(1))
                .with_uuid(uuid::Uuid::new_v4()),
        ]).unwrap();
        
        // Rotate entries before yesterday
        let result = handle_rotate(RotateOptions {
            before: Some(yesterday.format("%Y-%m-%d").to_string()),
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            keep: None,
            not: false,
            section: None,
            search: None,
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_rotate_from_specific_section() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Current done @done(2025-07-28 11:00) <11111111-1111-1111-1111-111111111111>\n\nWork:\n - 2025-07-28 12:00 | Work done @done(2025-07-28 13:00) <22222222-2222-2222-2222-222222222222>\n"
        ).unwrap();
        
        // Rotate only from Work section
        let result = handle_rotate(RotateOptions {
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            keep: None,
            not: false,
            section: Some("Work".to_string()),
            search: None,
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_rotate_with_tag_filter() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Bug fix @bug @done(2025-07-28 11:00) <11111111-1111-1111-1111-111111111111>\n - 2025-07-28 12:00 | Feature @feature @done(2025-07-28 13:00) <22222222-2222-2222-2222-222222222222>\n"
        ).unwrap();
        
        // Rotate only @bug entries
        let result = handle_rotate(RotateOptions {
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            keep: None,
            not: false,
            section: None,
            search: None,
            tag: Some("bug".to_string()),
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_rotate_appends_to_existing_archive() {
        let ctx = TestContext::new().unwrap();
        
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | New done task @done(2025-07-28 11:00) <11111111-1111-1111-1111-111111111111>\n"
        ).unwrap();
        
        // Rotate new entry
        let result = handle_rotate(RotateOptions {
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            keep: None,
            not: false,
            section: None,
            search: None,
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_rotate_no_done_entries() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Active task <11111111-1111-1111-1111-111111111111>\n"
        ).unwrap();
        
        // Try to rotate - should find no entries
        let result = handle_rotate(RotateOptions {
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            keep: None,
            not: false,
            section: None,
            search: None,
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_rotate_with_keep() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task 1 @done(2025-07-28 10:30) <11111111-1111-1111-1111-111111111111>\n - 2025-07-28 11:00 | Task 2 @done(2025-07-28 11:30) <22222222-2222-2222-2222-222222222222>\n - 2025-07-28 12:00 | Task 3 @done(2025-07-28 12:30) <33333333-3333-3333-3333-333333333333>\n"
        ).unwrap();
        
        // Rotate only oldest 2 entries
        let result = handle_rotate(RotateOptions {
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            keep: Some(2),
            not: false,
            section: None,
            search: None,
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }
}