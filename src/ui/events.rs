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

            KeyCode::Char('o') => {
                app.create_task_below_cursor();
                app.state.mode = Mode::Insert;
                app.clear_error_message();
            }
            KeyCode::Char('O') => {
                app.create_task_above_cursor();
                app.state.mode = Mode::Insert;
                app.clear_error_message();
            }
            KeyCode::Char('h') => {
                app.state.show_help = !app.state.show_help;
            }
            KeyCode::Char('j') => app.cursor_next_task(),
            KeyCode::Char('k') => app.cursor_previous_task(),
            KeyCode::Char('g') => app.cursor_first_task(),
            KeyCode::Char('G') => app.cursor_last_task(),
            KeyCode::Char('!') => {
                if let Err(e) = app.toggle_task_completion() {
                    app.set_error_message(format!("Error updating task: {e}"));
                } else {
                    app.cursor_first_task();
                }
            }
            KeyCode::Char('x') => app.toggle_task_selection(),
            KeyCode::Char('D') => {
                if let Err(e) = app.delete_tasks() {
                    app.set_error_message(format!("Error deleting tasks: {e}"));
                }
            }
            _ => {}
        }
    }

    fn handle_insert_key<D: Db>(&self, key: KeyCode, app: &mut App<D>) {
        match key {
            KeyCode::Esc => {
                app.clear_input_buffer();
                app.state.mode = Mode::Normal;
                app.state.editing_task = None;
            }
            KeyCode::Backspace => {
                app.backspace_input_buffer();
            }
            KeyCode::Enter => {
                if app.state.editing_task.is_some() {
                    // Save current field
                    let input_text = app.state.input_buffer.trim().to_string();
                    app.update_editing_task_field(app.state.editing_field, input_text);

                    if let Err(e) = app.save_editing_task() {
                        app.set_error_message(format!("Error saving task: {e}"));
                    }
                    app.clear_input_buffer();
                    app.state.mode = Mode::Normal;
                } else {
                    // Legacy behavior for backward compatibility
                    let input_text = app.state.input_buffer.trim().to_string();
                    if !input_text.is_empty() {
                        if let Err(e) = app.add_task(&input_text) {
                            app.set_error_message(format!("Error adding task: {e}"));
                        } else {
                            app.cursor_last_task();
                        }
                    }
                    app.clear_input_buffer();
                    app.state.mode = Mode::Normal;
                }
            }
            KeyCode::Tab => {
                if app.state.editing_task.is_some() {
                    // Save current field before switching
                    let input_text = app.state.input_buffer.trim().to_string();
                    app.update_editing_task_field(app.state.editing_field, input_text);

                    // Move to next field (0: title, 1: description)
                    app.state.editing_field = (app.state.editing_field + 1) % 2;

                    // Load the new field's content into input buffer
                    let field_content = app.get_editing_task_field(app.state.editing_field);
                    app.state.input_buffer = field_content;
                }
            }
            KeyCode::BackTab => {
                if app.state.editing_task.is_some() {
                    // Save current field before switching
                    let input_text = app.state.input_buffer.trim().to_string();
                    app.update_editing_task_field(app.state.editing_field, input_text);

                    // Move to previous field
                    app.state.editing_field = if app.state.editing_field == 0 { 1 } else { 0 };

                    // Load the new field's content into input buffer
                    let field_content = app.get_editing_task_field(app.state.editing_field);
                    app.state.input_buffer = field_content;
                }
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
