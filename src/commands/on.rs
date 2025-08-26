use crate::display::{DisplayOptions, OutputFormat, SortOrder, TagSort, display_entries};
use crate::filtering::{
    BoolOp, CaseSensitivity, FilterOptions, filter_entries, parse_date_filter, parse_date_range,
};
use crate::storage::{Config, parse_taskpaper};
use chrono::Local;

#[derive(Debug)]
pub struct OnFilterOptions {
    pub date_string: String,
    pub after: Option<String>,
    pub before: Option<String>,
    pub bool_op: String,
    pub case: String,
    pub from: Option<String>,
    pub not: bool,
    pub only_timed: bool,
    pub sections: Vec<String>,
    pub search: Option<String>,
    pub tag: Option<String>,
    pub val: Vec<String>,
    pub exact: bool,
}

#[derive(Debug)]
pub struct OnDisplayOptions {
    pub duration: bool,
    pub output: Option<String>,
    pub times: bool,
    pub tag_order: String,
    pub tag_sort: String,
    pub totals: bool,
}

#[derive(Debug)]
pub struct OnConfigOptions {
    pub _config_template: Option<String>,
    pub _save: Option<String>,
    pub _template: Option<String>,
    pub _title: Option<String>,
}

pub fn handle_on(
    filter_opts: OnFilterOptions,
    display_opts: OnDisplayOptions,
    _config_opts: OnConfigOptions,
) -> color_eyre::Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let doing_file = parse_taskpaper(&doing_file_path)?;

    // Parse the date argument
    let (start_date, end_date) = parse_date_range(&filter_opts.date_string)?;

    // If no end date specified, use end of day
    let end_date = end_date.unwrap_or_else(|| {
        start_date
            .date_naive()
            .and_hms_opt(23, 59, 59)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
    });

    // Parse tags from --tag option
    let filter_tags = if let Some(tag_str) = filter_opts.tag {
        tag_str.split(',').map(|t| t.trim().to_string()).collect()
    } else {
        vec![]
    };

    // Build filter options
    let mut filter_options = FilterOptions {
        search: filter_opts.search,
        tags: filter_tags,
        sections: filter_opts.sections,
        from: Some((start_date, Some(end_date))),
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

    // Apply time filters if specified
    if let Some(after_str) = filter_opts.after {
        let after_time = parse_date_filter(&after_str)?;
        // If it's just a time, apply to the date range
        if after_str.contains(':') || after_str.contains("am") || after_str.contains("pm") {
            filter_options.after = Some(after_time);
        }
    }

    if let Some(before_str) = filter_opts.before {
        let before_time = parse_date_filter(&before_str)?;
        // If it's just a time, apply to the date range
        if before_str.contains(':') || before_str.contains("am") || before_str.contains("pm") {
            filter_options.before = Some(before_time);
        }
    }

    if let Some(from_str) = filter_opts.from {
        // Override with time range if specified
        let (from_time, to_time) = parse_date_range(&from_str)?;
        filter_options.after = Some(from_time);
        if let Some(to) = to_time {
            filter_options.before = Some(to);
        }
    }

    // Filter entries
    let entries = filter_entries(&doing_file, &filter_options)?;

    if entries.is_empty() {
        println!("No entries found for {}", filter_opts.date_string);
        return Ok(());
    }

    // Build display options
    let display_options = DisplayOptions {
        times: display_opts.times,
        duration: display_opts.duration,
        totals: display_opts.totals,
        hilite: false,
        search_query: filter_options.search.clone(),
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
