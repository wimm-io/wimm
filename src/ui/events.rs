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
                // Load the current field content into input buffer
                let field_content = app.get_editing_task_field(app.state.editing_field);
                app.state.input_buffer = field_content;
            }
            KeyCode::Char('O') => {
                app.create_task_above_cursor();
                app.state.mode = Mode::Insert;
                app.clear_error_message();
                // Load the current field content into input buffer
                let field_content = app.get_editing_task_field(app.state.editing_field);
                app.state.input_buffer = field_content;
            }
            KeyCode::Char('i') => {
                app.start_editing_current_task();
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

                    // Move to next field (0: title, 1: description, 2: due, 3: defer_until)
                    app.state.editing_field = (app.state.editing_field + 1) % 4;

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
                    app.state.editing_field = if app.state.editing_field == 0 {
                        3
                    } else {
                        app.state.editing_field - 1
                    };

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;
    use crate::types::{AppState, Task};
    use ratatui::crossterm::event::{KeyEvent, KeyModifiers};
    use std::collections::HashMap;
    use std::time::SystemTime;

    fn create_test_app() -> App<MemoryStorage> {
        let store = MemoryStorage::new(HashMap::new());
        let state = AppState::new(store);
        App::new(state)
    }

    fn create_test_task(id: &str, title: &str) -> Task {
        Task {
            id: id.to_string(),
            title: title.to_string(),
            description: format!("Description for {title}"),
            completed: false,
            created_at: SystemTime::now(),
            due: None,
            defer_until: None,
        }
    }

    fn create_key_event(code: KeyCode) -> Event {
        Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: ratatui::crossterm::event::KeyEventState::NONE,
        })
    }

    #[test]
    fn test_event_handler_new() {
        let _handler = EventHandler::new();
        // Just verify it creates successfully
        // Test passes if creation succeeds without panic
    }

    #[test]
    fn test_event_handler_default() {
        let _handler = EventHandler;
        // Just verify it creates successfully
        // Test passes if creation succeeds without panic
    }

    #[test]
    fn test_handle_quit_key() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        assert!(!app.state.should_quit);

        let event = create_key_event(KeyCode::Char('q'));
        handler.handle_event(event, &mut app);

        assert!(app.state.should_quit);
    }

    #[test]
    fn test_handle_help_toggle() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        assert!(!app.state.show_help);

        let event = create_key_event(KeyCode::Char('h'));
        handler.handle_event(event, &mut app);

        assert!(app.state.show_help);

        // Toggle again
        let event = create_key_event(KeyCode::Char('h'));
        handler.handle_event(event, &mut app);
        assert!(!app.state.show_help);
    }

    #[test]
    fn test_handle_create_task_below() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        assert_eq!(app.state.mode, Mode::Normal);
        assert!(app.state.editing_task.is_none());

        let event = create_key_event(KeyCode::Char('o'));
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.mode, Mode::Insert);
        assert!(app.state.editing_task.is_some());
    }

    #[test]
    fn test_handle_create_task_above() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        assert_eq!(app.state.mode, Mode::Normal);
        assert!(app.state.editing_task.is_none());

        let event = create_key_event(KeyCode::Char('O'));
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.mode, Mode::Insert);
        assert!(app.state.editing_task.is_some());
    }

    #[test]
    fn test_handle_start_editing() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        // Add a task first
        let task = create_test_task("1", "Test Task");
        app.state.tasks.push(task);

        assert_eq!(app.state.mode, Mode::Normal);

        let event = create_key_event(KeyCode::Char('i'));
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.mode, Mode::Insert);
    }

    #[test]
    fn test_handle_insert_mode_escape() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        app.state.mode = Mode::Insert;
        app.state.input_buffer = "test input".to_string();
        app.state.editing_task = Some(create_test_task("test", "Test"));

        let event = create_key_event(KeyCode::Esc);
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.mode, Mode::Normal);
        assert!(app.state.input_buffer.is_empty());
        assert!(app.state.editing_task.is_none());
    }

    #[test]
    fn test_handle_insert_mode_backspace() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        app.state.mode = Mode::Insert;
        app.state.input_buffer = "test".to_string();

        let event = create_key_event(KeyCode::Backspace);
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.input_buffer, "tes");
    }

    #[test]
    fn test_handle_insert_mode_char() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        app.state.mode = Mode::Insert;
        app.state.input_buffer = "test".to_string();

        let event = create_key_event(KeyCode::Char('x'));
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.input_buffer, "testx");
    }

    #[test]
    fn test_handle_insert_mode_enter_with_editing_task() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        app.state.mode = Mode::Insert;
        app.state.input_buffer = "Updated Title".to_string();
        app.state.editing_task = Some(create_test_task("test", "Original Title"));
        app.state.editing_field = 0; // title field

        let event = create_key_event(KeyCode::Enter);
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.mode, Mode::Normal);
        assert!(app.state.input_buffer.is_empty());
    }

    #[test]
    fn test_handle_insert_mode_enter_without_editing_task() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        app.state.mode = Mode::Insert;
        app.state.input_buffer = "New Task".to_string();
        app.state.editing_task = None;

        let event = create_key_event(KeyCode::Enter);
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.mode, Mode::Normal);
        assert!(app.state.input_buffer.is_empty());
    }

    #[test]
    fn test_handle_insert_mode_tab() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        app.state.mode = Mode::Insert;
        app.state.editing_task = Some(create_test_task("test", "Test"));
        app.state.editing_field = 0;
        app.state.input_buffer = "test input".to_string();

        let event = create_key_event(KeyCode::Tab);
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.editing_field, 1);
    }

    #[test]
    fn test_handle_insert_mode_tab_wrap_around() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        app.state.mode = Mode::Insert;
        app.state.editing_task = Some(create_test_task("test", "Test"));
        app.state.editing_field = 3; // last field
        app.state.input_buffer = "test input".to_string();

        let event = create_key_event(KeyCode::Tab);
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.editing_field, 0); // wraps to first field
    }

    #[test]
    fn test_handle_insert_mode_backtab() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        app.state.mode = Mode::Insert;
        app.state.editing_task = Some(create_test_task("test", "Test"));
        app.state.editing_field = 1;
        app.state.input_buffer = "test input".to_string();

        let event = create_key_event(KeyCode::BackTab);
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.editing_field, 0);
    }

    #[test]
    fn test_handle_insert_mode_backtab_wrap_around() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        app.state.mode = Mode::Insert;
        app.state.editing_task = Some(create_test_task("test", "Test"));
        app.state.editing_field = 0; // first field
        app.state.input_buffer = "test input".to_string();

        let event = create_key_event(KeyCode::BackTab);
        handler.handle_event(event, &mut app);

        assert_eq!(app.state.editing_field, 3); // wraps to last field
    }

    #[test]
    fn test_handle_normal_mode_navigation() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        // Add some tasks
        app.state.tasks.push(create_test_task("1", "Task 1"));
        app.state.tasks.push(create_test_task("2", "Task 2"));
        app.state.tasks.push(create_test_task("3", "Task 3"));

        // Test j (next)
        let event = create_key_event(KeyCode::Char('j'));
        handler.handle_event(event, &mut app);

        // Test k (previous)
        let event = create_key_event(KeyCode::Char('k'));
        handler.handle_event(event, &mut app);

        // Test g (first)
        let event = create_key_event(KeyCode::Char('g'));
        handler.handle_event(event, &mut app);

        // Test G (last)
        let event = create_key_event(KeyCode::Char('G'));
        handler.handle_event(event, &mut app);

        // All should execute without panicking
        // Test passes if no panic occurs
    }

    #[test]
    fn test_handle_task_completion_toggle() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        // Add a task
        let task = create_test_task("1", "Test Task");
        app.state.tasks.push(task);

        let event = create_key_event(KeyCode::Char('!'));
        handler.handle_event(event, &mut app);

        // Should execute without panicking
        // Test passes if no panic occurs
    }

    #[test]
    fn test_handle_task_selection_toggle() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        let event = create_key_event(KeyCode::Char('x'));
        handler.handle_event(event, &mut app);

        // Should execute without panicking
        // Test passes if no panic occurs
    }

    #[test]
    fn test_handle_delete_tasks() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        let event = create_key_event(KeyCode::Char('D'));
        handler.handle_event(event, &mut app);

        // Should execute without panicking
        // Test passes if no panic occurs
    }

    #[test]
    fn test_handle_unknown_normal_key() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        let original_mode = app.state.mode.clone();
        let original_quit = app.state.should_quit;

        let event = create_key_event(KeyCode::Char('z')); // unmapped key
        handler.handle_event(event, &mut app);

        // State should remain unchanged
        assert_eq!(app.state.mode, original_mode);
        assert_eq!(app.state.should_quit, original_quit);
    }

    #[test]
    fn test_handle_unknown_insert_key() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        app.state.mode = Mode::Insert;
        let original_buffer = app.state.input_buffer.clone();

        let event = create_key_event(KeyCode::Home); // unmapped key
        handler.handle_event(event, &mut app);

        // Input buffer should remain unchanged
        assert_eq!(app.state.input_buffer, original_buffer);
        assert_eq!(app.state.mode, Mode::Insert);
    }

    #[test]
    fn test_handle_non_key_event() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        let original_mode = app.state.mode.clone();
        let original_quit = app.state.should_quit;

        let event = Event::Mouse(ratatui::crossterm::event::MouseEvent {
            kind: ratatui::crossterm::event::MouseEventKind::Down(
                ratatui::crossterm::event::MouseButton::Left,
            ),
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        });
        handler.handle_event(event, &mut app);

        // State should remain unchanged
        assert_eq!(app.state.mode, original_mode);
        assert_eq!(app.state.should_quit, original_quit);
    }

    #[test]
    fn test_handle_key_release_event() {
        let handler = EventHandler::new();
        let mut app = create_test_app();

        let original_mode = app.state.mode.clone();
        let original_quit = app.state.should_quit;

        let event = Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Release, // Release, not Press
            state: ratatui::crossterm::event::KeyEventState::NONE,
        });
        handler.handle_event(event, &mut app);

        // State should remain unchanged since we only handle Press events
        assert_eq!(app.state.mode, original_mode);
        assert_eq!(app.state.should_quit, original_quit);
    }
}
