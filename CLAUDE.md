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
- **clap** (4.5): Command-line argument parsing with derive
- **chrono** (0.4): Date and time handling with serde support
- **chrono-english** (0.1): Natural language date parsing
- **serde** + **serde_json**: Configuration serialization
- **dirs** (5.0): Standard directory paths (for config/data files)
- **regex** (1.10): For parsing TaskPaper format and tag extraction
- **uuid** (1.10): Unique identifiers for entries

## Important Notes

- The app uses Rust edition 2024
- Terminal event handling is done through crossterm events
- The main loop runs while `App.running` is true
- Exit with Esc, q, or Ctrl-C

## Implementation Status

### ✅ Completed Phases

1. **Phase 1: Core Data Structures** 
   - ✅ Entry struct with timestamp, description, tags, section, UUID
   - ✅ Section support for categories (Currently, Archive, custom)
   - ✅ DoingFile struct representing the entire file
   - ✅ TaskPaper format: ` - YYYY-MM-DD HH:MM | description @tags <uuid>`

2. **Phase 2: CLI Framework**
   - ✅ Clap integration with subcommands
   - ✅ Command routing to handlers
   - ✅ Natural language date parsing with chrono-english

3. **Phase 3: Basic Commands**
   - ✅ `now` command - adds entries with full feature support:
     - Backdating with --back
     - Time ranges with --from
     - Parenthetical notes
     - Tag extraction from text
     - Multi-line notes with --ask
     - Finish last entry with --finish_last
   - ✅ `last` command - shows most recent entry
   - ✅ `recent` command - lists recent entries with:
     - Box drawing characters display
     - Duration calculation for done entries
     - 24-hour time format
     - Note display with proper indentation
   - ✅ `today` command - shows today's entries
   - ✅ `tui` command - Terminal UI with:
     - Entry list view with navigation
     - Detail view for entries
     - Note display
     - Elapsed time for done entries
     - Delete entries with 'd' key
     - Space bar to toggle @done status

4. **Phase 4: File Persistence**
   - ✅ TaskPaper format parser with UUID support
   - ✅ TaskPaper format writer
   - ✅ Automatic file loading/saving
   - ✅ Configuration support (~/.doingrc)

5. **Phase 5: Entry Completion & Time Tracking**
   - ✅ `done, did` - Add completed item with @done(date)
     - No argument finishes last entry
     - Supports --back, --at, --took, --from for time control
     - Archive with --archive
     - Remove @done with --remove
     - Note support with --note and --ask
   - ✅ `finish` - Mark last X entries as @done
     - Advanced filtering (search, tags, sections)
     - Auto mode to calculate times from next entry
     - NOT mode to invert filters
     - Update existing @done tags
   - ✅ `cancel` - End last X entries with no time tracked
     - Adds @done without timestamp
     - Same filtering as finish command
   - ✅ Duration tracking between start and @done times
     - Displayed in recent command and TUI
     - Format: HH:MM:SS
   - ✅ `delete` - Delete entries from the doing file
     - Confirmation prompt (bypass with --force)
     - Advanced filtering (search, tags, sections)
     - TUI integration with 'd' key

### 🚧 Next Implementation Phases

6. **Phase 6: Entry Management** ✅ COMPLETE
   - ✅ `again, resume` - Repeat last entry as new entry
     - Duplicates most recent entry with new timestamp
     - Removes @done tag from duplicated entry
     - Advanced filtering (search, tags, sections, NOT mode)
     - Backdating with --back
     - Note management (--note, --ask)
     - Section control with --in
   - ✅ `note` - Add a note to the last entry
     - Append, replace, or remove notes
     - Multi-line note support with --ask
     - Advanced filtering options
   - ✅ `tag` - Add tag(s) to last entry
     - Add/remove tags with wildcard support
     - Rename tags across entries
     - Date tagging with -d flag
     - Tag values with --value
   - ✅ `mark, flag` - Mark last entry as flagged
     - Add/remove @flagged tag
     - Date flagging with -d flag
     - Advanced filtering options
   - ✅ `reset, begin` - Reset the start time of an entry
     - Reset to current time or specified date
     - Resume entries by removing @done (default)
     - Option to keep @done with -n flag
     - --took option to set completion time
     - --from option for time ranges

7. **Phase 7: Viewing & Search** ✅ COMPLETE
   - ✅ `show` - List all entries with filtering
     - Comprehensive filtering by search, tags, sections, dates
     - Multiple output formats (JSON, CSV, HTML, Markdown, TaskPaper, Timeline)
     - Case sensitivity modes (smart/ignore/case-sensitive)
     - Boolean operators for combining filters (AND/OR/NOT/PATTERN)
     - Time range filtering with natural language support
   - ✅ `grep, search` - Search for entries
     - Regex and fuzzy matching support
     - Delete mode for removing matching entries
     - Highlights search matches in output
     - All filtering options from show command
   - ✅ `on` - List entries for a specific date
     - Natural language date parsing
     - Date range support with "to/through/-" separators
     - Time filtering within the date
   - ✅ `since` - List entries since a date
     - Always interprets dates as being in the past
     - All filtering options available
   - ✅ `yesterday` - List entries from yesterday
     - Shows entries from previous 24-hour period
     - Time filtering within the day

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
- 2025-07-28 20:41 | Meeting with client @flagged @importance(high) @done(2025-07-28 20:40) <17b6ef06cdab4396baf5ffb1786a0634>
- 2025-07-28 15:00 | Testing reset command <75eb23d271684c5786a08cc6a69c9c5d>
- 2025-07-28 09:03 | Testing the recent command display format @testing @done(2025-07-28 17:37) <ab17cb75e9754598a4dbfe9ca790f6e4>

