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
    pub created_at: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState<T: Db = MemoryStorage> {
    pub mode: Mode,
    pub should_quit: bool,
    pub input_buffer: String,
    pub show_help: bool,
    pub tasks: Vec<Task>,
    pub store: T,
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
        }
    }
}
