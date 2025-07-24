//! Terminal user interface module for WIMM
//!
//! This module provides the complete terminal-based user interface for the WIMM task
//! management application. It handles:
//! - Task list rendering and formatting
//! - Input processing and event handling
//! - Help system and UI overlays
//! - Date/time formatting and display
//! - Task highlighting based on urgency and status
//!
//! The UI is built using the ratatui library and follows a component-based architecture
//! with separate modules for different UI concerns.

use chrono::{DateTime, Local};
use ratatui::Frame;
use ratatui::crossterm::event;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Cell, Padding, Row, Table};
use std::collections::HashSet;
use std::time::SystemTime;
use thiserror::Error;

use crate::storage::{self, Db};
use crate::types::{AppState, Task};

/// Format an optional timestamp for display in the UI
///
/// Converts an optional SystemTime to a human-readable string:
/// - Some(time) -> relative time format (e.g., "2h ago", "in 3d")
/// - None -> "-" to indicate no date set
fn format_date(time: Option<SystemTime>) -> String {
    match time {
        Some(t) => format_time_relative(t),
        None => "-".to_string(),
    }
}

/// Format a timestamp as relative time (e.g., "2h ago", "in 3d")
///
/// This function converts absolute timestamps to human-readable relative time
/// strings, making it easier for users to understand when tasks are due or
/// when they were created. Handles both past and future dates gracefully.
///
/// # Arguments
/// * `time` - The timestamp to format
///
/// # Returns
/// A string representation like "2d ago", "3h ago", "in 1d", or "now"
fn format_time_relative(time: SystemTime) -> String {
    let now = SystemTime::now();
    match now.duration_since(time) {
        Ok(duration) => {
            // Time is in the past
            let secs = duration.as_secs();
            let days = secs / 86400;
            let hours = (secs % 86400) / 3600;
            let minutes = (secs % 3600) / 60;

            if days > 0 {
                format!("{days}d ago")
            } else if hours > 0 {
                format!("{hours}h ago")
            } else if minutes > 0 {
                format!("{minutes}m ago")
            } else {
                "now".to_string()
            }
        }
        Err(_) => {
            // Time is in the future
            match time.duration_since(now) {
                Ok(duration) => {
                    let secs = duration.as_secs();
                    let days = secs / 86400;
                    let hours = (secs % 86400) / 3600;
                    let minutes = (secs % 3600) / 60;

                    if days > 0 {
                        format!("in {days}d")
                    } else if hours > 0 {
                        format!("in {hours}h")
                    } else if minutes > 0 {
                        format!("in {minutes}m")
                    } else {
                        "now".to_string()
                    }
                }
                Err(_) => "Invalid".to_string(),
            }
        }
    }
}

/// Format task creation timestamp for display
///
/// Uses a hybrid approach to balance usefulness and readability:
/// - Recent tasks (< 24h): Show relative time ("2h ago", "30m ago")
/// - Older tasks: Show absolute date ("2024-01-15")
///
/// This provides immediate context for recent activity while keeping
/// older entries compact and dateable.
///
/// # Arguments
/// * `time` - The creation timestamp to format
///
/// # Returns
/// A formatted string suitable for display in the task list
fn format_created_at(time: SystemTime) -> String {
    let now = SystemTime::now();
    let datetime = DateTime::<Local>::from(time);

    match now.duration_since(time) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let days = secs / 86400;
            let hours = (secs % 86400) / 3600;
            let minutes = (secs % 3600) / 60;

            // For tasks created within the last day, show relative time
            if days == 0 {
                if hours > 0 {
                    format!("{hours}h ago")
                } else if minutes > 0 {
                    format!("{minutes}m ago")
                } else {
                    "now".to_string()
                }
            } else {
                // For older tasks, show the actual date
                datetime.format("%Y-%m-%d").to_string()
            }
        }
        Err(_) => datetime.format("%Y-%m-%d").to_string(),
    }
}

