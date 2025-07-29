#[cfg(test)]
mod tests {
    use crate::test_utils::test_utils::*;
    use crate::commands::tags::{handle_tags, TagsFilterOptions, TagsDisplayOptions};
    
    #[test]
    fn test_tags_list_all() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task 1 @bug @priority(high) <uuid1>\n - 2025-07-28 11:00 | Task 2 @feature @done(2025-07-28 12:00) <uuid2>\n"
        ).unwrap();
        
        // List all tags (basic functionality test - output goes to stdout)
        let result = handle_tags(
            TagsFilterOptions {
                section: vec![],
                search: None,
                tag: None,
                val: vec![],
                case: "smart".to_string(),
                exact: false,
                not: false,
            },
            TagsDisplayOptions {
                max_count: None,
                counts: false,
                line: false,
                order: "asc".to_string(),
                sort: "name".to_string(),
            },
            false, // interactive
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_tags_with_counts() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task 1 @bug <uuid1>\n - 2025-07-28 11:00 | Task 2 @bug <uuid2>\n - 2025-07-28 12:00 | Task 3 @feature <uuid3>\n"
        ).unwrap();
        
        // List tags with counts
        let result = handle_tags(
            TagsFilterOptions {
                section: vec![],
                search: None,
                tag: None,
                val: vec![],
                case: "smart".to_string(),
                exact: false,
                not: false,
            },
            TagsDisplayOptions {
                max_count: None,
                counts: true,
                line: false,
                order: "asc".to_string(),
                sort: "name".to_string(),
            },
            false,
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_tags_from_specific_section() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task 1 @current <uuid1>\n\nWork:\n - 2025-07-28 11:00 | Task 2 @work <uuid2>\n"
        ).unwrap();
        
        // List tags only from Work section
        let result = handle_tags(
            TagsFilterOptions {
                section: vec!["Work".to_string()],
                search: None,
                tag: None,
                val: vec![],
                case: "smart".to_string(),
                exact: false,
                not: false,
            },
            TagsDisplayOptions {
                max_count: None,
                counts: false,
                line: false,
                order: "asc".to_string(),
                sort: "name".to_string(),
            },
            false,
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_tags_with_search_filter() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Fix bug @bug <uuid1>\n - 2025-07-28 11:00 | Add feature @feature <uuid2>\n"
        ).unwrap();
        
        // List tags only from entries containing "bug"
        let result = handle_tags(
            TagsFilterOptions {
                section: vec![],
                search: Some("bug".to_string()),
                tag: None,
                val: vec![],
                case: "smart".to_string(),
                exact: false,
                not: false,
            },
            TagsDisplayOptions {
                max_count: None,
                counts: false,
                line: false,
                order: "asc".to_string(),
                sort: "name".to_string(),
            },
            false,
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_tags_with_max_count() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task @tag1 <uuid1>\n - 2025-07-28 11:00 | Task @tag2 <uuid2>\n - 2025-07-28 12:00 | Task @tag3 <uuid3>\n"
        ).unwrap();
        
        // List only 2 tags
        let result = handle_tags(
            TagsFilterOptions {
                section: vec![],
                search: None,
                tag: None,
                val: vec![],
                case: "smart".to_string(),
                exact: false,
                not: false,
            },
            TagsDisplayOptions {
                max_count: Some(2),
                counts: false,
                line: false,
                order: "asc".to_string(),
                sort: "name".to_string(),
            },
            false,
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_tags_sort_by_count() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task 1 @common <uuid1>\n - 2025-07-28 11:00 | Task 2 @common <uuid2>\n - 2025-07-28 12:00 | Task 3 @rare <uuid3>\n"
        ).unwrap();
        
        // Sort by count (descending)
        let result = handle_tags(
            TagsFilterOptions {
                section: vec![],
                search: None,
                tag: None,
                val: vec![],
                case: "smart".to_string(),
                exact: false,
                not: false,
            },
            TagsDisplayOptions {
                max_count: None,
                counts: true,
                line: false,
                order: "desc".to_string(),
                sort: "count".to_string(),
            },
            false,
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_tags_line_format() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file(
            "Currently:\n - 2025-07-28 10:00 | Task @tag1 @tag2 <uuid1>\n"
        ).unwrap();
        
        // Display in line format
        let result = handle_tags(
            TagsFilterOptions {
                section: vec![],
                search: None,
                tag: None,
                val: vec![],
                case: "smart".to_string(),
                exact: false,
                not: false,
            },
            TagsDisplayOptions {
                max_count: None,
                counts: false,
                line: true,
                order: "asc".to_string(),
                sort: "name".to_string(),
            },
            false,
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_tags_empty_file() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file("Currently:\n").unwrap();
        
        // List tags from empty file
        let result = handle_tags(
            TagsFilterOptions {
                section: vec![],
                search: None,
                tag: None,
                val: vec![],
                case: "smart".to_string(),
                exact: false,
                not: false,
            },
            TagsDisplayOptions {
                max_count: None,
                counts: false,
                line: false,
                order: "asc".to_string(),
                sort: "name".to_string(),
            },
            false,
        );
        assert!(result.is_ok());
    }
}