use crate::display::{DisplayOptions, OutputFormat, TagSort, SortOrder, display_entries};
use crate::filtering::{FilterOptions, CaseSensitivity, BoolOp, filter_entries, parse_date_filter};
use crate::storage::{Config, parse_taskpaper};

#[derive(Debug)]
pub struct SinceFilterOptions {
    pub date_string: String,
    pub bool_op: String,
    pub case: String,
    pub not: bool,
    pub only_timed: bool,
    pub sections: Vec<String>,
    pub search: Option<String>,
    pub tag: Option<String>,
    pub val: Vec<String>,
    pub exact: bool,
}

#[derive(Debug)]
pub struct SinceDisplayOptions {
    pub duration: bool,
    pub output: Option<String>,
    pub times: bool,
    pub tag_order: String,
    pub tag_sort: String,
    pub totals: bool,
}

#[derive(Debug)]
pub struct SinceConfigOptions {
    pub _config_template: Option<String>,
    pub _save: Option<String>,
    pub _template: Option<String>,
    pub _title: Option<String>,
}

pub fn handle_since(
    filter_opts: SinceFilterOptions,
    display_opts: SinceDisplayOptions,
    _config_opts: SinceConfigOptions,
) -> color_eyre::Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let doing_file = parse_taskpaper(&doing_file_path)?;

    // Parse the date argument
    let since_date = parse_date_filter(&filter_opts.date_string)?;

    // Parse tags from --tag option
    let filter_tags = if let Some(tag_str) = filter_opts.tag {
        tag_str.split(',').map(|t| t.trim().to_string()).collect()
    } else {
        vec![]
    };

    // Build filter options
    let filter_options = FilterOptions {
        search: filter_opts.search,
        tags: filter_tags,
        sections: filter_opts.sections,
        after: Some(since_date),
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

    // Filter entries
    let entries = filter_entries(&doing_file, &filter_options)?;

    if entries.is_empty() {
        println!("No entries found since {}", filter_opts.date_string);
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