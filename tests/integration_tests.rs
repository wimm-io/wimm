use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::{Terminal, backend::TestBackend};
use std::collections::HashMap;
use std::time::SystemTime;
use tempfile::TempDir;
use wimm::storage::{Db, MemoryStorage, SledStorage};
use wimm::types::{AppState, Mode, Task};
use wimm::ui::app::App;
use wimm::ui::events::EventHandler;
use wimm::ui::help_panel::HelpPanel;
use wimm::ui::layout::LayoutManager;

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
fn test_memory_storage_integration() {
    let mut storage = MemoryStorage::new(HashMap::new());

    // Test saving and loading tasks
    let task1 = create_test_task("1", "First Task");
    let task2 = create_test_task("2", "Second Task");

    storage.save_task(&task1).unwrap();
    storage.save_task(&task2).unwrap();

    let loaded_tasks = storage.load_tasks().unwrap();
    assert_eq!(loaded_tasks.len(), 2);

    // Test deleting a task
    storage.delete_task("1").unwrap();
    let remaining_tasks = storage.load_tasks().unwrap();
    assert_eq!(remaining_tasks.len(), 1);
    assert_eq!(remaining_tasks[0].id, "2");

    // Test clearing all tasks
    storage.clear().unwrap();
    let empty_tasks = storage.load_tasks().unwrap();
    assert!(empty_tasks.is_empty());
}

#[test]
fn test_sled_storage_integration() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("integration_test.db");

    let mut storage = SledStorage::new(&db_path).unwrap();

    // Test saving and loading tasks
    let task1 = create_test_task("1", "Persistent Task 1");
    let task2 = create_test_task("2", "Persistent Task 2");

    storage.save_task(&task1).unwrap();
    storage.save_task(&task2).unwrap();

    let loaded_tasks = storage.load_tasks().unwrap();
    assert_eq!(loaded_tasks.len(), 2);

    // Test persistence by recreating storage
    drop(storage);
    let new_storage = SledStorage::new(&db_path).unwrap();
    let persistent_tasks = new_storage.load_tasks().unwrap();
    assert_eq!(persistent_tasks.len(), 2);
}

#[test]
fn test_app_state_with_storage_integration() {
    let storage = MemoryStorage::new(HashMap::new());
    let mut app_state = AppState::new(storage);

    // Test initial state
    assert_eq!(app_state.mode, Mode::Normal);
    assert!(!app_state.should_quit);
    assert!(app_state.tasks.is_empty());

    // Test adding tasks to state
    let task = create_test_task("test", "Test Task");
    app_state.tasks.push(task.clone());
    assert_eq!(app_state.tasks.len(), 1);
    assert_eq!(app_state.tasks[0].id, "test");

    // Test mode changes
    app_state.mode = Mode::Insert;
    assert_eq!(app_state.mode, Mode::Insert);

    // Test editing state
    app_state.editing_task = Some(task);
    assert!(app_state.editing_task.is_some());
}

#[test]
fn test_app_with_storage_integration() {
    let storage = MemoryStorage::new(HashMap::new());
    let state = AppState::new(storage);
    let mut app = App::new(state);

    // Test adding tasks through the app
    app.add_task("Integration Test Task").unwrap();
    assert_eq!(app.state.tasks.len(), 1);
    assert_eq!(app.state.tasks[0].title, "Integration Test Task");

    // Test navigation
    app.cursor_next_task();
    app.cursor_previous_task();
    app.cursor_first_task();
    app.cursor_last_task();

    // Test task operations
    app.toggle_task_selection();
    app.toggle_task_completion().unwrap();
}

#[test]
fn test_event_handler_with_app_integration() {
    let storage = MemoryStorage::new(HashMap::new());
    let state = AppState::new(storage);
    let mut app = App::new(state);
    let handler = EventHandler::new();

    // Add a task first
    app.add_task("Event Test Task").unwrap();

    // Test normal mode navigation
    let down_event = create_key_event(KeyCode::Char('j'));
    handler.handle_event(down_event, &mut app);

    let up_event = create_key_event(KeyCode::Char('k'));
    handler.handle_event(up_event, &mut app);

    // Test mode switching
    let insert_event = create_key_event(KeyCode::Char('i'));
    handler.handle_event(insert_event, &mut app);
    assert_eq!(app.state.mode, Mode::Insert);

    // Test escape back to normal
    let escape_event = create_key_event(KeyCode::Esc);
    handler.handle_event(escape_event, &mut app);
    assert_eq!(app.state.mode, Mode::Normal);

    // Test quit
    let quit_event = create_key_event(KeyCode::Char('q'));
    handler.handle_event(quit_event, &mut app);
    assert!(app.state.should_quit);
}

