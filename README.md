# daily-log

A fast, simple command-line time tracking tool written in Rust. Track what you're working on and what you've done with an intuitive text-based system.

Inspired by Brett Terpstra's ["doing"](https://github.com/ttscoff/doing) but built from the ground up in Rust for speed and reliability.

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

### From Source

```bash
git clone https://github.com/lnds/daily-log.git
cd daily-log
cargo install --path .
```

### Check Installation

```bash
daily-log --version
```

## Usage

### Quick Start

```bash
# Add a new entry (what you're doing now)
daily-log now "Writing documentation"

# Add entry with tags
daily-log now "Fixing bug in parser @bug @urgent"

# Show recent entries
daily-log recent

# Show today's entries
daily-log today

# Mark last entry as done
daily-log done

# Add a completed entry
daily-log done "Deployed to production @deploy"
```

### Core Commands

#### `now` - Add a new entry

```bash
# Simple entry
daily-log now "Working on feature X"

# With tags
daily-log now "Code review @review @team"

# With note (using parentheses)
daily-log now "Meeting (discuss roadmap with team)"

# Backdate an entry
daily-log now "Lunch break" --back "30 minutes ago"

# Add to specific section
daily-log now "Project task" -s Projects

# Finish previous task and start new one
daily-log now "New task" -f

# Interactive mode (prompts for entry)
daily-log now
```

#### `done` / `did` - Mark tasks as completed

```bash
# Mark last entry as done
daily-log done

# Add a new completed entry
daily-log done "Finished report"

# Mark last entry done at specific time
daily-log done --at "2 hours ago"

# Specify duration
daily-log done --took 45m

# Archive completed entry
daily-log done --archive
```

#### `recent` - Show recent entries

```bash
# Show last 10 entries (default)
daily-log recent

# Show last 20 entries
daily-log recent 20

# Show entries from specific section
daily-log recent -s Projects
```

#### `today` - Show today's entries

```bash
# Show all of today's entries
daily-log today

# Show only from specific section
daily-log today -s Work
```

#### `delete` - Remove entries

```bash
# Delete last entry (with confirmation)
daily-log delete

# Delete last 3 entries
daily-log delete 3

# Force delete without confirmation
daily-log delete --force

# Delete entries matching search
daily-log delete --search "temp" --force

# Delete entries with specific tag
daily-log delete --tag "cancelled" --force
```

#### `again` / `resume` - Repeat the last entry

```bash
# Create a new entry with same description as last
daily-log again

# Resume with a different note
daily-log again --note "continuation of morning work"

# Resume in different section
daily-log again --in Projects
```

### Organization Commands

#### `sections` - Manage sections

```bash
# List all sections
daily-log sections

# Add a new section
daily-log sections -a "Personal"

# Remove a section (moves entries to Archive)
daily-log sections -r "OldProjects"
```

#### `archive` - Move entries between sections

```bash
# Archive all done entries from Currently
daily-log archive

# Archive entries with specific tag
daily-log archive @completed --to Done

# Archive entries older than a date
daily-log archive --before "last week"

# Archive from specific section
daily-log archive Projects --to "Archived Projects"
```

### Search and Filter Commands

#### `grep` / `search` - Search entries

```bash
# Search for text
daily-log grep "bug fix"

# Case-insensitive search
daily-log search -i "meeting"

# Search with regex
daily-log grep "/deploy.*production/"

# Search in specific sections
daily-log grep "feature" --section Development
```

#### `show` - Display entries with filters

```bash
# Show all entries
daily-log show all

# Show entries from date range
daily-log show all --from "last monday" --to "yesterday"

# Show only entries with specific tags
daily-log show all --tag "important"

# Show with duration totals
daily-log show all --totals
```

#### `tags` - List all tags

```bash
# List all tags
daily-log tags

# Show tag counts
daily-log tags --counts

# Sort by count
daily-log tags --counts --sort count

# Show tags from specific section
daily-log tags -s Projects
```

### Advanced Usage

#### Natural Language Dates

```bash
# Relative times
daily-log now "Task" --back "2 hours ago"
daily-log now "Task" --back "yesterday at 3pm"
daily-log done --at "30 minutes ago"

# Date ranges
daily-log now "Long task" --from "9am to 5pm"
daily-log show all --from "last monday" --to "today"
```

#### Tag Syntax

```bash
# Simple tags
daily-log now "Task @urgent @bug"

# Tags with values
daily-log now "Deploy @version(2.1.0) @environment(staging)"

# Automatic tag extraction
daily-log done "Fix @bug(ID-123) in @module(auth)"
```

#### Notes

```bash
# Parenthetical notes (extracted from description)
daily-log now "Meeting (with client about new features)"

# Note flag
daily-log now "Development work" --note "Refactoring auth module"

# Multi-line notes (interactive)
daily-log now "Research" --ask
```

### Terminal UI

Launch the interactive terminal interface:

```bash
daily-log tui
# or just
daily-log t
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

## Contributing

We welcome contributions! Please see [DEVELOPMENT.md](./DEVELOPMENT.md) for:

- Development environment setup
- Project architecture overview
- Testing guidelines
- Code style standards
- Pull request process

## Acknowledgments

This project is a Rust implementation inspired by [doing](https://github.com/ttscoff/doing) by Brett Terpstra. The original doing tool is a comprehensive command-line time tracker written in Ruby. We are grateful to Brett for creating such a useful tool and for making it open source.

If you're looking for a more feature-complete time tracking solution with extensive functionality, please check out the original [doing](https://github.com/ttscoff/doing) project.

## License

Copyright (c) Eduardo Díaz

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
