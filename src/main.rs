mod app;
mod cli;
mod commands;
mod models;
mod storage;

use crate::app::App;
use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Now { task, tag, note }) => {
            commands::handle_now(task, tag, note)?;
        }
        Some(Commands::Later { task, tag, note }) => {
            commands::handle_later(task, tag, note)?;
        }
        Some(Commands::Last) => {
            commands::handle_last()?;
        }
        Some(Commands::Recent { count, section }) => {
            commands::handle_recent(count, section)?;
        }
        Some(Commands::Today { section }) => {
            commands::handle_today(section)?;
        }
        Some(Commands::Tui) => {
            let terminal = ratatui::init();
            let result = App::new().run(terminal);
            ratatui::restore();
            result?;
        }
        None => {
            // If no command but task words provided, treat as "now" command
            if !cli.task.is_empty() {
                commands::handle_now(cli.task, vec![], None)?;
            } else {
                // If no command and no task words, show recent entries
                commands::handle_recent(10, None)?;
            }
        }
    }

    Ok(())
}
