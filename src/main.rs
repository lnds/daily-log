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
        Some(Commands::Now { 
            entry, 
            note, 
            back, 
            section, 
            finish_last, 
            from, 
            editor, 
            ask, 
            noauto 
        }) => {
            commands::handle_now(
                entry, 
                note, 
                back, 
                section, 
                finish_last, 
                from, 
                editor, 
                ask, 
                noauto
            )?;
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
        Some(Commands::Done {
            entry,
            note,
            ask,
            back,
            at,
            took,
            from,
            section,
            editor,
            archive,
            remove,
            unfinished,
            date,
            noauto,
        }) => {
            commands::handle_done(
                entry,
                note,
                ask,
                back,
                at,
                took,
                from,
                section,
                editor,
                archive,
                remove,
                unfinished,
                date,
                noauto,
            )?;
        }
        Some(Commands::Finish {
            count,
            archive,
            at,
            auto,
            back,
            from,
            interactive,
            not,
            remove,
            sections,
            search,
            took,
            tag,
            unfinished,
            update,
            exact,
            date,
        }) => {
            commands::handle_finish(
                count,
                archive,
                at,
                auto,
                back,
                from,
                interactive,
                not,
                remove,
                sections,
                search,
                took,
                tag,
                unfinished,
                update,
                exact,
                date,
            )?;
        }
        Some(Commands::Did {
            entry,
            note,
            ask,
            back,
            at,
            took,
            from,
            section,
            editor,
            archive,
            remove,
            unfinished,
            date,
            noauto,
        }) => {
            // Did is an alias for done
            commands::handle_done(
                entry,
                note,
                ask,
                back,
                at,
                took,
                from,
                section,
                editor,
                archive,
                remove,
                unfinished,
                date,
                noauto,
            )?;
        }
        Some(Commands::Cancel {
            count,
            archive,
            interactive,
            not,
            sections,
            search,
            tag,
            unfinished,
            exact,
        }) => {
            commands::handle_cancel(
                count,
                archive,
                interactive,
                not,
                sections,
                search,
                tag,
                unfinished,
                exact,
            )?;
        }
        None => {
            // If no command but task words provided, treat as "now" command
            if !cli.task.is_empty() {
                commands::handle_now(
                    cli.task, 
                    None, 
                    None, 
                    None, 
                    false, 
                    None, 
                    false, 
                    false, 
                    false
                )?;
            } else {
                // If no command and no task words, show recent entries
                commands::handle_recent(10, None)?;
            }
        }
    }

    Ok(())
}
