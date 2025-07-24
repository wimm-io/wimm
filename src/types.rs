use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::SystemTime};

use crate::storage::{Db, MemoryStorage};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    Normal,
    Insert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    /// When the task was created (read-only in UI)
    pub created_at: SystemTime,
    /// Optional due date - when the task should be completed
    pub due: Option<SystemTime>,
    /// Optional defer date - when to start working on the task
    pub defer_until: Option<SystemTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState<T: Db = MemoryStorage> {
    pub mode: Mode,
    pub should_quit: bool,
    pub input_buffer: String,
    pub show_help: bool,
    pub tasks: Vec<Task>,
    pub store: T,
    pub editing_task: Option<Task>,
    pub editing_field: usize,
}

impl<T: Db> AppState<T> {
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
