#[cfg(test)]
mod tests {
    use crate::test_utils::test_utils::{TestContext, TestEntry};
    use crate::commands::handle_now;
    use crate::storage::parse_taskpaper;
    use chrono::{Local, Duration};
    use regex::Regex;

    #[test]
    fn test_now_simple_entry() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n")?;
        
        // Add a simple entry
        handle_now(
            vec!["Working on tests".to_string()],
            None,
            None,
            None,
            false,
            None,
            false,
            false,
            false,
        )?;
        
        let content = ctx.read_test_file()?;
        assert!(content.contains("Working on tests"));
        assert!(content.contains("Currently:"));
        
        // Verify the entry was added with proper format
        let re = Regex::new(r"- \d{4}-\d{2}-\d{2} \d{2}:\d{2} \| Working on tests @?.*<[a-f0-9-]+>")?;
        assert!(re.is_match(&content));
        
        Ok(())
    }
    
    #[test]
    fn test_now_with_tags() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n")?;
        
        // Add entry with tags
        handle_now(
            vec!["Fix bug @urgent @bug".to_string()],
            None,
            None,
            None,
            false,
            None,
            false,
            false,
            false,
        )?;
        
        let content = ctx.read_test_file()?;
        assert!(content.contains("Fix bug"));
        assert!(content.contains("@bug"));
        assert!(content.contains("@urgent"));
        
        Ok(())
    }
    
    #[test]
    fn test_now_with_parenthetical_note() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n")?;
        
        // Add entry with parenthetical note
        handle_now(
            vec!["Deploy app (remember to update configs)".to_string()],
            None,
            None,
            None,
            false,
            None,
            false,
            false,
            false,
        )?;
        
        let content = ctx.read_test_file()?;
        assert!(content.contains("Deploy app"));
        assert!(content.contains("remember to update configs"));
        
        Ok(())
    }
    
    #[test]
    fn test_now_with_note_flag() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n")?;
        
        // Add entry with --note flag
        handle_now(
            vec!["Important task".to_string()],
            Some("This is a note".to_string()),
            None,
            None,
            false,
            None,
            false,
            false,
            false,
        )?;
        
        let content = ctx.read_test_file()?;
        assert!(content.contains("Important task"));
        assert!(content.contains("This is a note"));
        
        Ok(())
    }
    
    #[test]
    fn test_now_with_back_flag() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n")?;
        
        // Add backdated entry
        handle_now(
            vec!["Old task".to_string()],
            None,
            Some("2 hours ago".to_string()),
            None,
            false,
            None,
            false,
            false,
            false,
        )?;
        
        let _content = ctx.read_test_file()?;
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        
        // Check that the entry exists and is backdated
        let entry = doing_file.sections.get("Currently")
            .and_then(|entries| entries.iter().find(|e| e.description == "Old task"))
            .expect("Entry should exist");
        
        let expected_time = Local::now() - Duration::hours(2);
        let time_diff = entry.timestamp.signed_duration_since(expected_time).num_minutes().abs();
        assert!(time_diff < 2, "Entry should be backdated by approximately 2 hours");
        
        Ok(())
    }
    
    #[test]
    fn test_now_with_finish_last() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        
        // Create file with an unfinished entry
        let now = Local::now();
        let entries = vec![
            TestEntry::new("Previous task")
                .with_timestamp(now - Duration::minutes(5))
                .with_tags(vec!["wip"]),
        ];
        ctx.create_doing_file_with_entries(entries)?;
        
        // Add new entry with --finish_last
        handle_now(
            vec!["New task".to_string()],
            None,
            None,
            None,
            true,
            None,
            false,
            false,
            false,
        )?;
        
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entries = doing_file.sections.get("Currently").unwrap();
        
        // Previous task should be marked as done
        let prev_entry = entries.iter().find(|e| e.description == "Previous task").unwrap();
        assert!(prev_entry.is_done(), "Previous entry should be marked as done");
        
        // New task should exist and not be done
        let new_entry = entries.iter().find(|e| e.description == "New task").unwrap();
        assert!(!new_entry.is_done(), "New entry should not be marked as done");
        
        Ok(())
    }
    
    #[test]
    fn test_now_with_from_flag() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n")?;
        
        // Add entry with time range
        handle_now(
            vec!["Meeting".to_string()],
            None,
            None,
            None,
            false,
            Some("from 2pm to 3:30pm".to_string()),
            false,
            false,
            false,
        )?;
        
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        let entry = doing_file.sections.get("Currently")
            .and_then(|entries| entries.iter().find(|e| e.description == "Meeting"))
            .expect("Entry should exist");
        
        // Entry should be marked as done with the end time
        assert!(entry.is_done(), "Entry should be marked as done");
        
        Ok(())
    }
    
    #[test]
    fn test_now_with_section() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n\nProjects:\n")?;
        
        // Add entry to specific section
        handle_now(
            vec!["Project task".to_string()],
            None,
            None,
            Some("Projects".to_string()),
            false,
            None,
            false,
            false,
            false,
        )?;
        
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        
        // Debug: print all sections
        println!("Sections: {:?}", doing_file.sections.keys().collect::<Vec<_>>());
        
        // Entry should be in Projects section
        assert!(doing_file.sections.get("Projects")
            .map(|entries| entries.iter().any(|e| e.description == "Project task"))
            .unwrap_or(false), "Entry should be in Projects section");
        
        // Entry should NOT be in Currently section
        assert!(!doing_file.sections.get("Currently")
            .map(|entries| entries.iter().any(|e| e.description == "Project task"))
            .unwrap_or(false), "Entry should NOT be in Currently section");
        
        Ok(())
    }
    
    #[test]
    fn test_now_creates_section_if_not_exists() -> color_eyre::Result<()> {
        let ctx = TestContext::new()?;
        ctx.create_test_file("Currently:\n")?;
        
        // Add entry to non-existent section
        handle_now(
            vec!["New section task".to_string()],
            None,
            None,
            Some("NewSection".to_string()),
            false,
            None,
            false,
            false,
            false,
        )?;
        
        let doing_file = parse_taskpaper(&ctx.doing_file_path)?;
        
        // NewSection should exist
        assert!(doing_file.sections.contains_key("NewSection"));
        assert!(doing_file.sections.get("NewSection")
            .map(|entries| entries.iter().any(|e| e.description == "New section task"))
            .unwrap_or(false));
        
        Ok(())
    }
}