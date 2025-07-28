use crate::commands::finish::handle_finish;

pub fn handle_cancel(
    count: usize,
    archive: bool,
    interactive: bool,
    not: bool,
    sections: Vec<String>,
    search: Option<String>,
    tag: Option<String>,
    unfinished: bool,
    exact: bool,
) -> color_eyre::Result<()> {
    // Cancel is an alias for finish --no-date
    // This adds @done tag without a timestamp
    handle_finish(
        count,
        archive,
        None,        // at
        false,       // auto
        None,        // back
        None,        // from
        interactive,
        not,
        false,       // remove
        sections,
        search,
        None,        // took
        tag,
        unfinished,
        false,       // update
        exact,
        false,       // date - this is the key difference, no date means no timestamp
    )
}