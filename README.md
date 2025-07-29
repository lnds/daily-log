# daily-log

A command-line time tracking tool written in Rust, inspired by Brett Terpstra's ["doing"](https://github.com/ttscoff/doing). Track what you're working on and what you've done with a simple, text-based system.

## About

This project started as a Rust clone of Brett Terpstra's excellent [doing](https://github.com/ttscoff/doing) time tracker, originally written in Ruby. While `daily-log` aims to be compatible with doing's file format and implements many of its core features, it is being developed independently in Rust for performance and cross-platform compatibility.

## Overview

`daily-log` helps you track your daily activities using a TaskPaper-formatted text file. It provides both a command-line interface (CLI) and a terminal user interface (TUI) for managing your work logs.

## Features

### Currently Implemented

- **Add entries** with `now` command - track what you're working on
- **View entries** with `recent`, `today`, `yesterday`, `last` commands
- **Mark tasks as done** with `done`/`did` commands
- **Delete entries** with `delete` command
- **Repeat entries** with `again`/`resume` commands
- **Add notes and tags** to entries
- **Organize with sections** - group related tasks together
- **Archive completed tasks** to keep your log clean
- **Search and filter** entries by text, tags, or date ranges
- **Terminal UI** for interactive browsing (press `t` to launch)

### Key Features

- **TaskPaper format** - human-readable text files that work with other apps
- **Time tracking** - automatic timestamps and duration calculations
- **Flexible tagging** - use `@tag` or `@tag(value)` syntax
- **Smart date parsing** - use natural language like "2 hours ago" or "yesterday at 3pm"
- **Multiple sections** - organize tasks by project or context
- **UUID tracking** - each entry has a unique identifier

## Installation

```bash
# Clone the repository
git clone https://github.com/lnds/daily-log.git
cd daily-log

# Build and install
cargo install --path .
```

## Usage

### Quick Start

```bash
# Add a new entry (what you're doing now)
doing now "Writing documentation"

# Add entry with tags
doing now "Fixing bug in parser @bug @urgent"

# Show recent entries
doing recent

# Show today's entries
doing today

# Mark last entry as done
doing done

# Add a completed entry
doing done "Deployed to production @deploy"
```

### Core Commands

#### `now` - Add a new entry

```bash
# Simple entry
doing now "Working on feature X"

# With tags
doing now "Code review @review @team"

# With note (using parentheses)
doing now "Meeting (discuss roadmap with team)"

# Backdate an entry
doing now "Lunch break" --back "30 minutes ago"

# Add to specific section
doing now "Project task" -s Projects

# Finish previous task and start new one
doing now "New task" -f

# Interactive mode (prompts for entry)
doing now
```

#### `done` / `did` - Mark tasks as completed

```bash
# Mark last entry as done
doing done

# Add a new completed entry
doing done "Finished report"

# Mark last entry done at specific time
doing done --at "2 hours ago"

# Specify duration
doing done --took 45m

# Archive completed entry
doing done --archive
```

#### `recent` - Show recent entries

```bash
# Show last 10 entries (default)
doing recent

# Show last 20 entries
doing recent 20

# Show entries from specific section
doing recent -s Projects
```

#### `today` - Show today's entries

```bash
# Show all of today's entries
doing today

# Show only from specific section
doing today -s Work
```

#### `delete` - Remove entries

```bash
# Delete last entry (with confirmation)
doing delete

# Delete last 3 entries
doing delete 3

# Force delete without confirmation
doing delete --force

# Delete entries matching search
doing delete --search "temp" --force

# Delete entries with specific tag
doing delete --tag "cancelled" --force
```

#### `again` / `resume` - Repeat the last entry

```bash
# Create a new entry with same description as last
doing again

# Resume with a different note
doing again --note "continuation of morning work"

# Resume in different section
doing again --in Projects
```

### Organization Commands

#### `sections` - Manage sections

