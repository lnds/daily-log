use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use crate::storage::{Config, parse_taskpaper, save_taskpaper};
use crate::models::Entry;
use chrono::{Local, Duration, TimeZone};

/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    /// List of entries to display
    entries: Vec<Entry>,
    /// Selected entry index
    selected: usize,
    /// Error message to display
    error: Option<String>,
    /// Show detailed view of selected entry
    show_detail: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let mut app = Self {
            running: false,
            entries: Vec::new(),
            selected: 0,
            error: None,
            show_detail: false,
        };
        app.load_entries();
        app
    }

    /// Load entries from the doing file
    fn load_entries(&mut self) {
        let config = Config::load();
        let doing_file_path = config.doing_file_path();
        match parse_taskpaper(&doing_file_path) {
            Ok(doing_file) => {
                self.entries = doing_file.get_recent_entries(50)
                    .into_iter()
                    .cloned()
                    .collect();
                self.error = None;
            }
            Err(e) => {
                self.error = Some(format!("Failed to load entries: {e}"));
            }
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    fn render(&mut self, frame: &mut Frame) {
        if self.show_detail {
            self.render_detail(frame);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        // Title
        let title = Paragraph::new("Daily Log - Doing TUI")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        // Entries list
        let items: Vec<ListItem> = self.entries.iter().enumerate().map(|(i, entry)| {
            let mut lines = vec![];
            
            // Main entry line
            let mut spans = vec![
                Span::styled(
                    entry.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(" | "),
                Span::raw(&entry.description),
            ];

            // Add tags
            for (tag, value) in &entry.tags {
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    if let Some(v) = value {
                        format!("@{tag}({v})")
                    } else {
                        format!("@{tag}")
                    },
                    Style::default().fg(Color::Green),
                ));
            }

            // Add section
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                format!("[{}]", entry.section),
                Style::default().fg(Color::Magenta),
            ));

            // Add elapsed time if done
            if let Some(Some(done_time_str)) = entry.tags.get("done") {
                // Parse done timestamp
                if let Ok(done_time) = chrono::NaiveDateTime::parse_from_str(done_time_str, "%Y-%m-%d %H:%M") {
                    let done_local = Local.from_local_datetime(&done_time).single().unwrap_or_else(Local::now);
                    let duration = done_local.timestamp() - entry.timestamp.timestamp();
                    if duration > 0 {
                        let elapsed = Duration::seconds(duration);
                        let hours = elapsed.num_hours();
                        let minutes = (elapsed.num_minutes() % 60) as u32;
                        let seconds = (elapsed.num_seconds() % 60) as u32;
                        
                        spans.push(Span::raw(" "));
                        spans.push(Span::styled(
                            format!("{hours:02}:{minutes:02}:{seconds:02}"),
                            Style::default().fg(Color::Cyan),
                        ));
                    }
                }
            }

            lines.push(Line::from(spans));

            // Add note lines if present
            if let Some(note) = &entry.note {
                for note_line in note.lines() {
                    lines.push(Line::from(vec![
                        Span::raw("                     ┃ "),
                        Span::styled(note_line, Style::default().fg(Color::Gray)),
                    ]));
                }
            }

            let style = if i == self.selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            ListItem::new(lines).style(style)
        }).collect();

        let entries_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Entries"));
        frame.render_widget(entries_list, chunks[1]);

        // Help/status bar
        let help_text = if let Some(error) = &self.error {
            format!("Error: {error} | Press 'q' to quit, 'r' to reload")
        } else {
            "q: quit | ↑/↓: navigate | Enter: details | d: delete | r: reload".to_string()
        };
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(if self.error.is_some() { Color::Red } else { Color::Gray }))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(help, chunks[2]);
    }

    /// Render detailed view of selected entry
    fn render_detail(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        // Title
        let title = Paragraph::new("Entry Details")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        // Entry details
        if let Some(entry) = self.entries.get(self.selected) {
            let mut text = vec![
                Line::from(vec![
                    Span::styled("Time: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        entry.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                        Style::default().fg(Color::Yellow),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Description: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&entry.description),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Section: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        &entry.section,
                        Style::default().fg(Color::Magenta),
                    ),
                ]),
            ];

            // Add elapsed time if done
            if let Some(Some(done_time_str)) = entry.tags.get("done") {
                // Parse done timestamp
                if let Ok(done_time) = chrono::NaiveDateTime::parse_from_str(done_time_str, "%Y-%m-%d %H:%M") {
                    let done_local = Local.from_local_datetime(&done_time).single().unwrap_or_else(Local::now);
                    let duration = done_local.timestamp() - entry.timestamp.timestamp();
                    if duration > 0 {
                        let elapsed = Duration::seconds(duration);
                        let hours = elapsed.num_hours();
                        let minutes = (elapsed.num_minutes() % 60) as u32;
                        let seconds = (elapsed.num_seconds() % 60) as u32;
                        
                        text.push(Line::from(""));
                        text.push(Line::from(vec![
                            Span::styled("Elapsed Time: ", Style::default().add_modifier(Modifier::BOLD)),
                            Span::styled(
                                format!("{hours:02}:{minutes:02}:{seconds:02}"),
                                Style::default().fg(Color::Cyan),
                            ),
                        ]));
                    }
                }
            }

            // Add tags
            if !entry.tags.is_empty() {
                text.push(Line::from(""));
                text.push(Line::from(Span::styled("Tags:", Style::default().add_modifier(Modifier::BOLD))));
                for (tag, value) in &entry.tags {
                    text.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            if let Some(v) = value {
                                format!("@{tag}({v})")
                            } else {
                                format!("@{tag}")
                            },
                            Style::default().fg(Color::Green),
                        ),
                    ]));
                }
            }

            // Add note
            if let Some(note) = &entry.note {
                text.push(Line::from(""));
                text.push(Line::from(Span::styled("Note:", Style::default().add_modifier(Modifier::BOLD))));
                for line in note.lines() {
                    text.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(line, Style::default().fg(Color::Gray)),
                    ]));
                }
            }

            let details = Paragraph::new(text)
                .wrap(Wrap { trim: false })
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(details, chunks[1]);
        }

        // Help bar
        let help = Paragraph::new("Press Esc or Enter to return to list view")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(help, chunks[2]);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        // Handle detail view keys separately
        if self.show_detail {
            match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Enter) => {
                    self.show_detail = false;
                }
                (_, KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
                _ => {}
            }
            return;
        }

        // Handle list view keys
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Up) => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            (_, KeyCode::Down) => {
                if self.selected < self.entries.len().saturating_sub(1) {
                    self.selected += 1;
                }
            }
            (_, KeyCode::Char('r')) => {
                self.load_entries();
                if self.selected >= self.entries.len() && !self.entries.is_empty() {
                    self.selected = self.entries.len() - 1;
                }
            }
            (_, KeyCode::Enter) => {
                if !self.entries.is_empty() {
                    self.show_detail = true;
                }
            }
            (_, KeyCode::Char('d')) => {
                // Delete selected entry
                if self.selected < self.entries.len() {
                    self.delete_entry();
                }
            }
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    /// Delete the selected entry
    fn delete_entry(&mut self) {
        if let Some(entry) = self.entries.get(self.selected) {
            // Use the delete command logic
            let config = crate::storage::Config::load();
            let doing_file_path = config.doing_file_path();
            
            match crate::storage::parse_taskpaper(&doing_file_path) {
                Ok(mut doing_file) => {
                    // Find and remove the entry
                    let mut deleted = false;
                    for (_section_name, entries) in doing_file.sections.iter_mut() {
                        let initial_len = entries.len();
                        entries.retain(|e| e.uuid != entry.uuid);
                        if entries.len() < initial_len {
                            deleted = true;
                            break;
                        }
                    }
                    
                    if deleted {
                        // Save the file
                        if let Err(e) = save_taskpaper(&doing_file) {
                            self.error = Some(format!("Failed to save after deletion: {e}"));
                        } else {
                            // Remove from UI and adjust selection
                            self.entries.remove(self.selected);
                            if self.selected >= self.entries.len() && !self.entries.is_empty() {
                                self.selected = self.entries.len() - 1;
                            }
                            self.error = None;
                        }
                    } else {
                        self.error = Some("Entry not found in file".to_string());
                    }
                }
                Err(e) => {
                    self.error = Some(format!("Failed to load file for deletion: {e}"));
                }
            }
        }
    }
}
