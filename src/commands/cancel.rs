use crate::commands::finish::{handle_finish, FinishOptions};

#[derive(Debug)]
pub struct CancelOptions {
    pub count: usize,
    pub archive: bool,
    pub interactive: bool,
    pub not: bool,
    pub sections: Vec<String>,
    pub search: Option<String>,
    pub tag: Option<String>,
    pub unfinished: bool,
    pub exact: bool,
}

pub fn handle_cancel(opts: CancelOptions) -> color_eyre::Result<()> {
    // Cancel is an alias for finish --no-date
    // This adds @done tag without a timestamp
    handle_finish(FinishOptions {
        count: opts.count,
        archive: opts.archive,
        at: None,
        auto: false,
        back: None,
        from: None,
        interactive: opts.interactive,
        not: opts.not,
        remove: false,
        sections: opts.sections,
        search: opts.search,
        took: None,
        tag: opts.tag,
        unfinished: opts.unfinished,
        update: false,
        exact: opts.exact,
        date: false, // this is the key difference, no date means no timestamp
    })
}