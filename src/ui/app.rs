use std::collections::HashSet;

use crate::{
    storage::{Db, DbError},
    types::{AppState, Task},
};
use ratatui::widgets::TableState;
use uuid::Uuid;

pub struct App<D: Db> {
    pub state: AppState<D>,
    pub message: Option<String>,
    pub task_list_state: TableState,
    task_selection: HashSet<usize>,
}

impl<D: Db> App<D> {
    pub fn new(state: AppState<D>) -> Self {
        Self {
            state,
            message: None,
            task_list_state: TableState::default(),
            task_selection: HashSet::default(),
        }
    }

    pub fn add_task(&mut self, title: &str) -> Result<(), DbError> {
        let new_task = self.create_task(title);
        self.state.tasks.push(new_task);
        self.sync_to_storage()
    }

    pub fn toggle_task_completion(&mut self) -> Result<(), DbError> {
        self.apply_to_selection(|t| t.completed = !t.completed);
        self.clear_task_selection();
        Ok(())
    }

    pub fn delete_tasks(&mut self) -> Result<(), DbError> {
        let mut indices: Vec<usize> = self.selection().collect();
        indices.sort();

        for index in indices.iter().rev() {
            if *index < self.state.tasks.len() {
                self.state.tasks.swap_remove(*index);
            }
        }
        self.sync_to_storage()?;
        self.clear_task_selection();
        Ok(())
    }

    pub fn quit(&mut self) {
        self.state.should_quit = true;
    }

    pub fn clear_input_buffer(&mut self) {
        self.state.input_buffer.clear();
    }

    pub fn add_to_input_buffer(&mut self, c: char) {
        self.state.input_buffer.push(c);
    }

    pub fn backspace_input_buffer(&mut self) {
        self.state.input_buffer.pop();
    }

    pub fn set_error_message(&mut self, message: String) {
        self.message = Some(message);
    }

    pub fn clear_error_message(&mut self) {
        self.message = None;
    }

    fn create_task(&self, title: &str) -> Task {
        Task {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: String::new(),
            completed: false,
            created_at: std::time::SystemTime::now(),
        }
    }

    fn apply_to_selection<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut Task),
    {
        let indices: Vec<usize> = self.selection().collect();
        for index in indices {
            if let Some(task) = self.state.tasks.get_mut(index) {
                func(task);
            }
        }
        self.sync_to_storage().unwrap_or_else(|e| {
            self.set_error_message(format!("Error syncing tasks: {}", e));
        });
        self.clear_task_selection();
    }

    fn sync_to_storage(&mut self) -> Result<(), DbError> {
        self.state.store.clear()?;
        for task in &self.state.tasks {
            self.state.store.save_task(task)?;
        }
        Ok(())
    }

    // Task list selection methods
    pub fn cursor_next_task(&mut self) {
        self.task_list_state.select_next();
    }

    pub fn cursor_previous_task(&mut self) {
        self.task_list_state.select_previous();
    }

    pub fn cursor_first_task(&mut self) {
        self.task_list_state.select_first();
    }

    pub fn cursor_last_task(&mut self) {
        self.task_list_state.select_last();
    }

    pub fn cursor_task_index(&self) -> Option<usize> {
        self.task_list_state.selected()
    }

    pub fn clear_task_selection(&mut self) {
        self.task_selection.clear();
    }

    pub fn selection(&self) -> SelectionIterator<'_> {
        if !self.task_selection.is_empty() {
            SelectionIterator::Multiple(self.task_selection.iter())
        } else if let Some(selected) = self.task_list_state.selected() {
            SelectionIterator::Single(std::iter::once(selected))
        } else {
            SelectionIterator::Empty
        }
    }

    pub fn toggle_task_selection(&mut self) {
        if let Some(selected) = self.task_list_state.selected() {
            if self.task_selection.contains(&selected) {
                self.task_selection.remove(&selected);
            } else {
                self.task_selection.insert(selected);
            }
        }
    }

    pub fn task_list_state(&mut self) -> &mut TableState {
        &mut self.task_list_state
    }

    pub fn get_task_selection(&self) -> &HashSet<usize> {
        &self.task_selection
    }

    pub fn create_task_below_cursor(&mut self) {
        let new_task = self.create_task("");
        let cursor_index = self.task_list_state.selected().unwrap_or(0);
        let insert_index = if self.state.tasks.is_empty() {
            0
        } else {
            (cursor_index + 1).min(self.state.tasks.len())
        };
        self.state.tasks.insert(insert_index, new_task.clone());
        self.state.editing_task = Some(new_task);
        self.state.editing_field = 0;
        self.task_list_state.select(Some(insert_index));
    }

    pub fn create_task_above_cursor(&mut self) {
        let new_task = self.create_task("");
        let cursor_index = self.task_list_state.selected().unwrap_or(0);
        self.state.tasks.insert(cursor_index, new_task.clone());
        self.state.editing_task = Some(new_task);
        self.state.editing_field = 0;
        self.task_list_state.select(Some(cursor_index));
    }

    pub fn save_editing_task(&mut self) -> Result<(), DbError> {
        if let Some(editing_task) = &self.state.editing_task {
            if let Some(selected_index) = self.task_list_state.selected() {
                if selected_index < self.state.tasks.len() {
                    self.state.tasks[selected_index] = editing_task.clone();
                    self.sync_to_storage()?;
                }
            }
        }
        self.state.editing_task = None;
        Ok(())
    }

    pub fn update_editing_task_field(&mut self, field_index: usize, value: String) {
        if let Some(ref mut editing_task) = self.state.editing_task {
            match field_index {
                0 => editing_task.title = value,
                1 => editing_task.description = value,
                _ => {}
            }
        }
    }

    pub fn get_editing_task_field(&self, field_index: usize) -> String {
        if let Some(ref editing_task) = self.state.editing_task {
            match field_index {
                0 => editing_task.title.clone(),
                1 => editing_task.description.clone(),
                _ => String::new(),
            }
        } else {
            String::new()
        }
    }
}

pub enum SelectionIterator<'a> {
    Multiple(std::collections::hash_set::Iter<'a, usize>),
    Single(std::iter::Once<usize>),
    Empty,
}

impl<'a> Iterator for SelectionIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SelectionIterator::Multiple(iter) => iter.next().copied(),
            SelectionIterator::Single(iter) => iter.next(),
            SelectionIterator::Empty => None,
        }
    }
}
