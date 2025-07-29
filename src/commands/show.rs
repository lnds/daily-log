use crate::display::{DisplayOptions, OutputFormat, TagSort, SortOrder, display_entries};
use crate::filtering::{FilterOptions, CaseSensitivity, BoolOp, filter_entries, parse_date_filter, parse_date_range};
use crate::storage::{Config, parse_taskpaper};

pub fn handle_show(
    args: Vec<String>,
    age: String,
    after: Option<String>,
    before: Option<String>,
    bool_op: String,
    count: usize,
    case: String,
    _config_template: Option<String>,
    duration: bool,
    _editor: bool,
    from: Option<String>,
    hilite: bool,
    interactive: bool,
    menu: bool,
    not: bool,
    output: Option<String>,
    only_timed: bool,
    sections: Vec<String>,
    _save: Option<String>,
    search: Option<String>,
    sort: String,
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

    if menu {
        return Err(color_eyre::eyre::eyre!("Menu mode not yet implemented"));
    }

    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let doing_file = parse_taskpaper(&doing_file_path)?;

    // Parse arguments for sections and tags
    let mut filter_sections = sections;
    let mut filter_tags = vec![];

    for arg in &args {
        if arg.starts_with('@') {
            filter_tags.push(arg.clone());
        } else if arg == "pick" || arg == "choose" {
            return Err(color_eyre::eyre::eyre!("Section menu not yet implemented"));
        } else {
            // It's a section name
            filter_sections.push(arg.clone());
        }
    }

    // Add tags from --tag option
    if let Some(tag_str) = tag {
        for t in tag_str.split(',') {
            filter_tags.push(t.trim().to_string());
        }
    }

    // Build filter options
    let mut filter_opts = FilterOptions {
        search,
        tags: filter_tags,
        sections: filter_sections,
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
    let mut entries = filter_entries(&doing_file, &filter_opts)?;

    // Sort entries
    match sort.as_str() {
        "asc" => entries.sort_by(|a, b| a.1.timestamp.cmp(&b.1.timestamp)),
        "desc" | _ => entries.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp)),
    }

    // Apply age filter (newest/oldest)
    if age == "oldest" {
        entries.reverse();
    }

    // Apply count limit
    if count > 0 {
        entries.truncate(count);
    }

    // Build display options
    let display_opts = DisplayOptions {
        times,
        duration,
        totals,
        hilite,
        search_query: filter_opts.search.clone(),
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