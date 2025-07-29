use crate::display::{DisplayOptions, OutputFormat, TagSort, SortOrder, display_entries};
use crate::filtering::{FilterOptions, filter_entries, parse_date_filter, parse_date_range};
use crate::storage::{Config, parse_taskpaper};
use chrono::{Local, Duration};

pub fn handle_yesterday(
    after: Option<String>,
    before: Option<String>,
    _config_template: Option<String>,
    duration: bool,
    from: Option<String>,
    output: Option<String>,
    only_timed: bool,
    sections: Vec<String>,
    _save: Option<String>,
    times: bool,
    tag_order: String,
    tag_sort: String,
    _template: Option<String>,
    _title: Option<String>,
    totals: bool,
) -> color_eyre::Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let doing_file = parse_taskpaper(&doing_file_path)?;

    // Calculate yesterday's date range
    let now = Local::now();
    let yesterday = now - Duration::days(1);
    let yesterday_start = yesterday.date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap();
    let yesterday_end = yesterday.date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap();

    // Build filter options
    let mut filter_opts = FilterOptions {
        sections,
        from: Some((yesterday_start, Some(yesterday_end))),
        only_timed,
        ..Default::default()
    };

    // Apply time filters if specified
    if let Some(after_str) = after {
        let after_time = parse_date_filter(&after_str)?;
        // Apply time to yesterday's date
        let time = after_time.time();
        filter_opts.after = Some(
            yesterday.date_naive()
                .and_time(time)
                .and_local_timezone(Local)
                .unwrap()
        );
    }

    if let Some(before_str) = before {
        let before_time = parse_date_filter(&before_str)?;
        // Apply time to yesterday's date
        let time = before_time.time();
        filter_opts.before = Some(
            yesterday.date_naive()
                .and_time(time)
                .and_local_timezone(Local)
                .unwrap()
        );
    }

    if let Some(from_str) = from {
        // Parse time range and apply to yesterday
        let (from_time, to_time) = parse_date_range(&from_str)?;
        filter_opts.after = Some(
            yesterday.date_naive()
                .and_time(from_time.time())
                .and_local_timezone(Local)
                .unwrap()
        );
        if let Some(to) = to_time {
            filter_opts.before = Some(
                yesterday.date_naive()
                    .and_time(to.time())
                    .and_local_timezone(Local)
                    .unwrap()
            );
        }
    }

    // Filter entries
    let entries = filter_entries(&doing_file, &filter_opts)?;

    if entries.is_empty() {
        println!("No entries found for yesterday");
        return Ok(());
    }

    // Build display options
    let display_opts = DisplayOptions {
        times,
        duration,
        totals,
        hilite: false,
        search_query: None,
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