use crate::display::{DisplayOptions, OutputFormat, TagSort, SortOrder, display_entries};
use crate::filtering::{FilterOptions, CaseSensitivity, BoolOp, filter_entries, parse_date_filter, parse_date_range};
use crate::storage::{Config, parse_taskpaper};
use chrono::Local;

pub fn handle_on(
    date_string: String,
    after: Option<String>,
    before: Option<String>,
    bool_op: String,
    case: String,
    _config_template: Option<String>,
    duration: bool,
    from: Option<String>,
    not: bool,
    output: Option<String>,
    only_timed: bool,
    sections: Vec<String>,
    _save: Option<String>,
    search: Option<String>,
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
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let doing_file = parse_taskpaper(&doing_file_path)?;

    // Parse the date argument
    let (start_date, end_date) = parse_date_range(&date_string)?;

    // If no end date specified, use end of day
    let end_date = end_date.unwrap_or_else(|| {
        start_date.date_naive()
            .and_hms_opt(23, 59, 59)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
    });

    // Parse tags from --tag option
    let filter_tags = if let Some(tag_str) = tag {
        tag_str.split(',').map(|t| t.trim().to_string()).collect()
    } else {
        vec![]
    };

    // Build filter options
    let mut filter_opts = FilterOptions {
        search,
        tags: filter_tags,
        sections,
        from: Some((start_date, Some(end_date))),
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

    // Apply time filters if specified
    if let Some(after_str) = after {
        let after_time = parse_date_filter(&after_str)?;
        // If it's just a time, apply to the date range
        if after_str.contains(':') || after_str.contains("am") || after_str.contains("pm") {
            filter_opts.after = Some(after_time);
        }
    }

    if let Some(before_str) = before {
        let before_time = parse_date_filter(&before_str)?;
        // If it's just a time, apply to the date range
        if before_str.contains(':') || before_str.contains("am") || before_str.contains("pm") {
            filter_opts.before = Some(before_time);
        }
    }

    if let Some(from_str) = from {
        // Override with time range if specified
        let (from_time, to_time) = parse_date_range(&from_str)?;
        filter_opts.after = Some(from_time);
        if let Some(to) = to_time {
            filter_opts.before = Some(to);
        }
    }

    // Filter entries
    let entries = filter_entries(&doing_file, &filter_opts)?;

    if entries.is_empty() {
        println!("No entries found for {date_string}");
        return Ok(());
    }

    // Build display options
    let display_opts = DisplayOptions {
        times,
        duration,
        totals,
        hilite: false,
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