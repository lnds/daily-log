use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "doing")]
#[command(about = "A command line tool for tracking what you're doing", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Task description (if provided without subcommand, creates a 'now' entry)
    #[arg(value_name = "TASK", trailing_var_arg = true)]
    pub task: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add an entry
    #[command(
        about = "Add an entry",
        long_about = "Record what you're starting now, or backdate the start time using natural language. A parenthetical at the end of the entry will be converted to a note. Run without arguments to create a new entry interactively. Run with --editor to create a new entry using your editor."
    )]
    Now {
        /// Entry text
        #[arg(value_name = "ENTRY")]
        entry: Vec<String>,

        /// Include a note
        #[arg(short = 'n', long = "note")]
        note: Option<String>,

        /// Backdate start date for new entry to date string [4pm|20m|2h|yesterday noon]
        #[arg(short = 'b', long = "back", alias = "started", alias = "since")]
        back: Option<String>,

        /// Section
        #[arg(short = 's', long = "section")]
        section: Option<String>,

        /// Timed entry, marks last entry in section as @done
        #[arg(short = 'f', long = "finish_last")]
        finish_last: bool,

        /// Set a start and optionally end time as a date range ("from 1pm to 2:30pm")
        #[arg(long = "from")]
        from: Option<String>,

        /// Edit entry with editor
        #[arg(short = 'e', long = "editor")]
        editor: bool,

        /// Prompt for note via multi-line input
        #[arg(long = "ask")]
        ask: bool,

        /// Exclude auto tags and default tags
        #[arg(short = 'X', long = "noauto")]
        noauto: bool,
    },


    /// Show the last entry
    Last,

    /// Show recent entries (default command)
    Recent {
        /// Number of entries to show
        #[arg(short, long, default_value = "10")]
        count: usize,

        /// Show entries from a specific section
        #[arg(short, long)]
        section: Option<String>,
    },

    /// Show entries from today
    Today {
        /// Show entries from a specific section
        #[arg(short, long)]
        section: Option<String>,
    },

    /// Launch the TUI interface
    Tui,

    /// Add a completed item with @done(date). No argument finishes last entry
    Done {
        /// Entry text
        #[arg(value_name = "ENTRY")]
        entry: Vec<String>,

        /// Include a note
        #[arg(short, long = "note")]
        note: Option<String>,

        /// Prompt for note via multi-line input
        #[arg(long = "ask")]
        ask: bool,

        /// Backdate start date for new entry to date string [4pm|20m|2h|yesterday noon]
        #[arg(short = 'b', long = "back", alias = "started", alias = "since")]
        back: Option<String>,

        /// Set finish date to specific date/time (natural language parsed, e.g. --at=1:30pm)
        #[arg(long = "at", alias = "finished")]
        at: Option<String>,

        /// Set completion date to start date plus interval (XX[mhd] or HH:MM)
        #[arg(short = 't', long = "took", alias = "for")]
        took: Option<String>,

        /// Start and end times as a date/time range (e.g., "1am to 8am")
        #[arg(long = "from")]
        from: Option<String>,

        /// Section
        #[arg(short = 's', long = "section")]
        section: Option<String>,

        /// Edit entry with editor
        #[arg(short = 'e', long = "editor")]
        editor: bool,

        /// Immediately archive the entry
        #[arg(short = 'a', long = "archive")]
        archive: bool,

        /// Remove @done tag
        #[arg(short = 'r', long = "remove")]
        remove: bool,

        /// Finish last entry not already marked @done
        #[arg(short = 'u', long = "unfinished")]
        unfinished: bool,

        /// Include date
        #[arg(long = "date", default_value = "true")]
        date: bool,

        /// Exclude auto tags and default tags
        #[arg(short = 'X', long = "noauto")]
        noauto: bool,
    },

    /// Mark last X entries as @done
    Finish {
        /// Number of entries to finish (default: 1)
        #[arg(value_name = "COUNT", default_value = "1")]
        count: usize,

        /// Archive entries
        #[arg(short = 'a', long = "archive")]
        archive: bool,

        /// Set finish date to specific date/time (natural language parsed)
        #[arg(long = "at", alias = "finished")]
        at: Option<String>,

        /// Auto-generate finish dates from next entry's start time
        #[arg(long = "auto")]
        auto: bool,

        /// Backdate completed date to date string
        #[arg(short = 'b', long = "back", alias = "started")]
        back: Option<String>,

        /// Start and end times as a date/time range
        #[arg(long = "from")]
        from: Option<String>,

        /// Select item(s) to finish from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Finish items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Remove @done tag
        #[arg(short = 'r', long = "remove")]
        remove: bool,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Set completion date to start date plus interval
        #[arg(short = 't', long = "took", alias = "for")]
        took: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Finish last entry (or entries) not already marked @done
        #[arg(short = 'u', long = "unfinished")]
        unfinished: bool,

        /// Overwrite existing @done tag with new date
        #[arg(long = "update")]
        update: bool,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,

        /// Include date
        #[arg(long = "date", default_value = "true")]
        date: bool,
    },

    /// Alias for done command
    Did {
        /// Entry text
        #[arg(value_name = "ENTRY")]
        entry: Vec<String>,

        /// Include a note
        #[arg(short, long = "note")]
        note: Option<String>,

        /// Prompt for note via multi-line input
        #[arg(long = "ask")]
        ask: bool,

        /// Backdate start date for new entry to date string [4pm|20m|2h|yesterday noon]
        #[arg(short = 'b', long = "back", alias = "started", alias = "since")]
        back: Option<String>,

        /// Set finish date to specific date/time (natural language parsed, e.g. --at=1:30pm)
        #[arg(long = "at", alias = "finished")]
        at: Option<String>,

        /// Set completion date to start date plus interval (XX[mhd] or HH:MM)
        #[arg(short = 't', long = "took", alias = "for")]
        took: Option<String>,

        /// Start and end times as a date/time range (e.g., "1am to 8am")
        #[arg(long = "from")]
        from: Option<String>,

        /// Section
        #[arg(short = 's', long = "section")]
        section: Option<String>,

        /// Edit entry with editor
        #[arg(short = 'e', long = "editor")]
        editor: bool,

        /// Immediately archive the entry
        #[arg(short = 'a', long = "archive")]
        archive: bool,

        /// Remove @done tag
        #[arg(short = 'r', long = "remove")]
        remove: bool,

        /// Finish last entry not already marked @done
        #[arg(short = 'u', long = "unfinished")]
        unfinished: bool,

        /// Include date
        #[arg(long = "date", default_value = "true")]
        date: bool,

        /// Exclude auto tags and default tags
        #[arg(short = 'X', long = "noauto")]
        noauto: bool,
    },

    /// End last X entries with no time tracked
    Cancel {
        /// Number of entries to cancel (default: 1)
        #[arg(value_name = "COUNT", default_value = "1")]
        count: usize,

        /// Archive entries
        #[arg(short = 'a', long = "archive")]
        archive: bool,

        /// Select item(s) to cancel from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Cancel items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Cancel last entry (or entries) not already marked @done
        #[arg(short = 'u', long = "unfinished")]
        unfinished: bool,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// Repeat last entry as new entry
    #[command(
        about = "Repeat last entry as new entry",
        long_about = "This command is designed to allow multiple time intervals to be created for an entry by duplicating it with a new start (and end, eventually) time"
    )]
    Again {
        /// Exclude auto tags and default tags
        #[arg(short = 'X', long = "noauto")]
        noauto: bool,

        /// Prompt for note via multi-line input
        #[arg(long = "ask")]
        ask: bool,

        /// Backdate start date for new entry to date string [4pm|20m|2h|yesterday noon]
        #[arg(short = 'b', long = "back", alias = "started", alias = "since")]
        back: Option<String>,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Edit entry with editor
        #[arg(short = 'e', long = "editor")]
        editor: bool,

        /// Select item to resume from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Add new entry to section (default: same section as repeated entry)
        #[arg(long = "in")]
        in_section: Option<String>,

        /// Include a note
        #[arg(short = 'n', long = "note")]
        note: Option<String>,

        /// Repeat items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Get last entry from a specific section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// Alias for again command
    Resume {
        /// Exclude auto tags and default tags
        #[arg(short = 'X', long = "noauto")]
        noauto: bool,

        /// Prompt for note via multi-line input
        #[arg(long = "ask")]
        ask: bool,

        /// Backdate start date for new entry to date string [4pm|20m|2h|yesterday noon]
        #[arg(short = 'b', long = "back", alias = "started", alias = "since")]
        back: Option<String>,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Edit entry with editor
        #[arg(short = 'e', long = "editor")]
        editor: bool,

        /// Select item to resume from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Add new entry to section (default: same section as repeated entry)
        #[arg(long = "in")]
        in_section: Option<String>,

        /// Include a note
        #[arg(short = 'n', long = "note")]
        note: Option<String>,

        /// Repeat items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Get last entry from a specific section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// Add tag(s) to last entry
    #[command(
        about = "Add tag(s) to last entry",
        long_about = "Add (or remove) tags from the last entry, or from multiple entries (with `--count`), entries matching a search (with `--search`), or entries containing another tag (with `--tag`). When removing tags with `-r`, wildcards are allowed (`*` to match multiple characters, `?` to match a single character). With `--regex`, regular expressions will be interpreted instead of wildcards. For all tag removals the match is case insensitive by default, but if the tag search string contains any uppercase letters, the match will become case sensitive automatically. Tag name arguments do not need to be prefixed with @."
    )]
    Tag {
        /// Tags to add/remove
        #[arg(value_name = "TAG", required = true)]
        tags: Vec<String>,

        /// Autotag entries based on autotag configuration
        #[arg(short = 'a', long = "autotag")]
        autotag: bool,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// How many recent entries to tag (0 for all)
        #[arg(short = 'c', long = "count", default_value = "1")]
        count: usize,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Include current date/time with tag
        #[arg(short = 'd', long = "date")]
        date: bool,

        /// Don't ask permission to tag all entries when count is 0
        #[arg(long = "force")]
        force: bool,

        /// Select item(s) to tag from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Tag items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Remove given tag(s)
        #[arg(short = 'r', long = "remove")]
        remove: bool,

        /// Interpret tag string as regular expression (with --remove)
        #[arg(long = "regex")]
        regex: bool,

        /// Replace existing tag with tag argument
        #[arg(long = "rename")]
        rename: Option<String>,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Tag last entry (or entries) not marked @done
        #[arg(short = 'u', long = "unfinished")]
        unfinished: bool,

        /// Include a value, e.g. @tag(value)
        #[arg(short = 'v', long = "value")]
        value: Option<String>,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// Add a note to the last entry
    #[command(
        about = "Add a note to the last entry",
        long_about = "If -r is provided with no other arguments, the last note is removed. If new content is specified through arguments or STDIN, any previous note will be replaced with the new one. Use -e to load the last entry in a text editor where you can append a note."
    )]
    Note {
        /// Note text to add
        #[arg(value_name = "NOTE_TEXT")]
        note: Vec<String>,

        /// Prompt for note via multi-line input
        #[arg(long = "ask")]
        ask: bool,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Edit entry with editor
        #[arg(short = 'e', long = "editor")]
        editor: bool,

        /// Select item for new note from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Note items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Replace/Remove last entry's note (default append)
        #[arg(short = 'r', long = "remove")]
        remove: bool,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// Delete entries from the doing file
    Delete {
        /// Number of entries to delete (default: 1)
        #[arg(value_name = "COUNT", default_value = "1")]
        count: usize,

        /// Select item(s) to delete from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Delete items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,

        /// Force deletion without confirmation
        #[arg(short = 'f', long = "force")]
        force: bool,
    },

    /// Mark last entry as flagged
    #[command(
        about = "Mark last entry as flagged",
        long_about = "Mark the last entry as @flagged. If provided with arguments, mark entries matching the given search filter (search terms, `--tag`, `--search`). The argument can be a count of recent entries, an index for an entry, or a search string. Use `--remove` to unflag."
    )]
    Mark {
        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// How many recent entries to flag (0 for all)
        #[arg(short = 'c', long = "count", default_value = "1")]
        count: usize,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Include current date/time with flag
        #[arg(short = 'd', long = "date")]
        date: bool,

        /// Don't ask permission to flag all entries when count is 0
        #[arg(long = "force")]
        force: bool,

        /// Select item(s) to flag from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Flag items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Remove flag
        #[arg(short = 'r', long = "remove")]
        remove: bool,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Flag last entry (or entries) not marked @done
        #[arg(short = 'u', long = "unfinished")]
        unfinished: bool,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// Alias for mark command
    Flag {
        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// How many recent entries to flag (0 for all)
        #[arg(short = 'c', long = "count", default_value = "1")]
        count: usize,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Include current date/time with flag
        #[arg(short = 'd', long = "date")]
        date: bool,

        /// Don't ask permission to flag all entries when count is 0
        #[arg(long = "force")]
        force: bool,

        /// Select item(s) to flag from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Flag items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Remove flag
        #[arg(short = 'r', long = "remove")]
        remove: bool,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Flag last entry (or entries) not marked @done
        #[arg(short = 'u', long = "unfinished")]
        unfinished: bool,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// Reset the start time of an entry
    #[command(
        about = "Reset the start time of an entry",
        long_about = "Update the start time of the last entry or the last entry matching a tag/search filter. If no argument is provided, the start time will be reset to the current time. If a date string is provided as an argument, the start time will be set to the parsed result."
    )]
    Reset {
        /// Date string to set as new start time
        #[arg(value_name = "DATE_STRING")]
        date_string: Option<String>,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Start and end times as a date/time range (e.g., "1am to 8am")
        #[arg(long = "from")]
        from: Option<String>,

        /// Select from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Change start date but do not remove @done (shortcut for --no-resume)
        #[arg(short = 'n', conflicts_with = "resume")]
        no_resume: bool,

        /// Reset items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Resume entry (remove @done)
        #[arg(short = 'r', long = "resume", default_value = "true")]
        resume: bool,

        /// Limit search to section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Set completion date to start date plus interval (XX[mhd] or HH:MM)
        #[arg(short = 't', long = "took", alias = "for")]
        took: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// Alias for reset command
    Begin {
        /// Date string to set as new start time
        #[arg(value_name = "DATE_STRING")]
        date_string: Option<String>,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Start and end times as a date/time range (e.g., "1am to 8am")
        #[arg(long = "from")]
        from: Option<String>,

        /// Select from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Change start date but do not remove @done (shortcut for --no-resume)
        #[arg(short = 'n', conflicts_with = "resume")]
        no_resume: bool,

        /// Reset items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Resume entry (remove @done)
        #[arg(short = 'r', long = "resume", default_value = "true")]
        resume: bool,

        /// Limit search to section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Set completion date to start date plus interval (XX[mhd] or HH:MM)
        #[arg(short = 't', long = "took", alias = "for")]
        took: Option<String>,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching (case sensitive)
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// List all entries
    #[command(
        about = "List all entries",
        long_about = "The argument can be a section name, @tag(s) or both. \"pick\" or \"choose\" as an argument will offer a section menu."
    )]
    Show {
        /// Section or @tags to show
        #[arg(value_name = "SECTION|@TAGS")]
        args: Vec<String>,

        /// Age (oldest|newest)
        #[arg(short = 'a', long = "age", default_value = "newest")]
        age: String,

        /// Show entries newer than date
        #[arg(long = "after")]
        after: Option<String>,

        /// Show entries older than date
        #[arg(long = "before")]
        before: Option<String>,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// Max count to show
        #[arg(short = 'c', long = "count", default_value = "0")]
        count: usize,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Output using a template from configuration
        #[arg(long = "config_template")]
        config_template: Option<String>,

        /// Show elapsed time on entries without @done tag
        #[arg(long = "duration")]
        duration: bool,

        /// Edit matching entries with editor
        #[arg(short = 'e', long = "editor")]
        editor: bool,

        /// Date range to show
        #[arg(long = "from")]
        from: Option<String>,

        /// Highlight search matches in output
        #[arg(long = "hilite")]
        hilite: bool,

        /// Select from a menu of matching entries
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Select section or tag to display from a menu
        #[arg(short = 'm', long = "menu")]
        menu: bool,

        /// Show items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Output format
        #[arg(short = 'o', long = "output")]
        output: Option<String>,

        /// Only show items with recorded time intervals
        #[arg(long = "only_timed")]
        only_timed: bool,

        /// Only show entries within section
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Save all current options as a new view
        #[arg(long = "save")]
        save: Option<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Sort order (asc/desc)
        #[arg(long = "sort", default_value = "desc")]
        sort: String,

        /// Show time intervals on @done tasks
        #[arg(short = 't', long = "times", default_value = "true")]
        times: bool,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Tag sort direction (asc|desc)
        #[arg(long = "tag_order", default_value = "asc")]
        tag_order: String,

        /// Sort tags by (name|time)
        #[arg(long = "tag_sort", default_value = "name")]
        tag_sort: String,

        /// Override output format with template
        #[arg(long = "template")]
        template: Option<String>,

        /// Title string for output formats
        #[arg(long = "title")]
        title: Option<String>,

        /// Show time totals at end
        #[arg(long = "totals")]
        totals: bool,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// Search for entries
    #[command(
        about = "Search for entries",
        long_about = "Search all sections for entries matching text or regular expression"
    )]
    Grep {
        /// Search pattern
        #[arg(value_name = "SEARCH_PATTERN")]
        pattern: String,

        /// Search entries newer than date
        #[arg(long = "after")]
        after: Option<String>,

        /// Search entries older than date
        #[arg(long = "before")]
        before: Option<String>,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Output using a template from configuration
        #[arg(long = "config_template")]
        config_template: Option<String>,

        /// Delete matching entries
        #[arg(short = 'd', long = "delete")]
        delete: bool,

        /// Show elapsed time on entries without @done tag
        #[arg(long = "duration")]
        duration: bool,

        /// Edit matching entries with editor
        #[arg(short = 'e', long = "editor")]
        editor: bool,

        /// Date range to search
        #[arg(long = "from")]
        from: Option<String>,

        /// Highlight search matches in output
        #[arg(long = "hilite")]
        hilite: bool,

        /// Display an interactive menu of results
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Search items that *don't* match
        #[arg(long = "not")]
        not: bool,

        /// Output format
        #[arg(short = 'o', long = "output")]
        output: Option<String>,

        /// Only show items with recorded time intervals
        #[arg(long = "only_timed")]
        only_timed: bool,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Save all current options as a new view
        #[arg(long = "save")]
        save: Option<String>,

        /// Show time intervals on @done tasks
        #[arg(short = 't', long = "times", default_value = "true")]
        times: bool,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Tag sort direction (asc|desc)
        #[arg(long = "tag_order", default_value = "asc")]
        tag_order: String,

        /// Sort tags by (name|time)
        #[arg(long = "tag_sort", default_value = "name")]
        tag_sort: String,

        /// Override output format with template
        #[arg(long = "template")]
        template: Option<String>,

        /// Title string for output formats
        #[arg(long = "title")]
        title: Option<String>,

        /// Show time totals at end
        #[arg(long = "totals")]
        totals: bool,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact string matching
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// Alias for grep
    Search {
        /// Search pattern
        #[arg(value_name = "SEARCH_PATTERN")]
        pattern: String,

        /// Search entries newer than date
        #[arg(long = "after")]
        after: Option<String>,

        /// Search entries older than date
        #[arg(long = "before")]
        before: Option<String>,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Output using a template from configuration
        #[arg(long = "config_template")]
        config_template: Option<String>,

        /// Delete matching entries
        #[arg(short = 'd', long = "delete")]
        delete: bool,

        /// Show elapsed time on entries without @done tag
        #[arg(long = "duration")]
        duration: bool,

        /// Edit matching entries with editor
        #[arg(short = 'e', long = "editor")]
        editor: bool,

        /// Date range to search
        #[arg(long = "from")]
        from: Option<String>,

        /// Highlight search matches in output
        #[arg(long = "hilite")]
        hilite: bool,

        /// Display an interactive menu of results
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        /// Search items that *don't* match
        #[arg(long = "not")]
        not: bool,

        /// Output format
        #[arg(short = 'o', long = "output")]
        output: Option<String>,

        /// Only show items with recorded time intervals
        #[arg(long = "only_timed")]
        only_timed: bool,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Save all current options as a new view
        #[arg(long = "save")]
        save: Option<String>,

        /// Show time intervals on @done tasks
        #[arg(short = 't', long = "times", default_value = "true")]
        times: bool,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Tag sort direction (asc|desc)
        #[arg(long = "tag_order", default_value = "asc")]
        tag_order: String,

        /// Sort tags by (name|time)
        #[arg(long = "tag_sort", default_value = "name")]
        tag_sort: String,

        /// Override output format with template
        #[arg(long = "template")]
        template: Option<String>,

        /// Title string for output formats
        #[arg(long = "title")]
        title: Option<String>,

        /// Show time totals at end
        #[arg(long = "totals")]
        totals: bool,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact string matching
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// List entries for a date
    #[command(
        about = "List entries for a date",
        long_about = "Date argument can be natural language. Use \"to\" or \"through\" between two dates for a range."
    )]
    On {
        /// Date string
        #[arg(value_name = "DATE_STRING")]
        date_string: String,

        /// View entries after specified time
        #[arg(long = "after")]
        after: Option<String>,

        /// View entries before specified time
        #[arg(long = "before")]
        before: Option<String>,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Output using a template from configuration
        #[arg(long = "config_template")]
        config_template: Option<String>,

        /// Show elapsed time on entries without @done tag
        #[arg(long = "duration")]
        duration: bool,

        /// Time range to show
        #[arg(long = "from")]
        from: Option<String>,

        /// Show items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Output format
        #[arg(short = 'o', long = "output")]
        output: Option<String>,

        /// Only show items with recorded time intervals
        #[arg(long = "only_timed")]
        only_timed: bool,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Save all current options as a new view
        #[arg(long = "save")]
        save: Option<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Show time intervals on @done tasks
        #[arg(short = 't', long = "times", default_value = "true")]
        times: bool,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Tag sort direction (asc|desc)
        #[arg(long = "tag_order", default_value = "asc")]
        tag_order: String,

        /// Sort tags by (name|time)
        #[arg(long = "tag_sort", default_value = "name")]
        tag_sort: String,

        /// Override output format with template
        #[arg(long = "template")]
        template: Option<String>,

        /// Title string for output formats
        #[arg(long = "title")]
        title: Option<String>,

        /// Show time totals at end
        #[arg(long = "totals")]
        totals: bool,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// List entries since a date
    #[command(
        about = "List entries since a date",
        long_about = "Date argument can be natural language and are always interpreted as being in the past."
    )]
    Since {
        /// Date string
        #[arg(value_name = "DATE_STRING")]
        date_string: String,

        /// Boolean used to combine multiple tags (AND|OR|NOT)
        #[arg(long = "bool", default_value = "pattern")]
        bool_op: String,

        /// Case sensitivity for search string matching [(c)ase-sensitive, (i)gnore, (s)mart]
        #[arg(long = "case", default_value = "smart")]
        case: String,

        /// Output using a template from configuration
        #[arg(long = "config_template")]
        config_template: Option<String>,

        /// Show elapsed time on entries without @done tag
        #[arg(long = "duration")]
        duration: bool,

        /// Since items that *don't* match search/tag filters
        #[arg(long = "not")]
        not: bool,

        /// Output format
        #[arg(short = 'o', long = "output")]
        output: Option<String>,

        /// Only show items with recorded time intervals
        #[arg(long = "only_timed")]
        only_timed: bool,

        /// Section (may be used more than once)
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Save all current options as a new view
        #[arg(long = "save")]
        save: Option<String>,

        /// Filter entries using a search query
        #[arg(long = "search")]
        search: Option<String>,

        /// Show time intervals on @done tasks
        #[arg(short = 't', long = "times", default_value = "true")]
        times: bool,

        /// Filter entries by tag
        #[arg(long = "tag")]
        tag: Option<String>,

        /// Tag sort direction (asc|desc)
        #[arg(long = "tag_order", default_value = "asc")]
        tag_order: String,

        /// Sort tags by (name|time)
        #[arg(long = "tag_sort", default_value = "name")]
        tag_sort: String,

        /// Override output format with template
        #[arg(long = "template")]
        template: Option<String>,

        /// Title string for output formats
        #[arg(long = "title")]
        title: Option<String>,

        /// Show time totals at end
        #[arg(long = "totals")]
        totals: bool,

        /// Perform a tag value query
        #[arg(long = "val")]
        val: Vec<String>,

        /// Force exact search string matching
        #[arg(short = 'x', long = "exact")]
        exact: bool,
    },

    /// List entries from yesterday
    #[command(
        about = "List entries from yesterday",
        long_about = "Show only entries with start times within the previous 24 hour period."
    )]
    Yesterday {
        /// View entries after specified time
        #[arg(long = "after")]
        after: Option<String>,

        /// View entries before specified time
        #[arg(long = "before")]
        before: Option<String>,

        /// Output using a template from configuration
        #[arg(long = "config_template")]
        config_template: Option<String>,

        /// Show elapsed time on entries without @done tag
        #[arg(long = "duration")]
        duration: bool,

        /// Time range to show
        #[arg(long = "from")]
        from: Option<String>,

        /// Output format
        #[arg(short = 'o', long = "output")]
        output: Option<String>,

        /// Only show items with recorded time intervals
        #[arg(long = "only_timed")]
        only_timed: bool,

        /// Specify a section
        #[arg(short = 's', long = "section")]
        sections: Vec<String>,

        /// Save all current options as a new view
        #[arg(long = "save")]
        save: Option<String>,

        /// Show time intervals on @done tasks
        #[arg(short = 't', long = "times", default_value = "true")]
        times: bool,

        /// Tag sort direction (asc|desc)
        #[arg(long = "tag_order", default_value = "asc")]
        tag_order: String,

        /// Sort tags by (name|time)
        #[arg(long = "tag_sort", default_value = "name")]
        tag_sort: String,

        /// Override output format with template
        #[arg(long = "template")]
        template: Option<String>,

        /// Title string for output formats
        #[arg(long = "title")]
        title: Option<String>,

        /// Show time totals at end
        #[arg(long = "totals")]
        totals: bool,
    },
}
