use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    Normal,
    Insert,
    Command,
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
pub struct AppState {
    pub mode: Mode,
    pub tasks: Vec<Task>,
    pub selected_index: usize,
    pub should_quit: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            tasks: Vec::new(),
            selected_index: 0,
            should_quit: false,
        }
    }
}
