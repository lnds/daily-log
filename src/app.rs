use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use tui_textarea::{Input, TextArea};
use crate::models::Entry;
use crate::services::EntryService;
use chrono::{Local, Duration, TimeZone};

/// Different modes the app can be in
#[derive(Debug, PartialEq)]
pub enum AppMode {
    Normal,
    EditEntry,
    EditNote,
}

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
    /// Section filter (if any)
    section_filter: Option<String>,
    /// Current mode of the app
    mode: AppMode,
    /// Text area for editing entries
    edit_textarea: TextArea<'static>,
    /// Text area for editing notes  
    note_textarea: TextArea<'static>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::new_with_section(None)
    }

    /// Construct a new instance of [`App`] with section filter.
    pub fn new_with_section(section: Option<String>) -> Self {
        let mut app = Self {
            running: false,
            entries: Vec::new(),
            selected: 0,
            error: None,
            show_detail: false,
            section_filter: section,
            mode: AppMode::Normal,
            edit_textarea: TextArea::default(),
            note_textarea: TextArea::default(),
        };
        app.load_entries();
        app
    }

    /// Load entries from the doing file
    fn load_entries(&mut self) {
        match EntryService::get_tui_entries(self.section_filter.as_deref(), 50) {
            Ok(entries) => {
                self.entries = entries;
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
        match self.mode {
            AppMode::EditEntry => {
                self.render_edit_mode(frame);
                return;
            }
            AppMode::EditNote => {
                self.render_note_mode(frame);
                return;
            }
            AppMode::Normal => {
                if self.show_detail {
                    self.render_detail(frame);
                    return;
                }
            }
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        // Title
        let title_text = if let Some(ref section) = self.section_filter {
            format!("Daily Log - Doing TUI [Section: {section}]")
        } else {
            "Daily Log - Doing TUI".to_string()
        };
        let title = Paragraph::new(title_text)
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
            "q: quit | ↑/↓: navigate | Enter: details | e: edit | n: note | d: delete | Space: toggle done | r: reload".to_string()
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
        let help = Paragraph::new("Press e to edit, n to edit note, Esc or Enter to return to list view")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(help, chunks[2]);
    }

    /// Render edit mode for editing an entry
    fn render_edit_mode(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        // Title
        let title = Paragraph::new("Edit Entry Description")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        // Text area for editing
        self.edit_textarea.set_style(Style::default().fg(Color::White));
        self.edit_textarea.set_cursor_style(Style::default().bg(Color::White).fg(Color::Black));
        self.edit_textarea.set_cursor_line_style(Style::default());
        self.edit_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Description")
        );
        frame.render_widget(&self.edit_textarea, chunks[1]);

        // Help bar
        let help = Paragraph::new("Press Ctrl+S to save, Esc to cancel")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(help, chunks[2]);
    }

    /// Render note edit mode for editing an entry's note
    fn render_note_mode(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        // Title
        let title = Paragraph::new("Edit Entry Note")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        // Text area for editing note
        self.note_textarea.set_style(Style::default().fg(Color::White));
        self.note_textarea.set_cursor_style(Style::default().bg(Color::White).fg(Color::Black));
        self.note_textarea.set_cursor_line_style(Style::default());
        self.note_textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Note (leave empty to remove note)")
        );
        frame.render_widget(&self.note_textarea, chunks[1]);

        // Help bar
        let help = Paragraph::new("Press Ctrl+S to save, Esc to cancel")
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
        // Handle edit mode keys
        if self.mode == AppMode::EditEntry {
            match (key.modifiers, key.code) {
                (_, KeyCode::Esc) => {
                    // Cancel editing
                    self.mode = AppMode::Normal;
                }
                (KeyModifiers::CONTROL, KeyCode::Char('s') | KeyCode::Char('S')) => {
                    // Save changes
                    self.save_edit();
                }
                _ => {
                    // Pass other keys to the text area
                    let input = Input::from(key);
                    self.edit_textarea.input(input);
                }
            }
            return;
        }

        // Handle note edit mode keys
        if self.mode == AppMode::EditNote {
            match (key.modifiers, key.code) {
                (_, KeyCode::Esc) => {
                    // Cancel editing
                    self.mode = AppMode::Normal;
                }
                (KeyModifiers::CONTROL, KeyCode::Char('s') | KeyCode::Char('S')) => {
                    // Save changes
                    self.save_note();
                }
                _ => {
                    // Pass other keys to the text area
                    let input = Input::from(key);
                    self.note_textarea.input(input);
                }
            }
            return;
        }

        // Handle detail view keys separately
        if self.show_detail {
            match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Enter) => {
                    self.show_detail = false;
                }
                (_, KeyCode::Char('e')) => {
                    // Edit entry from detail view
                    self.show_detail = false;
                    self.enter_edit_mode();
                }
                (_, KeyCode::Char('n')) => {
                    // Edit note from detail view
                    self.show_detail = false;
                    self.enter_note_mode();
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
            (_, KeyCode::Char(' ')) => {
                // Toggle @done status
                if self.selected < self.entries.len() {
                    self.toggle_done();
                }
            }
            (_, KeyCode::Char('e')) => {
                // Edit selected entry
                if self.selected < self.entries.len() {
                    self.enter_edit_mode();
                }
            }
            (_, KeyCode::Char('n')) => {
                // Edit note of selected entry
                if self.selected < self.entries.len() {
                    self.enter_note_mode();
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
            // Use the service layer to delete the entry
            match EntryService::delete_by_uuid(&entry.uuid) {
                Ok(()) => {
                    // Remove from UI and adjust selection
                    self.entries.remove(self.selected);
                    if self.selected >= self.entries.len() && !self.entries.is_empty() {
                        self.selected = self.entries.len() - 1;
                    }
                    self.error = None;
                }
                Err(e) => {
                    self.error = Some(format!("Failed to delete entry: {e}"));
                }
            }
        }
    }
    
    /// Toggle the @done status of the selected entry
    fn toggle_done(&mut self) {
        if let Some(entry) = self.entries.get(self.selected) {
            // Use the service layer to toggle the done status
            match EntryService::toggle_done_by_uuid(&entry.uuid) {
                Ok(updated_entry) => {
                    // Update the entry in the UI
                    self.entries[self.selected] = updated_entry;
                    self.error = None;
                }
                Err(e) => {
                    self.error = Some(format!("Failed to toggle done status: {e}"));
                }
            }
        }
    }

    /// Enter edit mode for the selected entry
    fn enter_edit_mode(&mut self) {
        if let Some(entry) = self.entries.get(self.selected) {
            // Initialize the text area with the current description
            self.edit_textarea = TextArea::new(vec![entry.description.clone()]);
            self.edit_textarea.move_cursor(tui_textarea::CursorMove::End);
            self.mode = AppMode::EditEntry;
        }
    }

    /// Save the edited entry
    fn save_edit(&mut self) {
        if let Some(entry) = self.entries.get(self.selected) {
            let new_description = self.edit_textarea.lines().join("\n").trim().to_string();
            
            // Don't save if description is empty
            if new_description.is_empty() {
                self.error = Some("Description cannot be empty".to_string());
                return;
            }
            
            // Use the service layer to update the entry
            match EntryService::update_entry_description(&entry.uuid, new_description) {
                Ok(_updated_entry) => {
                    // Reload entries from file to ensure consistency
                    self.load_entries();
                    // Make sure selection is still valid
                    if self.selected >= self.entries.len() && !self.entries.is_empty() {
                        self.selected = self.entries.len() - 1;
                    }
                    self.mode = AppMode::Normal;
                    self.error = None;
                }
                Err(e) => {
                    self.error = Some(format!("Failed to update entry: {e}"));
                }
            }
        }
    }

    /// Enter note edit mode for the selected entry
    fn enter_note_mode(&mut self) {
        if let Some(entry) = self.entries.get(self.selected) {
            // Initialize the text area with the current note or empty if no note
            let note_lines = if let Some(note) = &entry.note {
                note.lines().map(|s| s.to_string()).collect()
            } else {
                vec!["".to_string()]
            };
            self.note_textarea = TextArea::new(note_lines);
            self.note_textarea.move_cursor(tui_textarea::CursorMove::End);
            self.mode = AppMode::EditNote;
        }
    }

    /// Save the edited note
    fn save_note(&mut self) {
        if let Some(entry) = self.entries.get(self.selected) {
            let new_note_text = self.note_textarea.lines().join("\n").trim().to_string();
            
            // Convert empty string to None (remove note)
            let new_note = if new_note_text.is_empty() {
                None
            } else {
                Some(new_note_text)
            };
            
            // Use the service layer to update the entry note
            match EntryService::update_entry_note(&entry.uuid, new_note) {
                Ok(_updated_entry) => {
                    // Reload entries from file to ensure consistency
                    self.load_entries();
                    // Make sure selection is still valid
                    if self.selected >= self.entries.len() && !self.entries.is_empty() {
                        self.selected = self.entries.len() - 1;
                    }
                    self.mode = AppMode::Normal;
                    self.error = None;
                }
                Err(e) => {
                    self.error = Some(format!("Failed to update note: {e}"));
                }
            }
        }
    }
}
