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
    /// Add an entry to the "Currently" section
    Now {
        /// The task description
        #[arg(value_name = "TASK")]
        task: Vec<String>,

        /// Add tags in the format @tag or @tag(value)
        #[arg(short, long)]
        tag: Vec<String>,

        /// Add a note
        #[arg(short, long)]
        note: Option<String>,
    },

    /// Add an entry to the "Later" section
    Later {
        /// The task description
        #[arg(value_name = "TASK")]
        task: Vec<String>,

        /// Add tags in the format @tag or @tag(value)
        #[arg(short, long)]
        tag: Vec<String>,

        /// Add a note
        #[arg(short, long)]
        note: Option<String>,
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