#[test]
fn test_task_crud_operations_integration() {
    let storage = MemoryStorage::new(HashMap::new());
    let state = AppState::new(storage);
    let mut app = App::new(state);
    let handler = EventHandler::new();

    // Create a new task using 'o' key
    let create_event = create_key_event(KeyCode::Char('o'));
    handler.handle_event(create_event, &mut app);
    assert_eq!(app.state.mode, Mode::Insert);
    assert!(app.state.editing_task.is_some());

    // Add some text
    let char_events = ['T', 'e', 's', 't'];
    for c in char_events {
        let char_event = create_key_event(KeyCode::Char(c));
        handler.handle_event(char_event, &mut app);
    }
    assert_eq!(app.state.input_buffer, "Test");

    // Save the task
    let enter_event = create_key_event(KeyCode::Enter);
    handler.handle_event(enter_event, &mut app);
    assert_eq!(app.state.mode, Mode::Normal);
    assert!(!app.state.tasks.is_empty());

    // Toggle completion
    let toggle_event = create_key_event(KeyCode::Char('!'));
    handler.handle_event(toggle_event, &mut app);

    // Delete the task
    app.toggle_task_selection(); // Select task first
    let delete_event = create_key_event(KeyCode::Char('D'));
    handler.handle_event(delete_event, &mut app);
}

#[test]
fn test_help_panel_integration() {
    let help_panel = HelpPanel::new();
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    // Test rendering help panel
    terminal
        .draw(|f| {
            let area = Rect::new(10, 5, 60, 14);
            help_panel.render(f, area);
        })
        .unwrap();

    // Test with different sizes
    let sizes = [(40, 10), (120, 30), (20, 8)];
    for (width, height) in sizes {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let area = f.area();
                help_panel.render(f, area);
            })
            .unwrap();
    }
}

#[test]
fn test_layout_manager_integration() {
    let layout_manager = LayoutManager::new();
    let storage = MemoryStorage::new(HashMap::new());
    let mut app_state = AppState::new(storage);

    // Test layout without help
    let area = Rect::new(0, 0, 80, 24);
    let layout = layout_manager.calculate_main_layout(area, &app_state);

    assert_eq!(layout.title.height, 1);
    assert_eq!(layout.status.height, 1);
    assert_eq!(layout.main.height, 22);
    assert!(layout.help.is_none());

    // Test layout with help
    app_state.show_help = true;
    let layout_with_help = layout_manager.calculate_main_layout(area, &app_state);
    assert!(layout_with_help.help.is_some());
}

#[test]
fn test_task_serialization_integration() {
    let task = create_test_task("serialize_test", "Serializable Task");

    // Test JSON serialization
    let json = serde_json::to_string(&task).unwrap();
    assert!(json.contains("serialize_test"));
    assert!(json.contains("Serializable Task"));

    // Test deserialization
    let deserialized: Task = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, task.id);
    assert_eq!(deserialized.title, task.title);
    assert_eq!(deserialized.description, task.description);
    assert_eq!(deserialized.completed, task.completed);
}

#[test]
fn test_app_state_properties_integration() {
    let storage = MemoryStorage::new(HashMap::new());
    let mut app_state = AppState::new(storage);

    // Add some data
    app_state.tasks.push(create_test_task("1", "Task 1"));
    app_state.input_buffer = "test input".to_string();
    app_state.mode = Mode::Insert;
    app_state.show_help = true;

    // Test that properties are set correctly
    assert_eq!(app_state.mode, Mode::Insert);
    assert_eq!(app_state.input_buffer, "test input");
    assert!(app_state.show_help);
    assert_eq!(app_state.tasks.len(), 1);
    assert_eq!(app_state.tasks[0].title, "Task 1");
}

