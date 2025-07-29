use crate::display::{DisplayOptions, OutputFormat, TagSort, SortOrder, display_entries};
use crate::filtering::{FilterOptions, CaseSensitivity, BoolOp, filter_entries, parse_date_filter, parse_date_range};
use crate::storage::{Config, parse_taskpaper};
use std::io::{self, Write};

pub fn handle_grep(
    pattern: String,
    after: Option<String>,
    before: Option<String>,
    bool_op: String,
    case: String,
    _config_template: Option<String>,
    delete: bool,
    duration: bool,
    _editor: bool,
    from: Option<String>,
    hilite: bool,
    interactive: bool,
    not: bool,
    output: Option<String>,
    only_timed: bool,
    sections: Vec<String>,
    _save: Option<String>,
    times: bool,
    tag: Option<String>,
    tag_order: String,
    tag_sort: String,
    _template: Option<String>,
    _title: Option<String>,
    totals: bool,
    val: Vec<String>,
    exact: bool,
) -> color_eyre::Result<()> {
    if interactive {
        return Err(color_eyre::eyre::eyre!("Interactive mode not yet implemented"));
    }

    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let mut doing_file = parse_taskpaper(&doing_file_path)?;

    // Parse tags from --tag option
    let filter_tags = if let Some(tag_str) = tag {
        tag_str.split(',').map(|t| t.trim().to_string()).collect()
    } else {
        vec![]
    };

    // Build filter options
    let mut filter_opts = FilterOptions {
        search: Some(pattern.clone()),
        tags: filter_tags,
        sections,
        case: match case.as_str() {
            "c" | "case-sensitive" => CaseSensitivity::CaseSensitive,
            "i" | "ignore" => CaseSensitivity::Ignore,
            _ => CaseSensitivity::Smart,
        },
        exact,
        not,
        bool_op: match bool_op.as_str() {
            "and" | "AND" => BoolOp::And,
            "or" | "OR" => BoolOp::Or,
            "not" | "NOT" => BoolOp::Not,
            _ => BoolOp::Pattern,
        },
        only_timed,
        val,
        ..Default::default()
    };

    // Parse date filters
    if let Some(after_str) = after {
        filter_opts.after = Some(parse_date_filter(&after_str)?);
    }

    if let Some(before_str) = before {
        filter_opts.before = Some(parse_date_filter(&before_str)?);
    }

    if let Some(from_str) = from {
        let (start, end) = parse_date_range(&from_str)?;
        filter_opts.from = Some((start, end));
    }

    // Filter entries
    let entries = filter_entries(&doing_file, &filter_opts)?;

    if entries.is_empty() {
        println!("No entries found matching '{}'", pattern);
        return Ok(());
    }

    // Handle delete mode
    if delete {
        print!("Delete {} matching {}? [y/N] ", 
            if entries.len() == 1 { "entry" } else { "entries" },
            if entries.len() == 1 { "" } else { "entries" }
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Deletion cancelled.");
            return Ok(());
        }

        // Remove matching entries
        let uuids_to_delete: std::collections::HashSet<_> = entries.iter()
            .map(|(_, e)| e.uuid)
            .collect();

        for section in doing_file.sections.values_mut() {
            section.retain(|entry| !uuids_to_delete.contains(&entry.uuid));
        }

        crate::storage::save_taskpaper(&doing_file)?;
        println!("Deleted {} {}.", 
            entries.len(), 
            if entries.len() == 1 { "entry" } else { "entries" }
        );
        return Ok(());
    }

    // Build display options
    let display_opts = DisplayOptions {
        times,
        duration,
        totals,
        hilite,
        search_query: Some(pattern),
        output_format: match output.as_deref() {
            Some("json") => OutputFormat::Json,
            Some("csv") => OutputFormat::Csv,
            Some("markdown") => OutputFormat::Markdown,
            Some("html") => OutputFormat::Html,
            Some("taskpaper") => OutputFormat::TaskPaper,
            Some("timeline") => OutputFormat::Timeline,
            _ => OutputFormat::Default,
        },
        tag_sort: match tag_sort.as_str() {
            "time" => TagSort::Time,
            _ => TagSort::Name,
        },
        tag_order: match tag_order.as_str() {
            "desc" => SortOrder::Desc,
            _ => SortOrder::Asc,
        },
        section_filter: filter_opts.sections.clone(),
    };

    // Display entries
    display_entries(&entries, &display_opts)?;

    Ok(())
}