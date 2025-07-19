use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::storage::Db;
use crate::types::Mode;
use crate::ui::app::App;

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_event<D: Db>(&self, event: Event, app: &mut App<D>) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match app.state.mode {
                    Mode::Normal => self.handle_normal_key(key.code, app),
                    Mode::Insert => self.handle_insert_key(key.code, app),
                }
            }
        }
    }

    fn handle_normal_key<D: Db>(&self, key: KeyCode, app: &mut App<D>) {
        match key {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('i') => {
                app.state.mode = Mode::Insert;
                app.clear_error_message();
            }
            KeyCode::Char('h') => {
                app.state.show_help = true;
            }
            KeyCode::Char('j') => app.select_next_task(),
            KeyCode::Char('k') => app.select_previous_task(),
            KeyCode::Char('g') => app.select_first_task(),
            KeyCode::Char('G') => app.select_last_task(),
            KeyCode::Char('x') => {
                if let Some(selected) = app.selected_task_index() {
                    if let Err(e) = app.toggle_task_completion(selected) {
                        app.set_error_message(format!("Error updating task: {e}"));
                    }
                }
            }
            KeyCode::Char('D') => {
                if let Some(selected) = app.selected_task_index() {
                    if let Err(e) = app.delete_task(selected) {
                        app.set_error_message(format!("Error deleting task: {e}"));
                    } else {
                        app.adjust_selection_after_delete();
                    }
                }
            }
            KeyCode::Esc => {
                app.state.show_help = false;
            }
            _ => {}
        }
    }

    fn handle_insert_key<D: Db>(&self, key: KeyCode, app: &mut App<D>) {
        match key {
            KeyCode::Esc => {
                app.clear_input_buffer();
                app.state.mode = Mode::Normal;
            }
            KeyCode::Backspace => {
                app.backspace_input_buffer();
            }
            KeyCode::Enter => {
                let input_text = app.state.input_buffer.trim().to_string();
                if !input_text.is_empty() {
                    if let Err(e) = app.add_task(&input_text) {
                        app.set_error_message(format!("Error adding task: {e}"));
                    } else {
                        app.move_selection_to_last_task();
                    }
                }
                app.clear_input_buffer();
                app.state.mode = Mode::Normal;
            }
            KeyCode::Char(c) => {
                app.add_to_input_buffer(c);
            }
            _ => {}
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
