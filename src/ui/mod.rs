use ratatui::Frame;
use ratatui::crossterm::event;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Cell, Padding, Row, Table};
use std::collections::HashSet;
use thiserror::Error;

use crate::storage::{self, Db};
use crate::types::AppState;

mod app;
mod events;
mod help_panel;

mod layout;

use app::App;
use events::EventHandler;
use help_panel::HelpPanel;
use layout::LayoutManager;

pub struct Ui<D: Db> {
    app: App<D>,
    help_panel: HelpPanel,
    layout_manager: LayoutManager,
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
                        _ => "Unknown",
                    };
                    format!("INSERT - Editing: {}", field_name)
                } else {
                    "INSERT".to_string()
                }
            }
        };

        let status = format!("Mode: {}", mode_text);
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
                    Cell::from(Line::from(vec![Span::styled(
                        input_buffer.clone(),
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
                    Cell::from(Line::from(vec![Span::styled(
                        input_buffer.clone(),
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

                Row::new(vec![status_cell, title_cell, description_cell]).style(
                    Style::default().bg(if selected_tasks.contains(&i) {
                        Color::DarkGray
                    } else if is_selected {
                        Color::Blue
                    } else {
                        Color::Reset
                    }),
                )
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(5),      // Status column
                Constraint::Percentage(40), // Title column
                Constraint::Percentage(55), // Description column
            ],
        )
        .header(header)
        .block(
            Block::bordered()
                .padding(Padding::uniform(1))
                .title(Line::from(format!(" Tasks ({}) ", task_count))),
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

#[derive(Debug, Error)]
pub enum UiError {
    #[error("Terminal IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("DB error: {0}")]
    DbError(#[from] storage::DbError),
}
