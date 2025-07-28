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
   - ✅ `last` command - shows most recent entry
   - ✅ `recent` command - lists recent entries
   - ✅ `today` command - shows today's entries

4. **Phase 4: File Persistence**
   - ✅ TaskPaper format parser
   - ✅ TaskPaper format writer
   - ✅ Automatic file loading/saving

### 🚧 Next Implementation Phases

5. **Phase 5: Entry Completion & Time Tracking**
   - `done, did` - Add completed item with @done(date), no arg finishes last
   - `finish` - Mark last X entries as @done
   - `cancel` - End last X entries with no time tracked
   - Duration tracking between start and @done times

6. **Phase 6: Entry Management**
   - `again, resume` - Repeat last entry as new entry
   - `note` - Add a note to the last entry
   - `tag` - Add tag(s) to last entry
   - `mark, flag` - Mark last entry as flagged
   - `reset, begin` - Reset the start time of an entry

7. **Phase 7: Viewing & Search**
   - `show` - List all entries with filtering
   - `grep, search` - Search for entries
   - `on` - List entries for a specific date
   - `since` - List entries since a date
   - `yesterday` - List entries from yesterday

8. **Phase 8: Organization & Archives**
   - `sections` - List, add, or remove sections
   - `archive, move` - Move entries between sections
   - `rotate` - Move entries to archive file
   - `tags` - List all tags in the current file

9. **Phase 9: Configuration & Views**
   - `config` - Edit configuration file
   - `view` - Display user-created views
   - `views` - List available custom views
   - Custom output formats (JSON, CSV, HTML)

10. **Phase 10: Advanced Features**
    - `undo/redo` - Undo/redo changes
    - `import` - Import from external sources
    - `select` - Interactive menu operations
    - `meanwhile` - Handle @meanwhile tasks
    - `open` - Open doing file in editor

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

Archive:
- Update documentation @priority(high) @done(2024-01-14 16:30)
- Refactor authentication module @done(2024-01-13 14:00)
```

## Usage Examples

### Basic Usage
```bash
# Add a new entry interactively
daily-log now

# Add entry with description
daily-log now "Working on the README"

# Add entry with parenthetical note
daily-log now "Deploy to staging (remember to update configs)"

# Add entry with tags in the text
daily-log now "Debugging issue @bug @priority(high)"

# Backdate an entry
daily-log now --back "2 hours ago" "Started debugging earlier"
daily-log now --back "yesterday 3pm" "Yesterday's meeting"

# Add timed entry with start and end
daily-log now --from "from 1pm to 2:30pm" "Team standup meeting"

# Finish last entry and start new one
daily-log now -f "Starting new task"

# Add entry with multi-line note
daily-log now --ask "Complex task"

# Add to specific section
daily-log now -s Archive "Old task that was completed"

# View entries
daily-log                   # Show recent entries (default: 10) - DEFAULT COMMAND
daily-log last              # Show most recent entry
daily-log recent            # Show recent entries (default: 10)
daily-log recent -c 20      # Show 20 recent entries
daily-log today             # Show today's entries
daily-log today -s Archive  # Show today's entries from Archive section

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