/// Determine the visual style for a task based on its scheduling status
///
/// This function implements visual priority cues to help users quickly identify
/// task urgency and scheduling states:
///
/// - **Deferred tasks**: Dimmed (dark gray) until defer date passes
/// - **Overdue tasks**: Bold red text for immediate attention
/// - **Due today**: Bold red text for high urgency
/// - **Due within 24h**: Bold yellow text for moderate urgency
/// - **Normal tasks**: Default styling
///
/// The styling follows a traffic light pattern (red = urgent, yellow = soon)
/// with additional dimming for deferred items.
///
/// # Arguments
/// * `task` - The task to determine styling for
///
/// # Returns
/// A ratatui Style object with appropriate colors and modifiers
fn get_task_highlight_style(task: &Task) -> Style {
    let now = SystemTime::now();

    // Check if task is deferred (should be dimmed)
    if let Some(defer_until) = task.defer_until {
        if now < defer_until {
            return Style::default().fg(Color::DarkGray);
        }
    }

    // Check due date highlighting
    if let Some(due_date) = task.due {
        let time_until_due = due_date.duration_since(now);

        match time_until_due {
            Ok(duration) => {
                let hours_until_due = duration.as_secs() / 3600;

                if hours_until_due == 0 {
                    // Due today - strong highlight (red text)
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                } else if hours_until_due <= 24 {
                    // Due within 24 hours - subtle highlight (yellow text)
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    // Not due soon - normal style
                    Style::default()
                }
            }
            Err(_) => {
                // Overdue - strong red highlight
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            }
        }
    } else {
        // No due date - normal style
        Style::default()
    }
}

// Sub-modules providing specialized UI functionality
pub mod app; // Core application state management and business logic
pub mod events; // Keyboard input processing and event handling
pub mod help_panel; // Help overlay system

pub mod layout; // Terminal layout management and responsive design

use app::App;
use events::EventHandler;
use help_panel::HelpPanel;
use layout::LayoutManager;

/// Main UI coordinator combining all interface components
///
/// This struct orchestrates the various UI subsystems to provide a cohesive
/// terminal interface. It combines:
/// - App: Core application logic and state management
/// - HelpPanel: Context-sensitive help system
/// - LayoutManager: Responsive terminal layout
/// - EventHandler: Input processing and command routing
///
/// The UI is generic over the database type to support different storage backends.
pub struct Ui<D: Db> {
    /// Core application state and business logic
    app: App<D>,
    /// Help system for displaying contextual assistance
    help_panel: HelpPanel,
    /// Terminal layout management for responsive design
    layout_manager: LayoutManager,
    /// Input processing and event routing
    event_handler: EventHandler,
}

impl<D: Db> Ui<D> {
    pub fn new(app_state: AppState<D>) -> Self {
        Self {
            app: App::new(app_state),
            help_panel: HelpPanel::new(),
            layout_manager: LayoutManager::new(),
            event_handler: EventHandler::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), UiError> {
        let mut terminal = ratatui::init();

        while !self.app.state.should_quit {
            terminal.draw(|f| self.draw(f))?;
            let event = event::read()?;
            self.event_handler.handle_event(event, &mut self.app);
        }

        ratatui::restore();
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame) {
        let layout = self
            .layout_manager
            .calculate_main_layout(f.area(), &self.app.state);

        // Render title
        self.render_title(f, layout.title);

        // Render main task list
        self.render_task_list(f, layout.main);

        // Render status bar
        self.render_status(f, layout.status);

        // Show error messages in status if needed
        if let Some(ref message) = self.app.message {
            self.render_error_status(f, layout.status, message);
        }

        // Render help panel if visible
        if let Some(help_area) = layout.help {
            self.help_panel.render(f, help_area);
        }
    }

    fn render_title(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        use ratatui::{layout::Alignment, widgets::Paragraph};

        let title = Paragraph::new("Wimm Task Manager - Press 'q' to quit, 'h' for help")
            .alignment(Alignment::Center);
        f.render_widget(title, area);
    }

    fn render_status(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        use ratatui::{layout::Alignment, widgets::Paragraph};

        let mode_text = match self.app.state.mode {
            crate::types::Mode::Normal => "NORMAL".to_string(),
            crate::types::Mode::Insert => {
                if self.app.state.editing_task.is_some() {
                    let field_name = match self.app.state.editing_field {
                        0 => "Title",
                        1 => "Description",
                        2 => "Due Date",
                        3 => "Defer Until",
                        _ => "Unknown",
                    };
                    format!("INSERT - Editing: {field_name}")
                } else {
                    "INSERT".to_string()
                }
            }
        };

        let status = format!("Mode: {mode_text}");
        let status_paragraph = Paragraph::new(status).alignment(Alignment::Left);
        f.render_widget(status_paragraph, area);
    }

    fn render_task_list(&mut self, f: &mut Frame, area: ratatui::layout::Rect) {
        // Auto-select first item if nothing is selected and tasks exist
        if !self.app.state.tasks.is_empty() && self.app.cursor_task_index().is_none() {
            self.app.cursor_first_task();
        }

        let header = Row::new(vec![
            Cell::from("Status").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Title").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Description").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Created").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Due").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from("Defer Until").style(Style::default().add_modifier(Modifier::BOLD)),
        ]);

