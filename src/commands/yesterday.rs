use crate::display::{DisplayOptions, OutputFormat, SortOrder, TagSort, display_entries};
use crate::filtering::{FilterOptions, filter_entries, parse_date_filter, parse_date_range};
use crate::storage::{Config, parse_taskpaper};
use chrono::{Duration, Local};

#[derive(Debug)]
pub struct YesterdayOptions {
    pub after: Option<String>,
    pub before: Option<String>,
    pub _config_template: Option<String>,
    pub duration: bool,
    pub from: Option<String>,
    pub output: Option<String>,
    pub only_timed: bool,
    pub sections: Vec<String>,
    pub _save: Option<String>,
    pub times: bool,
    pub tag_order: String,
    pub tag_sort: String,
    pub _template: Option<String>,
    pub _title: Option<String>,
    pub totals: bool,
}

pub fn handle_yesterday(opts: YesterdayOptions) -> color_eyre::Result<()> {
    let config = Config::load();
    let doing_file_path = config.doing_file_path();
    let doing_file = parse_taskpaper(&doing_file_path)?;

    // Calculate yesterday's date range
    let now = Local::now();
    let yesterday = now - Duration::days(1);
    let yesterday_start = yesterday
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap();
    let yesterday_end = yesterday
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap();

    // Build filter options
    let mut filter_opts = FilterOptions {
        sections: opts.sections.clone(),
        from: Some((yesterday_start, Some(yesterday_end))),
        only_timed: opts.only_timed,
        ..Default::default()
    };

    // Apply time filters if specified
    if let Some(after_str) = &opts.after {
        let after_time = parse_date_filter(after_str)?;
        // Apply time to yesterday's date
        let time = after_time.time();
        filter_opts.after = Some(
            yesterday
                .date_naive()
                .and_time(time)
                .and_local_timezone(Local)
                .unwrap(),
        );
    }

    if let Some(before_str) = &opts.before {
        let before_time = parse_date_filter(before_str)?;
        // Apply time to yesterday's date
        let time = before_time.time();
        filter_opts.before = Some(
            yesterday
                .date_naive()
                .and_time(time)
                .and_local_timezone(Local)
                .unwrap(),
        );
    }

    if let Some(from_str) = &opts.from {
        // Parse time range and apply to yesterday
        let (from_time, to_time) = parse_date_range(from_str)?;
        filter_opts.after = Some(
            yesterday
                .date_naive()
                .and_time(from_time.time())
                .and_local_timezone(Local)
                .unwrap(),
        );
        if let Some(to) = to_time {
            filter_opts.before = Some(
                yesterday
                    .date_naive()
                    .and_time(to.time())
                    .and_local_timezone(Local)
                    .unwrap(),
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
        times: opts.times,
        duration: opts.duration,
        totals: opts.totals,
        hilite: false,
        search_query: None,
        output_format: match opts.output.as_deref() {
            Some("json") => OutputFormat::Json,
            Some("csv") => OutputFormat::Csv,
            Some("markdown") => OutputFormat::Markdown,
            Some("html") => OutputFormat::Html,
            Some("taskpaper") => OutputFormat::TaskPaper,
            Some("timeline") => OutputFormat::Timeline,
            _ => OutputFormat::Default,
        },
        tag_sort: match opts.tag_sort.as_str() {
            "time" => TagSort::Time,
            _ => TagSort::Name,
        },
        tag_order: match opts.tag_order.as_str() {
            "desc" => SortOrder::Desc,
            _ => SortOrder::Asc,
        },
        section_filter: filter_opts.sections.clone(),
    };

    // Display entries
    display_entries(&entries, &display_opts)?;

    Ok(())
}
