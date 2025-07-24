//! Core data types for the WIMM task management application
//!
//! This module defines the fundamental data structures used throughout the application,
//! including tasks, application state, and operational modes.

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::SystemTime};

use crate::storage::{Db, MemoryStorage};

/// Application input mode - determines how user input is interpreted
///
/// The application operates in different modes similar to vim:
/// - Normal mode: Navigate and execute commands
/// - Insert mode: Input text for creating/editing tasks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    /// Default mode for navigation and command execution
    Normal,
    /// Text input mode for creating and editing task content
    Insert,
}

/// Represents a single task in the task management system
///
/// Tasks are the core entity of the application, containing all information
/// needed to track work items including scheduling, completion status, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier for the task (typically a UUID)
    pub id: String,
    /// Short, descriptive name for the task
    pub title: String,
    /// Detailed description or notes about the task
    pub description: String,
    /// Whether the task has been completed
    pub completed: bool,
    /// When the task was created (immutable timestamp)
    pub created_at: SystemTime,
    /// Optional deadline - when the task should be completed
    pub due: Option<SystemTime>,
    /// Optional defer date - when to start working on the task (GTD-style)
    pub defer_until: Option<SystemTime>,
}

/// Global application state containing all runtime data and configuration
///
/// This structure holds everything needed to run the application, including
/// the current UI state, loaded tasks, and the storage backend. It's generic
/// over the storage type to allow different backends (memory vs persistent).
#[derive(Debug, Serialize, Deserialize)]
pub struct AppState<T: Db = MemoryStorage> {
    /// Current input mode (Normal or Insert)
    pub mode: Mode,
    /// Flag to signal the application should exit
    pub should_quit: bool,
    /// Buffer for text input in Insert mode
    pub input_buffer: String,
    /// Whether to display the help panel overlay
    pub show_help: bool,
    /// All loaded tasks from storage
    pub tasks: Vec<Task>,
    /// Storage backend for persistence (generic for testability)
    pub store: T,
    /// Task currently being edited (if any)
    pub editing_task: Option<Task>,
    /// Index of the field being edited (0=title, 1=description, etc.)
    pub editing_field: usize,
}

impl<T: Db> AppState<T> {
    /// Create a new application state with the given storage backend
    ///
    /// Initializes all fields to their default values with the application
    /// starting in Normal mode with no tasks loaded.
    pub fn new(store: T) -> Self {
        Self {
            mode: Mode::Normal,
            should_quit: false,
            input_buffer: String::new(),
            show_help: false,
            tasks: Vec::new(),
            store,
            editing_task: None,
            editing_field: 0,
        }
    }
}

impl Default for AppState {
    /// Create a default application state using in-memory storage
    ///
    /// This is primarily useful for testing and development scenarios
    /// where persistent storage is not needed.
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            should_quit: false,
            input_buffer: String::new(),
            show_help: false,
            tasks: Vec::new(),
            store: MemoryStorage::new(HashMap::new()),
            editing_task: None,
            editing_field: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

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

    #[test]
    fn test_mode_equality() {
        assert_eq!(Mode::Normal, Mode::Normal);
        assert_eq!(Mode::Insert, Mode::Insert);
        assert_ne!(Mode::Normal, Mode::Insert);
    }

    #[test]
    fn test_mode_clone() {
        let mode = Mode::Normal;
        let cloned_mode = mode.clone();
        assert_eq!(mode, cloned_mode);
    }

