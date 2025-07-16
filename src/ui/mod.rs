use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph},
};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    storage::{self, Db},
    types::{AppState, Mode, Task},
};

pub struct Ui<D: Db> {
    app: AppState<D>,
    state: ListState,
}

impl<D: Db> Ui<D> {
    pub fn new(app: AppState<D>) -> Self {
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
        let (title_area, main_area, status_area, input_area) = self.calculate_layout(f.area());

        self.render_title(f, title_area);
        self.render_main(f, main_area);
        self.render_status(f, status_area);
        self.render_input(f, input_area);
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

        if !list_items.is_empty() && self.state.selected().is_none() {
            self.state.select_first();
        }

        f.render_stateful_widget(
            List::new(list_items)
                .block(
                    Block::bordered()
                        .padding(Padding::uniform(1))
                        .title(Line::from(format!(" Tasks ({}) ", self.app.tasks.len()))),
                )
                .highlight_symbol("> ")
                .highlight_spacing(HighlightSpacing::Always),
            area,
            &mut self.state,
        );
    }

    fn calculate_layout(&self, area: Rect) -> (Rect, Rect, Rect, Rect) {
        let main_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ]);
        let [title_area, main_area, status_area, input_area] = main_layout.areas(area);
        (title_area, main_area, status_area, input_area)
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
        };
        let status = format!("Mode: {}", mode);
        f.render_widget(Paragraph::new(status).alignment(Alignment::Left), area);
    }

    fn render_input(&self, f: &mut Frame, area: Rect) {
        if self.app.mode == Mode::Insert {
            f.render_widget(Line::from(format!("> {}", &self.app.input_buffer)), area);
        }
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match self.app.mode {
                    Mode::Normal => self.handle_normal_key(key.code),
                    Mode::Insert => self.handle_insert_key(key.code),
                }
            }
        }
    }

    fn handle_normal_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.app.should_quit = true,
            KeyCode::Char('i') => self.app.mode = Mode::Insert,
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
            KeyCode::Esc => {
                self.app.input_buffer.clear();
                self.app.mode = Mode::Normal;
            }
            KeyCode::Backspace => {
                self.app.input_buffer.pop();
            }
            KeyCode::Enter => {
                let new_task_title = self.app.input_buffer.trim().to_string();
                if !new_task_title.is_empty() {
                    self.add_task(&new_task_title);
                }
                self.app.input_buffer.clear();
                self.app.mode = Mode::Normal;
            }
            KeyCode::Char(c) => {
                self.app.input_buffer.push(c);
            }
            _ => {}
        }
    }

    fn add_task(&mut self, title: &str) {
        let new_task = create_task(title);
        self.app.tasks.push(new_task);
        self.state.select_last();
        if let Err(e) = self.sync_to_storage() {
            self.app.message = Some(format!("Error adding task: {}", e));
        }
    }

    fn toggle_task_completion(&mut self) {
        if let Some(selected) = self.state.selected() {
            if let Some(task) = self.app.tasks.get_mut(selected) {
                task.completed = !task.completed;
                if let Err(e) = self.sync_to_storage() {
                    self.app.message = Some(format!("Error updating task: {}", e));
                }
            }
        }
    }

    fn delete_task(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected < self.app.tasks.len() {
                self.app.tasks.remove(selected);
                self.state.select_previous();
                if let Err(e) = self.sync_to_storage() {
                    self.app.message = Some(format!("Error deleting task: {}", e));
                }
            }
        }
    }

    fn sync_to_storage(&mut self) -> Result<(), UiError> {
        self.app.store.clear()?;
        for task in &self.app.tasks {
            self.app.store.save_task(task)?;
        }
        Ok(())
    }
}

fn create_task(title: &str) -> Task {
    Task {
        id: Uuid::new_v4().to_string(),
        title: title.to_string(),
        description: String::new(),
        completed: false,
        created_at: std::time::SystemTime::now(),
    }
}

#[derive(Debug, Error)]
pub enum UiError {
    #[error("Terminal IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("DB error: {0}")]
    DbError(#[from] storage::DbError),
}
