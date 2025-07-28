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
}
