# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust clone of Brett Terpstra's "doing" CLI tool - a command-line application for tracking what you're working on and what you've done. It provides both a CLI interface and a TUI (terminal UI) for managing daily work logs.

### Core Features (Planned)
- Track current work with `doing now`
- Add future tasks with `doing later`
- View recent entries with `doing recent`
- Show today's work with `doing today`
- Display last entry with `doing last`
- Store data in TaskPaper-formatted text files
- Support multiple sections/categories

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

## Implementation Plan

1. **Phase 1: Core Data Structures**
   - Research and understand TaskPaper format
   - Create Entry struct with timestamp, description, tags, section
   - Create Section enum for categories (Currently, Later, etc.)
   - Create DoingFile struct to represent the entire file

2. **Phase 2: CLI Framework**
   - Add clap dependency and create CLI structure
   - Define subcommands: now, later, recent, today, last
   - Route commands to appropriate handlers

3. **Phase 3: Basic Commands**
   - Implement `now` command to add entries
   - Implement `last` command to show most recent
   - Implement `recent` command to list entries
   - Implement `today` command for today's entries
   - Implement `later` command for future tasks

4. **Phase 4: File Persistence**
   - Implement TaskPaper format parser
   - Implement TaskPaper format writer
   - Add file loading on startup
   - Add file saving after modifications

5. **Phase 5: Enhanced Features**
   - Add configuration file support
   - Add tag support and filtering
   - Add time tracking features
   - Update TUI to display entries

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