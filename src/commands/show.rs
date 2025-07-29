use crate::display::{DisplayOptions, OutputFormat, TagSort, SortOrder, display_entries};
use crate::filtering::{FilterOptions, CaseSensitivity, BoolOp, filter_entries, parse_date_filter, parse_date_range};
use crate::storage::{Config, parse_taskpaper};

#[derive(Debug)]
pub struct ShowFilterOptions {
    pub args: Vec<String>,
    pub age: String,
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
pub struct ShowDisplayOptions {
    pub count: usize,
    pub duration: bool,
    pub hilite: bool,
    pub output: Option<String>,
    pub sort: String,
    pub times: bool,
    pub tag_order: String,
    pub tag_sort: String,
    pub totals: bool,
}

#[derive(Debug)]
pub struct ShowUIOptions {
    pub interactive: bool,
    pub menu: bool,
    pub _editor: bool,
}

#[derive(Debug)]
pub struct ShowConfigOptions {
    pub _config_template: Option<String>,
    pub _save: Option<String>,
    pub _template: Option<String>,
    pub _title: Option<String>,
}

pub fn handle_show(
    filter_opts: ShowFilterOptions,
    display_opts: ShowDisplayOptions,
    ui_opts: ShowUIOptions,
    _config_opts: ShowConfigOptions,
) -> color_eyre::Result<()> {
    if ui_opts.interactive {
        return Err(color_eyre::eyre::eyre!("Interactive mode not yet implemented"));
    }

    if ui_opts.menu {
        return Err(color_eyre::eyre::eyre!("Menu mode not yet implemented"));
    }

    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let doing_file = parse_taskpaper(&doing_file_path)?;

    // Parse arguments for sections and tags
    let mut filter_sections = filter_opts.sections;
    let mut filter_tags = vec![];

    for arg in &filter_opts.args {
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
    if let Some(tag_str) = filter_opts.tag {
        for t in tag_str.split(',') {
            filter_tags.push(t.trim().to_string());
        }
    }

    // Build filter options
    let mut filter_options = FilterOptions {
        search: filter_opts.search,
        tags: filter_tags,
        sections: filter_sections,
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
    let mut entries = filter_entries(&doing_file, &filter_options)?;

    // Sort entries
    match display_opts.sort.as_str() {
        "asc" => entries.sort_by(|a, b| a.1.timestamp.cmp(&b.1.timestamp)),
        "desc" => entries.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp)),
        _ => entries.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp)),
    }

    // Apply age filter (newest/oldest)
    if filter_opts.age == "oldest" {
        entries.reverse();
    }

    // Apply count limit
    if display_opts.count > 0 {
        entries.truncate(display_opts.count);
    }

    // Build display options
    let display_options = DisplayOptions {
        times: display_opts.times,
        duration: display_opts.duration,
        totals: display_opts.totals,
        hilite: display_opts.hilite,
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