#[cfg(test)]
mod tests {
    use crate::commands::{ArchiveOptions, handle_archive};
    use crate::test_utils::utils::*;
    use chrono::Local;

    #[test]
    fn test_archive_by_section() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Current task <11111111-1111-1111-1111-111111111111>\n\nWork:\n - 2025-07-28 11:00 | Work task <22222222-2222-2222-2222-222222222222>\n"
        ).unwrap();

        // Archive Work section
        let result = handle_archive(ArchiveOptions {
            target: Some("Work".to_string()),
            after: None,
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            from: None,
            keep: None,
            label: true,
            not: false,
            search: None,
            to: "Archive".to_string(),
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_archive_by_tag() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task 1 @urgent <11111111-1111-1111-1111-111111111111>\n - 2025-07-28 11:00 | Task 2 <22222222-2222-2222-2222-222222222222>\n"
        ).unwrap();

        // Archive entries with @urgent tag
        let result = handle_archive(ArchiveOptions {
            target: Some("@urgent".to_string()),
            after: None,
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            from: None,
            keep: None,
            label: false,
            not: false,
            search: None,
            to: "Archive".to_string(),
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_archive_with_search() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Fix bug in parser <11111111-1111-1111-1111-111111111111>\n - 2025-07-28 11:00 | Add new feature <22222222-2222-2222-2222-222222222222>\n"
        ).unwrap();

        // Archive entries matching search
        let result = handle_archive(ArchiveOptions {
            target: None,
            after: None,
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            from: None,
            keep: None,
            label: false,
            not: false,
            search: Some("bug".to_string()),
            to: "Archive".to_string(),
            tag: None,
            val: vec![],
            exact: false,
        });
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
        ])
        .unwrap();

        // Archive entries before today
        let result = handle_archive(ArchiveOptions {
            target: None,
            after: None,
            before: Some(now.format("%Y-%m-%d").to_string()),
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            from: None,
            keep: None,
            label: false,
            not: false,
            search: None,
            to: "Archive".to_string(),
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_archive_with_keep() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task 1 <11111111-1111-1111-1111-111111111111>\n - 2025-07-28 11:00 | Task 2 <22222222-2222-2222-2222-222222222222>\n - 2025-07-28 12:00 | Task 3 <33333333-3333-3333-3333-333333333333>\n"
        ).unwrap();

        // Archive only 2 entries
        let result = handle_archive(ArchiveOptions {
            target: Some("Currently".to_string()),
            after: None,
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            from: None,
            keep: Some(2),
            label: false,
            not: false,
            search: None,
            to: "Archive".to_string(),
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_archive_creates_section() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task <11111111-1111-1111-1111-111111111111>\n",
        )
        .unwrap();

        // Archive to non-existent section
        let result = handle_archive(ArchiveOptions {
            target: Some("Currently".to_string()),
            after: None,
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            from: None,
            keep: None,
            label: false,
            not: false,
            search: None,
            to: "Completed".to_string(),
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_archive_empty_section() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file("Currently:\n\nWork:\n").unwrap();

        // Try to archive empty section
        let result = handle_archive(ArchiveOptions {
            target: Some("Work".to_string()),
            after: None,
            before: None,
            _bool_op: "pattern".to_string(),
            case: "smart".to_string(),
            from: None,
            keep: None,
            label: false,
            not: false,
            search: None,
            to: "Archive".to_string(),
            tag: None,
            val: vec![],
            exact: false,
        });
        assert!(result.is_ok());
    }
}
