use crate::display::{DisplayOptions, OutputFormat, TagSort, SortOrder, display_entries};
use crate::filtering::{FilterOptions, CaseSensitivity, BoolOp, filter_entries, parse_date_filter};
use crate::storage::{Config, parse_taskpaper};

pub fn handle_since(
    date_string: String,
    bool_op: String,
    case: String,
    _config_template: Option<String>,
    duration: bool,
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
    let since_date = parse_date_filter(&date_string)?;

    // Parse tags from --tag option
    let filter_tags = if let Some(tag_str) = tag {
        tag_str.split(',').map(|t| t.trim().to_string()).collect()
    } else {
        vec![]
    };

    // Build filter options
    let filter_opts = FilterOptions {
        search,
        tags: filter_tags,
        sections,
        after: Some(since_date),
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

    // Filter entries
    let entries = filter_entries(&doing_file, &filter_opts)?;

    if entries.is_empty() {
        println!("No entries found since {}", date_string);
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