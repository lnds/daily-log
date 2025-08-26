# Development Guide

This document contains information for developers who want to contribute to the daily-log project.

## AI-Assisted Development Notice

🤖 **This project uses [Claude Code](https://claude.ai/code) for development assistance.** 

We leverage AI-powered development tools to help with:
- Code generation and refactoring
- Architecture decisions and planning  
- Test writing and documentation
- Bug fixing and optimization

This approach allows us to maintain high code quality while accelerating development. All AI-generated code is reviewed and tested before being merged.

## Development Environment Setup

### Prerequisites

- Rust 2024 edition or later
- Git

### Clone and Build

```bash
git clone https://github.com/lnds/daily-log.git
cd daily-log
cargo build
```

## Project Architecture

This is a Rust clone of Brett Terpstra's "doing" CLI tool - a command-line application for tracking what you're working on and what you've done. It provides both a CLI interface and a TUI (terminal UI) for managing daily work logs.

### Principles

- **DRY**: Don't Repeat Yourself, always try to encapsulate operations
- **Separation of Concerns**: Business logic is separated from UI concerns using a Service Layer pattern
- Commands module handles CLI operations
- Services module handles business logic that can be shared between CLI and TUI
- TUI acts as a controller that delegates operations to the service layer

### Current Structure

- `src/main.rs`: Entry point with CLI argument parsing (clap)
- `src/app.rs`: TUI application - handles UI state and rendering, delegates business logic to services
- `src/cli.rs`: CLI command definitions and routing
- `src/models/`: Core data structures
  - `entry.rs`: Work entry structure with timestamp, description, tags
  - `section.rs`: Section/category management
  - `doing_file.rs`: TaskPaper file format handling
- `src/storage/`: File persistence
  - `taskpaper.rs`: TaskPaper format parser/writer
  - `config.rs`: Configuration management
- `src/services/`: Business logic layer (Service Layer Pattern)
  - `entry_service.rs`: Entry operations (toggle done, delete, fetch) by UUID
- `src/commands/`: CLI command implementations
  - Various command handlers (`now.rs`, `done.rs`, `recent.rs`, etc.)

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
- **tui-textarea** (0.7.0): Rich text editing in TUI

## Testing Requirements

**IMPORTANT: All new commands MUST include comprehensive unit tests**

When implementing a new command:

1. Create a test module in `src/commands/tests/{command}_tests.rs`
2. Add tests covering:
   - Basic functionality (happy path)
   - Edge cases (empty files, missing entries, etc.)
   - Error conditions
   - All command-line flags and options
   - Integration with sections, tags, and search filters
3. Ensure tests use the test infrastructure in `src/test_utils.rs` for proper isolation
4. Run `cargo test` to verify all tests pass before marking a phase complete

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use crate::test_utils::test_utils::*;
    
    #[test]
    fn test_command_basic() {
        let ctx = TestContext::new().unwrap();
        // Test implementation
    }
    
    #[test]
    fn test_command_with_flags() {
        let ctx = TestContext::new().unwrap();
        // Test implementation
    }
}
```

## Code Style Guidelines

1. Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)
2. Use descriptive variable and function names
3. Keep functions focused and single-purpose
4. Add error handling with descriptive error messages
5. Use the `color_eyre::Result` type for error propagation
6. Avoid adding comments unless specifically requested

## Command Implementation Checklist

When adding a new command:

- [ ] Add CLI definition in `src/cli.rs`
- [ ] Create command handler in `src/commands/{command}.rs`
- [ ] Add to `src/commands/mod.rs` exports
- [ ] Add routing in `src/main.rs`
- [ ] Write comprehensive unit tests
- [ ] Test manually with various inputs
- [ ] Update CLAUDE.md documentation
- [ ] Ensure `cargo build` and `cargo test` pass without warnings

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
- 2025-07-28 17:27 | Archived completed task @done(2025-07-28 17:27) <2584ad82acb54469a6096ce1ddf966a0>
```

## Versioning Strategy

This project follows [Pragmatic Versioning](https://pragmaticversioning.com/) using the format **BIGRELEASE.ANNOUNCE.INCREMENT**:

- **BIGRELEASE**: Major milestones (0.x for pre-1.0 development, 1.x for stable)
- **ANNOUNCE**: Substantial changes or new features  
- **INCREMENT**: Every project contribution/fix

### Current Status
- **0.1.0**: Initial development release
- **Path to 1.0.0**: When core functionality is stable and feature-complete

### Version Increment Rules
- Increment for every merged contribution
- ANNOUNCE bumps for significant features or breaking changes
- BIGRELEASE bumps for major milestones

## Contributing Guidelines

### Pull Request Process

1. **Create a feature branch**: `git checkout -b feature/your-feature-name`
2. **Make your changes**: Follow the code style guidelines
3. **Add tests**: Ensure comprehensive test coverage
4. **Run the test suite**: `cargo test` - all tests must pass
5. **Check code quality**: `cargo clippy` and `cargo fmt`
6. **Update documentation**: Update CLAUDE.md if needed
7. **Create pull request**: Target the `main` branch
8. **Address feedback**: Respond to code review comments

### Branch Protection

The `main` branch is protected and requires:
- Pull request reviews (1 approval minimum)
- All status checks must pass
- Conversation resolution before merging
- No direct pushes allowed

### Development Workflow

1. **Issues**: Create an issue describing the bug or feature
2. **Discussion**: Discuss approach in the issue before coding
3. **Implementation**: Create feature branch and implement
4. **Testing**: Ensure comprehensive test coverage
5. **Review**: Submit PR for code review
6. **Merge**: Squash and merge after approval

## Release Process

Releases are automated through GitHub Actions:

1. **Version bump**: Update version in `Cargo.toml`
2. **Create tag**: `git tag v0.1.1` (matching Cargo.toml version)
3. **Push tag**: `git push origin v0.1.1`
4. **Automated release**: GitHub Actions will build and create release
5. **Release notes**: Generated automatically from commit messages

## Important Notes

- The app uses Rust edition 2024
- Terminal event handling is done through crossterm events
- The main loop runs while `App.running` is true
- Exit with Esc, q, or Ctrl-C
- Data is stored in TaskPaper-formatted text files
- Each entry has a unique UUID identifier
- Support for tags with values (@tag or @tag(value))
- Support for notes on entries (indented with 2 spaces)
- Multiple sections/categories (Currently, Archive, custom sections)

## Getting Help

- **Issues**: Report bugs or request features via GitHub Issues
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: Check CLAUDE.md for technical details
- **Original Tool**: Reference [Brett Terpstra's doing](https://github.com/ttscoff/doing) for compatibility

## Acknowledgments

This project is inspired by Brett Terpstra's excellent [doing](https://github.com/ttscoff/doing) time tracker. We are grateful to Brett for creating such a useful tool and for making it open source.