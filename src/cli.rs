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
}