        // Get necessary data before borrowing self.app mutably
        let current_selection = self.app.cursor_task_index();
        let is_editing_task = self.app.state.editing_task.is_some();
        let editing_field = self.app.state.editing_field;
        let input_buffer = self.app.state.input_buffer.clone();
        let task_count = self.app.state.tasks.len();
        let editing_task = self.app.state.editing_task.clone();

        // Clone the tasks to avoid borrowing issues
        let tasks = self.app.state.tasks.clone();
        let selected_tasks: HashSet<usize> = self.app.get_task_selection().clone();

        let rows: Vec<Row> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let is_selected = current_selection == Some(i);
                let is_editing = is_editing_task && is_selected;

                let status_cell = Cell::from(if task.completed { "[x]" } else { "[ ]" });

                let title_cell = if is_editing && is_selected && editing_field == 0 {
                    // Currently editing title - show input buffer with highlight
                    let display_text = if input_buffer.is_empty() {
                        " "
                    } else {
                        &input_buffer
                    };
                    Cell::from(Line::from(vec![Span::styled(
                        display_text,
                        Style::default().bg(Color::Yellow).fg(Color::Black),
                    )]))
                } else if is_editing && is_selected {
                    // Show the current title from editing task
                    if let Some(ref editing_task) = editing_task {
                        Cell::from(editing_task.title.clone())
                    } else {
                        Cell::from(task.title.clone())
                    }
                } else {
                    Cell::from(task.title.clone())
                };

                let description_cell = if is_editing && is_selected && editing_field == 1 {
                    // Currently editing description - show input buffer with highlight
                    let display_text = if input_buffer.is_empty() {
                        " "
                    } else {
                        &input_buffer
                    };
                    Cell::from(Line::from(vec![Span::styled(
                        display_text,
                        Style::default().bg(Color::Yellow).fg(Color::Black),
                    )]))
                } else if is_editing && is_selected {
                    // Show the current description from editing task
                    if let Some(ref editing_task) = editing_task {
                        Cell::from(editing_task.description.clone())
                    } else {
                        Cell::from(task.description.clone())
                    }
                } else {
                    Cell::from(task.description.clone())
                };

                let created_cell = Cell::from(format_created_at(task.created_at));

                let due_cell = if is_editing && is_selected && editing_field == 2 {
                    let display_text = if input_buffer.is_empty() {
                        " "
                    } else {
                        &input_buffer
                    };
                    Cell::from(Line::from(vec![Span::styled(
                        display_text,
                        Style::default().bg(Color::Yellow).fg(Color::Black),
                    )]))
                } else if is_editing && is_selected {
                    if let Some(ref editing_task) = editing_task {
                        Cell::from(format_date(editing_task.due))
                    } else {
                        Cell::from(format_date(task.due))
                    }
                } else {
                    Cell::from(format_date(task.due))
                };

                let defer_cell = if is_editing && is_selected && editing_field == 3 {
                    let display_text = if input_buffer.is_empty() {
                        " "
                    } else {
                        &input_buffer
                    };
                    Cell::from(Line::from(vec![Span::styled(
                        display_text,
                        Style::default().bg(Color::Yellow).fg(Color::Black),
                    )]))
                } else if is_editing && is_selected {
                    if let Some(ref editing_task) = editing_task {
                        Cell::from(format_date(editing_task.defer_until))
                    } else {
                        Cell::from(format_date(task.defer_until))
                    }
                } else {
                    Cell::from(format_date(task.defer_until))
                };

