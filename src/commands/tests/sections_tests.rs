#[cfg(test)]
mod tests {
    use crate::cli::SectionsAction;
    use crate::commands::sections::handle_sections;
    use crate::storage::parse_taskpaper;
    use crate::test_utils::test_utils::*;

    #[test]
    fn test_sections_list_empty() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file("Currently:\n").unwrap();

        // List sections - should show Currently with 0 entries
        let result = handle_sections(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sections_list_with_entries() {
        let ctx = TestContext::new().unwrap();
        ctx.create_sample_doing_file().unwrap();

        // List sections
        let result = handle_sections(None);
        assert!(result.is_ok());

        // Just verify that sections command can run without errors
        // The actual output goes to stdout
    }

    #[test]
    fn test_sections_add() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file("Currently:\n").unwrap();

        // Add a new section
        let result = handle_sections(Some(SectionsAction::Add {
            section_name: "Work".to_string(),
        }));
        assert!(result.is_ok());

        // Verify section was added
        let doing_file = parse_taskpaper(&ctx.doing_file_path).unwrap();
        assert!(doing_file.sections.contains_key("Work"));
        assert_eq!(doing_file.sections.get("Work").unwrap().len(), 0);
    }

    #[test]
    fn test_sections_add_duplicate() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file("Currently:\n").unwrap();

        // Add a section
        handle_sections(Some(SectionsAction::Add {
            section_name: "Work".to_string(),
        }))
        .unwrap();

        // Try to add the same section again
        let result = handle_sections(Some(SectionsAction::Add {
            section_name: "Work".to_string(),
        }));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_sections_remove() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file("Currently:\n\nWork:\n").unwrap();

        // Remove the Work section
        let result = handle_sections(Some(SectionsAction::Remove {
            section_name: "Work".to_string(),
            archive: false,
        }));
        assert!(result.is_ok());

        // Verify section was removed
        let doing_file = parse_taskpaper(&ctx.doing_file_path).unwrap();
        assert!(!doing_file.sections.contains_key("Work"));
    }

    #[test]
    fn test_sections_remove_with_archive() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file("Currently:\n\nWork:\n - 2025-07-28 10:00 | Work task <12345678-1234-5678-1234-567812345678>\n\nArchive:\n").unwrap();

        // Remove the Work section with archive flag
        let result = handle_sections(Some(SectionsAction::Remove {
            section_name: "Work".to_string(),
            archive: true,
        }));
        assert!(result.is_ok());

        // Verify the command executed successfully
    }

    #[test]
    fn test_sections_remove_currently_fails() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file("Currently:\n").unwrap();

        // Try to remove the Currently section
        let result = handle_sections(Some(SectionsAction::Remove {
            section_name: "Currently".to_string(),
            archive: false,
        }));
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Cannot remove the 'Currently' section")
        );
    }

    #[test]
    fn test_sections_remove_nonexistent() {
        let ctx = TestContext::new().unwrap();
        ctx.create_test_file("Currently:\n").unwrap();

        // Try to remove a non-existent section
        let result = handle_sections(Some(SectionsAction::Remove {
            section_name: "NonExistent".to_string(),
            archive: false,
        }));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }
}
