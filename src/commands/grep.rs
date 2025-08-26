use crate::display::{DisplayOptions, OutputFormat, SortOrder, TagSort, display_entries};
use crate::filtering::{
    BoolOp, CaseSensitivity, FilterOptions, filter_entries, parse_date_filter, parse_date_range,
};
use crate::storage::{Config, parse_taskpaper};
use std::io::{self, Write};

#[derive(Debug)]
pub struct GrepFilterOptions {
    pub pattern: String,
    pub after: Option<String>,
    pub before: Option<String>,
    pub bool_op: String,
    pub case: String,
    pub from: Option<String>,
    pub not: bool,
    pub only_timed: bool,
    pub sections: Vec<String>,
    pub tag: Option<String>,
    pub val: Vec<String>,
    pub exact: bool,
}

#[derive(Debug)]
pub struct GrepDisplayOptions {
    pub duration: bool,
    pub hilite: bool,
    pub output: Option<String>,
    pub times: bool,
    pub tag_order: String,
    pub tag_sort: String,
    pub totals: bool,
}

#[derive(Debug)]
pub struct GrepActionOptions {
    pub delete: bool,
    pub interactive: bool,
    pub _editor: bool,
}

#[derive(Debug)]
pub struct GrepConfigOptions {
    pub _config_template: Option<String>,
    pub _save: Option<String>,
    pub _template: Option<String>,
    pub _title: Option<String>,
}

pub fn handle_grep(
    filter_opts: GrepFilterOptions,
    display_opts: GrepDisplayOptions,
    action_opts: GrepActionOptions,
    _config_opts: GrepConfigOptions,
) -> color_eyre::Result<()> {
    if action_opts.interactive {
        return Err(color_eyre::eyre::eyre!(
            "Interactive mode not yet implemented"
        ));
    }

    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let mut doing_file = parse_taskpaper(&doing_file_path)?;

    // Parse tags from --tag option
    let filter_tags = if let Some(tag_str) = filter_opts.tag {
        tag_str.split(',').map(|t| t.trim().to_string()).collect()
    } else {
        vec![]
    };

    // Build filter options
    let mut filter_options = FilterOptions {
        search: Some(filter_opts.pattern.clone()),
        tags: filter_tags,
        sections: filter_opts.sections,
        case: match filter_opts.case.as_str() {
            "c" | "case-sensitive" => CaseSensitivity::CaseSensitive,
            "i" | "ignore" => CaseSensitivity::Ignore,
            _ => CaseSensitivity::Smart,
        },
        exact: filter_opts.exact,
        not: filter_opts.not,
        bool_op: match filter_opts.bool_op.as_str() {
            "and" | "AND" => BoolOp::And,
            "or" | "OR" => BoolOp::Or,
            "not" | "NOT" => BoolOp::Not,
            _ => BoolOp::Pattern,
        },
        only_timed: filter_opts.only_timed,
        val: filter_opts.val,
        ..Default::default()
    };

    // Parse date filters
    if let Some(after_str) = filter_opts.after {
        filter_options.after = Some(parse_date_filter(&after_str)?);
    }

    if let Some(before_str) = filter_opts.before {
        filter_options.before = Some(parse_date_filter(&before_str)?);
    }

    if let Some(from_str) = filter_opts.from {
        let (start, end) = parse_date_range(&from_str)?;
        filter_options.from = Some((start, end));
    }

    // Filter entries
    let entries = filter_entries(&doing_file, &filter_options)?;

    if entries.is_empty() {
        println!("No entries found matching '{}'", filter_opts.pattern);
        return Ok(());
    }

    // Handle delete mode
    if action_opts.delete {
        print!(
            "Delete {} matching {}? [y/N] ",
            if entries.len() == 1 {
                "entry"
            } else {
                "entries"
            },
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
        let uuids_to_delete: std::collections::HashSet<_> =
            entries.iter().map(|(_, e)| e.uuid).collect();

        for section in doing_file.sections.values_mut() {
            section.retain(|entry| !uuids_to_delete.contains(&entry.uuid));
        }

        crate::storage::save_taskpaper(&doing_file)?;
        println!(
            "Deleted {} {}.",
            entries.len(),
            if entries.len() == 1 {
                "entry"
            } else {
                "entries"
            }
        );
        return Ok(());
    }

    // Build display options
    let display_options = DisplayOptions {
        times: display_opts.times,
        duration: display_opts.duration,
        totals: display_opts.totals,
        hilite: display_opts.hilite,
        search_query: Some(filter_opts.pattern),
        output_format: match display_opts.output.as_deref() {
            Some("json") => OutputFormat::Json,
            Some("csv") => OutputFormat::Csv,
            Some("markdown") => OutputFormat::Markdown,
            Some("html") => OutputFormat::Html,
            Some("taskpaper") => OutputFormat::TaskPaper,
            Some("timeline") => OutputFormat::Timeline,
            _ => OutputFormat::Default,
        },
        tag_sort: match display_opts.tag_sort.as_str() {
            "time" => TagSort::Time,
            _ => TagSort::Name,
        },
        tag_order: match display_opts.tag_order.as_str() {
            "desc" => SortOrder::Desc,
            _ => SortOrder::Asc,
        },
        section_filter: filter_options.sections.clone(),
    };

    // Display entries
    display_entries(&entries, &display_options)?;

    Ok(())
}