Archive:
- 2025-07-28 17:27 | Archived completed task @done(2025-07-28 17:27) <2584ad82acb54469a6096ce1dff966a0>
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

### Completion & Time Tracking
```bash
# Mark entries as done
daily-log done              # Mark last entry as done with current time
daily-log did              # Alias for done
daily-log done "Quick task" # Create and immediately complete an entry
daily-log done --at "30 minutes ago"  # Set specific completion time
daily-log done --took 2h    # Set duration (adds @done timestamp calculated from start)
daily-log done --remove     # Remove @done tag from last entry

# Finish multiple entries
daily-log finish            # Mark last entry as @done
daily-log finish 3          # Mark last 3 entries as @done
daily-log finish --tag meeting --at "3pm"  # Finish entries with @meeting tag
daily-log finish --auto     # Auto-calculate completion times from next entry's start

# Cancel entries (mark done without time)
daily-log cancel            # Cancel last entry (adds @done without timestamp)
daily-log cancel 2          # Cancel last 2 entries
daily-log cancel --tag abandoned  # Cancel entries with @abandoned tag
```

### Entry Management
```bash
# Repeat/Resume entries
daily-log again             # Duplicate the most recent entry
daily-log resume            # Same as again (alias)
daily-log again --tag project1 --in Projects  # Find entry with @project1 and create in Projects section
daily-log resume --search "meeting" --back "2 hours ago"  # Find meeting entry and backdate
daily-log again --note "Continuing where I left off"  # Add a different note to the duplicated entry

# Add/modify notes
daily-log note "Additional details about the task"  # Add note to last entry
daily-log note --remove     # Remove note from last entry
daily-log note --ask        # Interactive multi-line note entry
daily-log note --search "bug fix" "Found the root cause"  # Add note to matching entry

# Tag management
daily-log tag @urgent       # Add @urgent tag to last entry
daily-log tag @bug @fixed   # Add multiple tags
daily-log tag @priority high -d  # Add tag with date
daily-log tag --remove @wip # Remove @wip tag
daily-log tag --rename @wip @done  # Rename all @wip tags to @done
daily-log tag @status --value "in progress"  # Add tag with value

# Mark/flag entries
daily-log mark              # Add @flagged tag to last entry
daily-log flag              # Alias for mark
daily-log mark -d           # Flag with current date/time
daily-log mark --remove     # Remove @flagged tag
daily-log mark --search "important" -c 5  # Flag 5 entries matching "important"

# Reset entry times
daily-log reset             # Reset last entry to current time and remove @done
daily-log begin             # Alias for reset
daily-log reset "3pm"       # Reset to specific time
daily-log reset -n          # Reset time but keep @done tag
daily-log reset --took 1h   # Reset and set completion time
daily-log reset --tag meeting "yesterday 2pm"  # Reset meeting entry to yesterday

# Delete entries
daily-log delete            # Delete the most recent entry (with confirmation)
daily-log delete 3          # Delete the 3 most recent entries
daily-log delete --search "test" --force  # Delete entries containing "test" without confirmation
daily-log delete --tag done # Delete entries with @done tag
```

## Future Command Examples (Not Yet Implemented)
```bash
# Advanced viewing
daily-log show              # List all entries with filtering
daily-log show @tag=priority          # Show entries with priority tag
daily-log show --from "2024-01-01"   # Show entries from date
daily-log grep "bug"        # Search for entries containing "bug"
daily-log search "/regex/"  # Search with regex
daily-log on 2024-01-15     # Show entries for specific date
daily-log since yesterday   # Show entries since yesterday
daily-log yesterday         # Show yesterday's entries

# Organization
daily-log sections          # List all sections
daily-log sections add "Projects"  # Add new section
daily-log archive           # Move completed entries to Archive
daily-log move 3 --to Archive  # Move 3 entries to Archive section
daily-log rotate            # Archive old entries to separate file
daily-log tags              # List all tags used

# Configuration & Views
daily-log config            # Edit configuration
daily-log view work         # Display custom "work" view
daily-log views             # List available views
daily-log open              # Open doing file in editor

# Advanced features
daily-log undo              # Undo last change
daily-log redo              # Redo undone change
daily-log select            # Interactive menu
daily-log meanwhile         # Handle @meanwhile tasks
daily-log import data.csv   # Import entries from file
```