                let base_style = get_task_highlight_style(task);

                Row::new(vec![
                    status_cell,
                    title_cell,
                    description_cell,
                    created_cell,
                    due_cell,
                    defer_cell,
                ])
                .style(if selected_tasks.contains(&i) {
                    base_style.bg(Color::DarkGray)
                } else {
                    base_style
                })
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(5),      // Status column
                Constraint::Percentage(25), // Title column
                Constraint::Percentage(30), // Description column
                Constraint::Length(10),     // Created column
                Constraint::Length(10),     // Due column
                Constraint::Length(12),     // Defer Until column
            ],
        )
        .header(header)
        .block(
            Block::bordered()
                .padding(Padding::uniform(1))
                .title(Line::from(format!(" Tasks ({task_count}) "))),
        )
        .highlight_symbol("> ");

        f.render_stateful_widget(table, area, self.app.task_list_state());
    }

    fn render_error_status(&self, f: &mut Frame, area: ratatui::layout::Rect, message: &str) {
        use ratatui::{layout::Alignment, widgets::Paragraph};

        let error_paragraph = Paragraph::new(message)
            .alignment(Alignment::Left)
            .style(Style::default().fg(Color::Red));
        f.render_widget(error_paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_format_created_at_recent() {
        let now = SystemTime::now();
        let two_hours_ago = now - Duration::from_secs(2 * 60 * 60);
        let result = format_created_at(two_hours_ago);
        assert_eq!(result, "2h ago");
    }

    #[test]
    fn test_format_created_at_old() {
        let now = SystemTime::now();
        let two_days_ago = now - Duration::from_secs(2 * 24 * 60 * 60);
        let result = format_created_at(two_days_ago);
        // Should show actual date for tasks older than 1 day
        assert!(result.contains("-"));
        assert!(!result.contains("ago"));
    }

    #[test]
    fn test_format_time_relative() {
        let now = SystemTime::now();
        let one_hour_ago = now - Duration::from_secs(60 * 60);
        let result = format_time_relative(one_hour_ago);
        assert_eq!(result, "1h ago");

        let future_time = now + Duration::from_secs(2 * 24 * 60 * 60);
        let result = format_time_relative(future_time);
        assert!(result.starts_with("in ") && result.contains("d"));
    }

    #[test]
    fn test_get_task_highlight_style_normal() {
        let task = Task {
            id: "test".to_string(),
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            completed: false,
            created_at: SystemTime::now(),
            due: None,
            defer_until: None,
        };

        let style = get_task_highlight_style(&task);
        assert_eq!(style, Style::default());
    }

    #[test]
    fn test_get_task_highlight_style_deferred() {
        let future_time = SystemTime::now() + Duration::from_secs(60 * 60); // 1 hour from now
        let task = Task {
            id: "test".to_string(),
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            completed: false,
            created_at: SystemTime::now(),
            due: None,
            defer_until: Some(future_time),
        };

        let style = get_task_highlight_style(&task);
        assert_eq!(style.fg, Some(Color::DarkGray));
    }

    #[test]
    fn test_get_task_highlight_style_due_soon() {
        let due_in_12_hours = SystemTime::now() + Duration::from_secs(12 * 60 * 60);
        let task = Task {
            id: "test".to_string(),
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            completed: false,
            created_at: SystemTime::now(),
            due: Some(due_in_12_hours),
            defer_until: None,
        };

        let style = get_task_highlight_style(&task);
        assert_eq!(style.fg, Some(Color::Yellow));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_get_task_highlight_style_overdue() {
        let past_time = SystemTime::now() - Duration::from_secs(60 * 60); // 1 hour ago
        let task = Task {
            id: "test".to_string(),
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            completed: false,
            created_at: SystemTime::now(),
            due: Some(past_time),
            defer_until: None,
        };

        let style = get_task_highlight_style(&task);
        assert_eq!(style.fg, Some(Color::Red));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }
}

#[derive(Debug, Error)]
pub enum UiError {
    #[error("Terminal IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("DB error: {0}")]
    DbError(#[from] storage::DbError),
}