```bash
# List all sections
doing sections

# Add a new section
doing sections -a "Personal"

# Remove a section (moves entries to Archive)
doing sections -r "OldProjects"
```

#### `archive` - Move entries between sections

```bash
# Archive all done entries from Currently
doing archive

# Archive entries with specific tag
doing archive @completed --to Done

# Archive entries older than a date
doing archive --before "last week"

# Archive from specific section
doing archive Projects --to "Archived Projects"
```

### Search and Filter Commands

#### `grep` / `search` - Search entries

```bash
# Search for text
doing grep "bug fix"

# Case-insensitive search
doing search -i "meeting"

# Search with regex
doing grep "/deploy.*production/"

# Search in specific sections
doing grep "feature" --section Development
```

#### `show` - Display entries with filters

```bash
# Show all entries
doing show all

# Show entries from date range
doing show all --from "last monday" --to "yesterday"

# Show only entries with specific tags
doing show all --tag "important"

# Show with duration totals
doing show all --totals
```

#### `tags` - List all tags

```bash
# List all tags
doing tags

# Show tag counts
doing tags --counts

# Sort by count
doing tags --counts --sort count

# Show tags from specific section
doing tags -s Projects
```

### Advanced Usage

#### Natural Language Dates

```bash
# Relative times
doing now "Task" --back "2 hours ago"
doing now "Task" --back "yesterday at 3pm"
doing done --at "30 minutes ago"

# Date ranges
doing now "Long task" --from "9am to 5pm"
doing show all --from "last monday" --to "today"
```

#### Tag Syntax

```bash
# Simple tags
doing now "Task @urgent @bug"

# Tags with values
doing now "Deploy @version(2.1.0) @environment(staging)"

# Automatic tag extraction
doing done "Fix @bug(ID-123) in @module(auth)"
```

#### Notes

```bash
# Parenthetical notes (extracted from description)
doing now "Meeting (with client about new features)"

# Note flag
doing now "Development work" --note "Refactoring auth module"

# Multi-line notes (interactive)
doing now "Research" --ask
```

### Terminal UI

Launch the interactive terminal interface:

```bash
doing tui
# or just
doing t
```

**TUI Controls:**

- `↑/↓` or `j/k` - Navigate entries
- `Enter` - View entry details
- `/` - Search
- `q` - Quit
- `Tab` - Switch between sections

## Configuration

Daily-log stores its data in a TaskPaper-formatted file:

- Default location: `~/.doing.taskpaper`
- UTF-8 text format, editable with any text editor

### File Format

```
Currently:
  - 2024-01-15 09:30 | Working on documentation @docs <uuid>
    This is a note about the documentation work
  - 2024-01-15 10:45 | Fixed login bug @bug @done(2024-01-15 11:30) <uuid>

Projects:
  - 2024-01-14 14:00 | Project planning meeting @meeting <uuid>
```

## Tips

1. **Use tags consistently** - Develop a tagging system (`@bug`, `@feature`, `@meeting`)
2. **Review regularly** - Use `doing today` and `doing yesterday` to track progress
3. **Archive completed work** - Keep your "Currently" section focused
4. **Add notes** - Use parentheses or `--note` for context
5. **Use sections** - Organize by project, client, or type of work

## Planned Features

Additional commands from the original "doing" tool that may be implemented:

- `meanwhile` - Pause and resume tasks
- `view` - Custom filtered views
- `undo/redo` - Undo last changes
- `import` - Import from other time tracking tools
- `config` - Configuration management
- Templates and reports

## Acknowledgments

This project is a Rust implementation inspired by [doing](https://github.com/ttscoff/doing) by Brett Terpstra. The original doing tool is a comprehensive command-line time tracker written in Ruby. We are grateful to Brett for creating such a useful tool and for making it open source.

If you're looking for a more feature-complete time tracking solution with extensive functionality, please check out the original [doing](https://github.com/ttscoff/doing) project.

## License

Copyright (c) Eduardo Díaz

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE

