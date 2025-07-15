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
    fn save_task(&self, task: &Task) -> Result<(), DbError>;
    fn delete_task(&self, task_id: &str) -> Result<(), DbError>;
    fn flush(&self) -> Result<(), DbError>;
}

pub struct SledStorage {
    inner: sled::Db,
}

impl SledStorage {
    pub fn new(path: &str) -> Result<Self, DbError> {
        let db = open(path).map_err(|e| DbError::ConnectionError(e.to_string()))?;
        Ok(Self { inner: db })
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

    fn save_task(&self, task: &Task) -> Result<(), DbError> {
        let serialized = serde_json::to_vec(task)?;
        self.inner
            .insert(&task.id, serialized)
            .map_err(|e| DbError::OperationFailed(e.to_string()))?;
        Ok(())
    }

    fn delete_task(&self, task_id: &str) -> Result<(), DbError> {
        self.inner
            .remove(task_id)
            .map_err(|e| DbError::OperationFailed(e.to_string()))?
            .ok_or_else(|| DbError::NotFound(task_id.to_string()))?;
        Ok(())
    }

    fn flush(&self) -> Result<(), DbError> {
        self.inner
            .flush()
            .map_err(|e| DbError::OperationFailed(e.to_string()))?;
        Ok(())
    }
}

impl From<serde_json::Error> for DbError {
    fn from(err: serde_json::Error) -> Self {
        DbError::SerdeError(err.to_string())
    }
}
