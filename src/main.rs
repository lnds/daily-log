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
        Some(Commands::Delete {
            count,
            interactive,
            not,
            sections,
            search,
            tag,
            exact,
            force,
        }) => {
            commands::handle_delete(
                count,
                interactive,
                not,
                sections,
                search,
                tag,
                exact,
                force,
            )?;
        }
        Some(Commands::Again {
            noauto,
            ask,
            back,
            bool_op,
            case,
            editor,
            interactive,
            in_section,
            note,
            not,
            sections,
            search,
            tag,
            val,
            exact,
        }) => {
            commands::handle_again(
                noauto,
                ask,
                back,
                bool_op,
                case,
                editor,
                interactive,
                in_section,
                note,
                not,
                sections,
                search,
                tag,
                val,
                exact,
            )?;
        }
        Some(Commands::Tag {
            tags,
            autotag,
            bool_op,
            count,
            case,
            date,
            force,
            interactive,
            not,
            remove,
            regex,
            rename,
            sections,
            search,
            tag,
            unfinished,
            value,
            val,
            exact,
        }) => {
            commands::handle_tag(
                tags,
                autotag,
                bool_op,
                count,
                case,
                date,
                force,
                interactive,
                not,
                remove,
                regex,
                rename,
                sections,
                search,
                tag,
                unfinished,
                value,
                val,
                exact,
            )?;
        }
        Some(Commands::Note {
            note,
            ask,
            bool_op,
            case,
            editor,
            interactive,
            not,
            remove,
            sections,
            search,
            tag,
            val,
            exact,
        }) => {
            commands::handle_note(
                note,
                ask,
                bool_op,
                case,
                editor,
                interactive,
                not,
                remove,
                sections,
                search,
                tag,
                val,
                exact,
            )?;
        }
        Some(Commands::Resume {
            noauto,
            ask,
            back,
            bool_op,
            case,
            editor,
            interactive,
            in_section,
            note,
            not,
            sections,
            search,
            tag,
            val,
            exact,
        }) => {
            // Resume is an alias for again
            commands::handle_again(
                noauto,
                ask,
                back,
                bool_op,
                case,
                editor,
                interactive,
                in_section,
                note,
                not,
                sections,
                search,
                tag,
                val,
                exact,
            )?;
        }
        Some(Commands::Mark {
            bool_op,
            count,
            case,
            date,
            force,
            interactive,
            not,
            remove,
            sections,
            search,
            tag,
            unfinished,
            val,
            exact,
        }) => {
            commands::handle_mark(
                bool_op,
                count,
                case,
                date,
                force,
                interactive,
                not,
                remove,
                sections,
                search,
                tag,
                unfinished,
                val,
                exact,
            )?;
        }
        Some(Commands::Flag {
            bool_op,
            count,
            case,
            date,
            force,
            interactive,
            not,
            remove,
            sections,
            search,
            tag,
            unfinished,
            val,
            exact,
        }) => {
            // Flag is an alias for mark
            commands::handle_mark(
                bool_op,
                count,
                case,
                date,
                force,
                interactive,
                not,
                remove,
                sections,
                search,
                tag,
                unfinished,
                val,
                exact,
            )?;
        }
        Some(Commands::Reset {
            date_string,
            bool_op,
            case,
            from,
            interactive,
            no_resume,
            not,
            resume,
            sections,
            search,
            took,
            tag,
            val,
            exact,
        }) => {
            commands::handle_reset(
                date_string,
                bool_op,
                case,
                from,
                interactive,
                no_resume,
                not,
                resume,
                sections,
                search,
                took,
                tag,
                val,
                exact,
            )?;
        }
        Some(Commands::Begin {
            date_string,
            bool_op,
            case,
            from,
            interactive,
            no_resume,
            not,
            resume,
            sections,
            search,
            took,
            tag,
            val,
            exact,
        }) => {
            // Begin is an alias for reset
            commands::handle_reset(
                date_string,
                bool_op,
                case,
                from,
                interactive,
                no_resume,
                not,
                resume,
                sections,
                search,
                took,
                tag,
                val,
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