    #[test]
    fn test_task_creation() {
        let now = SystemTime::now();
        let task = Task {
            id: "test123".to_string(),
            title: "Test Task".to_string(),
            description: "This is a test task".to_string(),
            completed: false,
            created_at: now,
            due: None,
            defer_until: None,
        };

        assert_eq!(task.id, "test123");
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, "This is a test task");
        assert!(!task.completed);
        assert_eq!(task.created_at, now);
        assert!(task.due.is_none());
        assert!(task.defer_until.is_none());
    }

    #[test]
    fn test_task_with_dates() {
        let now = SystemTime::now();
        let due_date = now + Duration::from_secs(86400); // 1 day from now
        let defer_date = now + Duration::from_secs(3600); // 1 hour from now

        let task = Task {
            id: "dated_task".to_string(),
            title: "Task with dates".to_string(),
            description: "This task has due and defer dates".to_string(),
            completed: true,
            created_at: now,
            due: Some(due_date),
            defer_until: Some(defer_date),
        };

        assert!(task.completed);
        assert_eq!(task.due, Some(due_date));
        assert_eq!(task.defer_until, Some(defer_date));
    }

    #[test]
    fn test_task_serialization() {
        let task = create_test_task("serialize_test", "Serialize Task");

        let serialized = serde_json::to_string(&task).unwrap();
        assert!(serialized.contains("serialize_test"));
        assert!(serialized.contains("Serialize Task"));

        let deserialized: Task = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.id, task.id);
        assert_eq!(deserialized.title, task.title);
        assert_eq!(deserialized.description, task.description);
        assert_eq!(deserialized.completed, task.completed);
    }

    #[test]
    fn test_task_clone() {
        let original = create_test_task("clone_test", "Clone Task");
        let cloned = original.clone();

        assert_eq!(original.id, cloned.id);
        assert_eq!(original.title, cloned.title);
        assert_eq!(original.description, cloned.description);
        assert_eq!(original.completed, cloned.completed);
        assert_eq!(original.created_at, cloned.created_at);
        assert_eq!(original.due, cloned.due);
        assert_eq!(original.defer_until, cloned.defer_until);
    }

    #[test]
    fn test_appstate_new() {
        let store = MemoryStorage::new(HashMap::new());
        let app_state = AppState::new(store);

        assert_eq!(app_state.mode, Mode::Normal);
        assert!(!app_state.should_quit);
        assert!(app_state.input_buffer.is_empty());
        assert!(!app_state.show_help);
        assert!(app_state.tasks.is_empty());
        assert!(app_state.editing_task.is_none());
        assert_eq!(app_state.editing_field, 0);
    }

    #[test]
    fn test_appstate_default() {
        let app_state = AppState::default();

        assert_eq!(app_state.mode, Mode::Normal);
        assert!(!app_state.should_quit);
        assert!(app_state.input_buffer.is_empty());
        assert!(!app_state.show_help);
        assert!(app_state.tasks.is_empty());
        assert!(app_state.editing_task.is_none());
        assert_eq!(app_state.editing_field, 0);
    }

    #[test]
    fn test_appstate_with_tasks() {
        let mut store = MemoryStorage::new(HashMap::new());
        let task1 = create_test_task("1", "Task 1");
        let task2 = create_test_task("2", "Task 2");

        store.save_task(&task1).unwrap();
        store.save_task(&task2).unwrap();

        let mut app_state = AppState::new(store);
        app_state.tasks = vec![task1.clone(), task2.clone()];

        assert_eq!(app_state.tasks.len(), 2);
        assert_eq!(app_state.tasks[0].id, "1");
        assert_eq!(app_state.tasks[1].id, "2");
    }

    #[test]
    fn test_appstate_editing_task() {
        let store = MemoryStorage::new(HashMap::new());
        let mut app_state = AppState::new(store);
        let task = create_test_task("edit_test", "Editing Task");

        app_state.editing_task = Some(task.clone());
        app_state.editing_field = 2;

        assert!(app_state.editing_task.is_some());
        assert_eq!(app_state.editing_task.unwrap().id, "edit_test");
        assert_eq!(app_state.editing_field, 2);
    }

    #[test]
    fn test_appstate_mode_changes() {
        let store = MemoryStorage::new(HashMap::new());
        let mut app_state = AppState::new(store);

        assert_eq!(app_state.mode, Mode::Normal);

        app_state.mode = Mode::Insert;
        assert_eq!(app_state.mode, Mode::Insert);

        app_state.mode = Mode::Normal;
        assert_eq!(app_state.mode, Mode::Normal);
    }

    #[test]
    fn test_appstate_input_buffer() {
        let store = MemoryStorage::new(HashMap::new());
        let mut app_state = AppState::new(store);

        assert!(app_state.input_buffer.is_empty());

        app_state.input_buffer = "test input".to_string();
        assert_eq!(app_state.input_buffer, "test input");

        app_state.input_buffer.clear();
        assert!(app_state.input_buffer.is_empty());
    }

    #[test]
    fn test_appstate_flags() {
        let store = MemoryStorage::new(HashMap::new());
        let mut app_state = AppState::new(store);

        assert!(!app_state.should_quit);
        assert!(!app_state.show_help);

        app_state.should_quit = true;
        app_state.show_help = true;

        assert!(app_state.should_quit);
        assert!(app_state.show_help);
    }

    #[test]
    fn test_appstate_properties() {
        let app_state = AppState::default();

        // Test that default values are correct
        assert_eq!(app_state.mode, Mode::Normal);
        assert!(!app_state.should_quit);
        assert!(!app_state.show_help);
        assert!(app_state.input_buffer.is_empty());
        assert!(app_state.tasks.is_empty());
        assert_eq!(app_state.editing_field, 0);
        assert!(app_state.editing_task.is_none());
    }
}
