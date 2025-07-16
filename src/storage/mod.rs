use std::{collections::HashMap, path::Path};

use sled::open;
use thiserror::Error;

use crate::types::Task;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(String),
    #[error("Task not found: {0}")]
    NotFound(String),
    #[error("Database operation failed: {0}")]
    OperationFailed(String),
}

pub trait Db {
    fn load_tasks(&self) -> Result<Vec<Task>, DbError>;
    fn save_task(&mut self, task: &Task) -> Result<(), DbError>;
    fn delete_task(&mut self, task_id: &str) -> Result<(), DbError>;
    fn clear(&mut self) -> Result<(), DbError>;
}

pub struct SledStorage {
    inner: sled::Db,
}

impl SledStorage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DbError> {
        let db = open(path).map_err(|e| DbError::ConnectionError(e.to_string()))?;
        Ok(Self { inner: db })
    }
}

pub struct MemoryStorage {
    tasks: HashMap<String, Task>,
}

impl MemoryStorage {
    pub fn new(tasks: HashMap<String, Task>) -> Self {
        Self { tasks }
    }
}

impl Db for MemoryStorage {
    fn load_tasks(&self) -> Result<Vec<Task>, DbError> {
        Ok(self.tasks.values().cloned().collect())
    }

    fn save_task(&mut self, task: &Task) -> Result<(), DbError> {
        self.tasks.insert(task.id.clone(), task.clone());
        Ok(())
    }

    fn delete_task(&mut self, task_id: &str) -> Result<(), DbError> {
        self.tasks
            .remove(task_id)
            .ok_or_else(|| DbError::NotFound(task_id.to_string()))?;
        Ok(())
    }

    fn clear(&mut self) -> Result<(), DbError> {
        self.tasks.clear();
        Ok(())
    }
}

impl Db for SledStorage {
    fn load_tasks(&self) -> Result<Vec<Task>, DbError> {
        let values = self
            .inner
            .iter()
            .values()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DbError::OperationFailed(e.to_string()))?;

        values
            .iter()
            .map(|v| serde_json::from_slice(v).map_err(DbError::from))
            .collect::<Result<Vec<Task>, _>>()
    }

    fn save_task(&mut self, task: &Task) -> Result<(), DbError> {
        let serialized = serde_json::to_vec(task)?;
        self.inner
            .insert(&task.id, serialized)
            .map_err(|e| DbError::OperationFailed(e.to_string()))?;
        Ok(())
    }

    fn delete_task(&mut self, task_id: &str) -> Result<(), DbError> {
        self.inner
            .remove(task_id)
            .map_err(|e| DbError::OperationFailed(e.to_string()))?
            .ok_or_else(|| DbError::NotFound(task_id.to_string()))?;
        Ok(())
    }

    fn clear(&mut self) -> Result<(), DbError> {
        self.inner
            .clear()
            .map_err(|e| DbError::OperationFailed(e.to_string()))?;
        Ok(())
    }
}

impl From<serde_json::Error> for DbError {
    fn from(err: serde_json::Error) -> Self {
        DbError::SerdeError(err.to_string())
    }
}
