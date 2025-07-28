# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust clone of Brett Terpstra's "doing" CLI tool - a command-line application for tracking what you're working on and what you've done. It provides both a CLI interface and a TUI (terminal UI) for managing daily work logs.

Brett Terpstra's "doing" is a comprehensive time tracking tool with over 30 commands for tracking status, recording time, and analyzing results. It stores data in TaskPaper-formatted text files and supports multiple sections/categories with flexible output formatting.

### Currently Implemented Features
- Track current work with `doing now` - Add entries to "Currently" section
- Add future tasks with `doing later` - Add entries to "Later" section
- View recent entries with `doing recent` - List recent entries with customizable count
- Show today's work with `doing today` - Filter entries from today
- Display last entry with `doing last` - Show most recent entry with time ago
- Store data in TaskPaper-formatted text files (~/.doing.taskpaper)
- Support for tags with values (@tag or @tag(value))
- Support for notes on entries
- Multiple sections/categories (Currently, Later, custom sections)

### Features from Original "doing" (To Be Implemented)
According to the wiki documentation, the original tool includes:

**Core Commands:**
- `doing done` - Mark entries as completed
- `doing finish` - Finish last entry and mark done
- `doing again/resume` - Repeat last entry
- `doing meanwhile` - Add entry between others
- `doing archive` - Archive completed entries
- `doing show` - Display entries with advanced filtering

**Time Tracking:**
- Time spans on entries (duration tracking)
- `doing on/off` - Start/stop time tracking
- Time adjustments and editing

**Advanced Features:**
- Custom views with filtering and formatting
- Multiple output formats (JSON, CSV, HTML, Markdown)
- Plugins system for extensibility
- Hooks for automation
- Autotagging based on patterns
- Batch editing capabilities
- Undo history
- Import/export functionality
- Day One integration
- Configuration file (~/.doingrc)
- Environment variables support
- Templates for custom formatting

**Data Management:**
- Multiple doing files
- Sections beyond Currently/Later
- Tag and search filtering with complex queries
- Chronological and reverse chronological sorting

## Architecture

### Current Structure
- `src/main.rs`: Entry point that initializes the terminal and runs the app
- `src/app.rs`: Contains the `App` struct which manages application state and UI rendering

### Planned Structure
- `src/main.rs`: Entry point with CLI argument parsing (clap)
- `src/cli.rs`: CLI command definitions and handlers
- `src/tui/`: Terminal UI module
  - `app.rs`: TUI application state and rendering
  - `widgets.rs`: Custom widgets for displaying entries
- `src/models/`: Core data structures
  - `entry.rs`: Work entry structure with timestamp, description, tags
  - `section.rs`: Section/category management
  - `doing_file.rs`: TaskPaper file format handling
- `src/storage/`: File persistence
  - `taskpaper.rs`: TaskPaper format parser/writer
  - `config.rs`: Configuration management
- `src/commands/`: Command implementations
  - `now.rs`, `later.rs`, `recent.rs`, `today.rs`, `last.rs`

## Development Commands

### Build and Run
```bash
# Check if code compiles
cargo check

# Build the project
cargo build

# Run the application
cargo run

# Build and run in release mode
cargo run --release
```

### Code Quality
```bash
# Run clippy for lint checks
cargo clippy

# Format code
cargo fmt

# Run tests
cargo test

# Run a specific test
cargo test test_name
```

## Key Dependencies

### Current
- **ratatui** (0.29.0): Terminal UI framework
- **crossterm** (0.28.1): Cross-platform terminal manipulation
- **color-eyre** (0.6.3): Enhanced error handling and reporting

### To Add
- **clap** (4.x): Command-line argument parsing
- **chrono**: Date and time handling
- **serde** + **serde_json**: Configuration serialization
- **dirs**: Standard directory paths (for config/data files)
- **regex**: For parsing TaskPaper format

## Important Notes

- The app uses Rust edition 2024
- Terminal event handling is done through crossterm events
- The main loop runs while `App.running` is true
- Exit with Esc, q, or Ctrl-C

## Implementation Status

### ✅ Completed Phases

1. **Phase 1: Core Data Structures** 
   - ✅ Entry struct with timestamp, description, tags, section
   - ✅ Section enum for categories (Currently, Later, custom)
   - ✅ DoingFile struct representing the entire file

2. **Phase 2: CLI Framework**
   - ✅ Clap integration with subcommands
   - ✅ Command routing to handlers

3. **Phase 3: Basic Commands**
   - ✅ `now` command - adds entries to Currently section
   - ✅ `later` command - adds entries to Later section
   - ✅ `last` command - shows most recent entry
   - ✅ `recent` command - lists recent entries
   - ✅ `today` command - shows today's entries

4. **Phase 4: File Persistence**
   - ✅ TaskPaper format parser
   - ✅ TaskPaper format writer
   - ✅ Automatic file loading/saving

### 🚧 Next Implementation Phases

5. **Phase 5: Time Tracking**
   - Add `done` command to mark entries complete
   - Add `finish` command to complete current task
   - Implement duration tracking
   - Add time span support (@from(time) @to(time))

6. **Phase 6: Advanced Commands**
   - `show` command with filtering options
   - `archive` command for completed tasks
   - `again`/`resume` to repeat last entry
   - `meanwhile` for inserting entries

7. **Phase 7: Configuration & Views**
   - Load/save ~/.doingrc configuration
   - Custom views with filters
   - Output format options (JSON, CSV, etc.)

8. **Phase 8: Advanced Features**
   - Tag filtering and search
   - Batch editing
   - Undo/redo support
   - Import/export functionality

## TaskPaper Format Notes

TaskPaper format basics:
- Projects end with a colon (:)
- Tasks start with a dash (-)
- Tags are @tag or @tag(value)
- Notes are indented lines without markers

Example:
```
Currently:
- Working on daily-log app @done(2024-01-15 14:30)
- Review PR for feature X @in_progress
    This needs careful attention to the API changes

Later:
- Update documentation @priority(high)
- Refactor authentication module
```

## Usage Examples

### Basic Usage
```bash
# Add a new entry to Currently section
daily-log now "Working on the README"
daily-log now "Fixing bug #123" --tag priority=high --note "This affects production"

# Add entry to Later section
daily-log later "Research new testing framework"

# View entries
daily-log                   # Show recent entries (default: 10) - DEFAULT COMMAND
daily-log last              # Show most recent entry
daily-log recent            # Show recent entries (default: 10)
daily-log recent -c 20      # Show 20 recent entries
daily-log today             # Show today's entries
daily-log today -s Later    # Show today's entries from Later section

# Implicit now command - just type your task
daily-log Working on documentation    # Same as: daily-log now "Working on documentation"
daily-log Fix bug in login system     # Same as: daily-log now "Fix bug in login system"
```

### With Tags and Notes
```bash
# Add entry with tags
daily-log now "Deploy to staging" --tag environment=staging --tag version=2.1.0

# Add entry with note
daily-log now "Meeting with client" --note "Discussed new features and timeline"
```

## Future Command Examples (Not Yet Implemented)
```bash
# Mark as done
daily-log done            # Mark last entry as done
daily-log finish          # Complete current task and add end time

# Time tracking
daily-log on "Working on feature"     # Start timing
daily-log off                         # Stop timing

# Advanced viewing
daily-log show @tag=priority          # Show entries with priority tag
daily-log show --from "2024-01-01"   # Show entries from date
daily-log archive                     # Archive completed entries

# Repeat/Resume
daily-log again           # Repeat the last entry
daily-log resume          # Resume the last entry
```