#[test]
fn test_full_workflow_integration() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("workflow_test.db");

    // Create app with persistent storage
    let storage = SledStorage::new(&db_path).unwrap();
    let state = AppState::new(storage);
    let mut app = App::new(state);
    let handler = EventHandler::new();

    // 1. Create a new task
    let create_event = create_key_event(KeyCode::Char('o'));
    handler.handle_event(create_event, &mut app);

    // 2. Type task title
    for c in "Important Task".chars() {
        let char_event = create_key_event(KeyCode::Char(c));
        handler.handle_event(char_event, &mut app);
    }

    // 3. Move to description field
    let tab_event = create_key_event(KeyCode::Tab);
    handler.handle_event(tab_event, &mut app);

    // 4. Type description
    for c in "This is important".chars() {
        let char_event = create_key_event(KeyCode::Char(c));
        handler.handle_event(char_event, &mut app);
    }

    // 5. Save the task
    let enter_event = create_key_event(KeyCode::Enter);
    handler.handle_event(enter_event, &mut app);

    // 6. Verify task was created
    assert_eq!(app.state.tasks.len(), 1);
    assert_eq!(app.state.tasks[0].title, "Important Task");
    assert_eq!(app.state.tasks[0].description, "This is important");

    // 7. Toggle completion
    let toggle_event = create_key_event(KeyCode::Char('!'));
    handler.handle_event(toggle_event, &mut app);

    // 8. Show help
    let help_event = create_key_event(KeyCode::Char('h'));
    handler.handle_event(help_event, &mut app);
    assert!(app.state.show_help);

    // 9. Hide help
    let help_event = create_key_event(KeyCode::Char('h'));
    handler.handle_event(help_event, &mut app);
    assert!(!app.state.show_help);

    // 10. Create another task above
    let create_above_event = create_key_event(KeyCode::Char('O'));
    handler.handle_event(create_above_event, &mut app);

    for c in "Urgent Task".chars() {
        let char_event = create_key_event(KeyCode::Char(c));
        handler.handle_event(char_event, &mut app);
    }

    let enter_event = create_key_event(KeyCode::Enter);
    handler.handle_event(enter_event, &mut app);

    // 11. Verify we now have 2 tasks
    assert_eq!(app.state.tasks.len(), 2);

    // 12. Navigate between tasks
    let down_event = create_key_event(KeyCode::Char('j'));
    handler.handle_event(down_event, &mut app);

    let up_event = create_key_event(KeyCode::Char('k'));
    handler.handle_event(up_event, &mut app);

    // 13. Go to first and last
    let first_event = create_key_event(KeyCode::Char('g'));
    handler.handle_event(first_event, &mut app);

    let last_event = create_key_event(KeyCode::Char('G'));
    handler.handle_event(last_event, &mut app);

    // 14. Select and delete a task
    let select_event = create_key_event(KeyCode::Char('x'));
    handler.handle_event(select_event, &mut app);

    let delete_event = create_key_event(KeyCode::Char('D'));
    handler.handle_event(delete_event, &mut app);

    // The workflow completed successfully if we reach here
    // Test passes if no panic occurs
}

#[test]
fn test_date_parsing_integration() {
    let storage = MemoryStorage::new(HashMap::new());
    let state = AppState::new(storage);
    let app = App::new(state);

    // Test various date formats
    let test_cases = vec![
        ("2d", true),
        ("1w", true),
        ("3h", true),
        ("today", true),
        ("tomorrow", true),
        ("friday", true),
        ("2024-12-25", true),
        ("invalid", false),
        ("", false),
    ];

    for (input, should_parse) in test_cases {
        let result = app.parse_date_input(input, true);
        if should_parse {
            assert!(result.is_some(), "Failed to parse: {input}");
        } else {
            assert!(result.is_none(), "Should not parse: {input}");
        }
    }
}

#[test]
fn test_error_handling_integration() {
    let storage = MemoryStorage::new(HashMap::new());
    let state = AppState::new(storage);
    let mut app = App::new(state);

    // Test error scenarios
    app.set_error_message("Test error".to_string());
    assert!(app.get_error_message().is_some());

    app.clear_error_message();
    assert!(app.get_error_message().is_none());

    // Test operations that might fail gracefully
    let handler = EventHandler::new();

    // Try to delete non-existent task
    let delete_event = create_key_event(KeyCode::Char('D'));
    handler.handle_event(delete_event, &mut app);

    // Try to toggle completion on empty list
    let toggle_event = create_key_event(KeyCode::Char('!'));
    handler.handle_event(toggle_event, &mut app);

    // These should not panic
    // Test passes if no panic occurs
}

#[test]
fn test_multi_storage_backend_compatibility() {
    let task = create_test_task("compat_test", "Compatibility Test");

    // Test with MemoryStorage
    let mut memory_storage = MemoryStorage::new(HashMap::new());
    memory_storage.save_task(&task).unwrap();
    let memory_tasks = memory_storage.load_tasks().unwrap();
    assert_eq!(memory_tasks.len(), 1);

    // Test with SledStorage
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("compat_test.db");
    let mut sled_storage = SledStorage::new(&db_path).unwrap();
    sled_storage.save_task(&task).unwrap();
    let sled_tasks = sled_storage.load_tasks().unwrap();
    assert_eq!(sled_tasks.len(), 1);

    // Both should have the same task data
    assert_eq!(memory_tasks[0].id, sled_tasks[0].id);
    assert_eq!(memory_tasks[0].title, sled_tasks[0].title);
}
