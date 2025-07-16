use crate::{
    storage::{Db, DbError},
    types::{AppState, Task},
};
use uuid::Uuid;

pub struct App<D: Db> {
    pub state: AppState<D>,
}

impl<D: Db> App<D> {
    pub fn new(state: AppState<D>) -> Self {
        Self { state }
    }

    pub fn add_task(&mut self, title: &str) -> Result<(), DbError> {
        let new_task = self.create_task(title);
        self.state.tasks.push(new_task);
        self.sync_to_storage()
    }

    pub fn toggle_task_completion(&mut self, index: usize) -> Result<(), DbError> {
        if let Some(task) = self.state.tasks.get_mut(index) {
            task.completed = !task.completed;
            self.sync_to_storage()?;
        }
        Ok(())
    }

    pub fn delete_task(&mut self, index: usize) -> Result<(), DbError> {
        if index < self.state.tasks.len() {
            self.state.tasks.remove(index);
            self.sync_to_storage()?;
        }
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
        self.state.message = Some(message);
    }

    pub fn clear_error_message(&mut self) {
        self.state.message = None;
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

    fn sync_to_storage(&mut self) -> Result<(), DbError> {
        self.state.store.clear()?;
        for task in &self.state.tasks {
            self.state.store.save_task(task)?;
        }
        Ok(())
    }
}
