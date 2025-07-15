use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, Paragraph},
};
use thiserror::Error;

use crate::types::{AppState, Mode};

pub struct Ui {
    app: AppState,
    state: ListState,
}

impl Ui {
    pub fn new(app: AppState) -> Self {
        Self {
            app,
            state: ListState::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), UiError> {
        let mut terminal = ratatui::init();
        while !self.app.should_quit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_event(event::read()?);
        }
        ratatui::restore();

        Ok(())
    }

    fn draw(&mut self, f: &mut Frame) {
        let (title_area, main_area, status_area) = self.calculate_layout(f.area());

        self.render_title(f, title_area);
        self.render_main(f, main_area);
        self.render_status(f, status_area);
    }

    fn render_main(&mut self, f: &mut Frame, area: Rect) {
        let list_items = self
            .app
            .tasks
            .iter()
            .map(|task| {
                ListItem::new(Line::from(format!(
                    "[{}] {}",
                    if task.completed { "x" } else { " " },
                    task.title
                )))
            })
            .collect::<Vec<_>>();

        f.render_stateful_widget(
            List::new(list_items)
                .block(
                    Block::bordered()
                        .title(Line::from(format!(" Tasks ({}) ", self.app.tasks.len()))),
                )
                .highlight_symbol(" > ")
                .highlight_spacing(HighlightSpacing::Always),
            area,
            &mut self.state,
        );
    }

    fn calculate_layout(&self, area: Rect) -> (Rect, Rect, Rect) {
        let main_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [title_area, main_area, status_area] = main_layout.areas(area);
        (title_area, main_area, status_area)
    }

    fn render_title(&self, f: &mut Frame, area: Rect) {
        f.render_widget(
            Paragraph::new("Wimm Task Manager - Press 'q' to quit").alignment(Alignment::Center),
            area,
        );
    }

    fn render_status(&self, f: &mut Frame, area: Rect) {
        let mode = match self.app.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Command => "COMMAND",
        };
        let status = format!("Mode: {}", mode);
        f.render_widget(Paragraph::new(status).alignment(Alignment::Left), area);
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match self.app.mode {
                    Mode::Normal => self.handle_normal_key(key.code),
                    Mode::Insert => self.handle_insert_key(key.code),
                    Mode::Command => self.handle_command_key(key.code),
                }
            }
        }
    }

    fn handle_normal_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.app.should_quit = true,
            KeyCode::Char('i') => self.app.mode = Mode::Insert,
            KeyCode::Char(':') => self.app.mode = Mode::Command,
            KeyCode::Char('j') => self.state.select_next(),
            KeyCode::Char('k') => self.state.select_previous(),
            KeyCode::Char('g') => self.state.select_first(),
            KeyCode::Char('G') => self.state.select_last(),
            KeyCode::Char('x') => self.toggle_task_completion(),
            KeyCode::Char('D') => self.delete_task(),
            _ => {}
        }
    }

    fn handle_insert_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.app.mode = Mode::Normal,
            _ => {}
        }
    }

    fn handle_command_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => self.app.mode = Mode::Normal,
            _ => {}
        }
    }

    fn toggle_task_completion(&mut self) {
        if let Some(selected) = self.state.selected() {
            if let Some(task) = self.app.tasks.get_mut(selected) {
                task.completed = !task.completed;
            }
        }
    }

    fn delete_task(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected < self.app.tasks.len() {
                self.app.tasks.remove(selected);
                self.state.select_previous();
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum UiError {
    #[error("Terminal IO error: {0}")]
    IoError(#[from] std::io::Error),
}
