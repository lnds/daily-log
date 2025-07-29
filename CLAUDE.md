# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust clone of Brett Terpstra's "doing" CLI tool - a command-line application for tracking what you're working on and what you've done. It provides both a CLI interface and a TUI (terminal UI) for managing daily work logs.

Brett Terpstra's "doing" is a comprehensive time tracking tool with over 30 commands for tracking status, recording time, and analyzing results. It stores data in TaskPaper-formatted text files and supports multiple sections/categories with flexible output formatting.

### Currently Implemented Features

**Core Commands:**
- `now` - Add entries with full feature set:
  - Interactive mode (run without arguments)
  - Parenthetical notes: `doing now "Task (this becomes a note)"`
  - Backdating: `--back "1 hour ago"` or `--back "yesterday 2pm"`
  - Time ranges: `--from "from 2pm to 3:30pm"`
  - Finish last entry: `-f` marks previous entry as @done
  - Multi-line notes: `--ask` for interactive note entry
  - Section support: `-s SectionName`
  - Tag extraction from entry text: `@tag` or `@tag(value)`
- `recent` - List recent entries with customizable count
- `today` - Show today's entries
- `last` - Show most recent entry with time ago

**Other Features:**
- Store data in doing's exact format: ` - YYYY-MM-DD HH:MM | description @tags <uuid>`
- Each entry has a unique UUID identifier
- Support for tags with values (@tag or @tag(value))
- Support for notes on entries (indented with 2 spaces)
- Multiple sections/categories (Currently, custom sections)
- Default behavior: no args shows recent, any text creates new entry

### Features from Original "doing" (To Be Implemented)
The original doing tool includes 30+ commands:

**Entry Management:**
- `done, did` - Add a completed item with @done(date). No argument finishes last entry
- `finish` - Mark last X entries as @done
- `again, resume` - Repeat last entry as new entry
- `meanwhile` - Finish any running @meanwhile tasks and optionally create a new one
- `cancel` - End last X entries with no time tracked
- `update` - Update doing to the latest version

**Viewing & Searching:**
- `show` - List all entries (with advanced filtering)
- `grep, search` - Search for entries
- `on` - List entries for a date
- `since` - List entries since a date
- `yesterday` - List entries from yesterday
- `view` - Display a user-created view
- `views` - List available custom views

**Organization:**
- `archive, move` - Move entries between sections
- `rotate` - Move entries to archive file
- `sections` - List, add, or remove sections in the Doing file
- `tags` - List all tags in the current Doing file
- `tag_dir` - Set the default tags for the current directory

**Editing & Annotation:**
- `note` - Add a note to the last entry
- `tag` - Add tag(s) to last entry
- `mark, flag` - Mark last entry as flagged
- `reset, begin` - Reset the start time of an entry
- `select` - Display an interactive menu to perform operations

**Advanced Features:**
- `import` - Import entries from an external source
- `template` - Output HTML, CSS, and Markdown (ERB) templates for customization
- `colors` - List available color variables for configuration templates and views
- `plugins` - List installed plugins
- `commands` - Enable and disable Doing commands
- `completion` - Generate shell completion scripts
- `config` - Edit the configuration file or output a value from it
- `open` - Open the "doing" file in an editor
- `undo` - Undo the last X changes to the Doing file
- `redo` - Redo an undo command
- `changes, changelog` - List recent changes in Doing

**Global Options:**
- Color output control
- Custom config file
- Debug/verbose output
- Pager support
- Auto-tagging control
- Notes inclusion
- Yes/no menu automation

## Design Recommendations

- Try to keep functions with 7 or less arguments

## Architecture

### Current Structure
- `src/main.rs`: Entry point that initializes the terminal and runs the app
- `src/app.rs`: Contains the `App` struct which manages application state and UI rendering

(Rest of the file remains unchanged)