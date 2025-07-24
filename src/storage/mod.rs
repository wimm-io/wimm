//! Storage abstraction layer for task persistence
//!
//! This module provides a trait-based abstraction for storing and retrieving tasks,
//! allowing the application to use different storage backends (memory, disk, etc.)
//! while maintaining a consistent interface.
//!
//! The module includes:
//! - [`Db`] trait defining the storage interface
//! - [`SledStorage`] for persistent storage using the Sled embedded database
//! - [`MemoryStorage`] for in-memory storage (testing and development)
//! - [`DbError`] for comprehensive error handling

use std::{collections::HashMap, path::Path};

use sled::open;
use thiserror::Error;

use crate::types::Task;

/// Comprehensive error types for database operations
///
/// This enum covers all possible failure modes when interacting with storage,
/// providing detailed error information for debugging and user feedback.
#[derive(Debug, Error)]
pub enum DbError {
    /// Failed to establish or maintain database connection
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    /// JSON serialization/deserialization failed
    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(String),
    /// Requested task ID does not exist in storage
    #[error("Task not found: {0}")]
    NotFound(String),
    /// Generic database operation failure
    #[error("Database operation failed: {0}")]
    OperationFailed(String),
}

/// Storage abstraction trait for task persistence
///
/// This trait defines a consistent interface for all storage backends,
/// allowing the application to be storage-agnostic. Implementations
/// can provide different persistence strategies (memory, disk, remote, etc.)
/// while maintaining the same API contract.
pub trait Db {
    /// Load all tasks from storage
    ///
    /// Returns a vector of all stored tasks. For empty storage,
    /// returns an empty vector rather than an error.
    fn load_tasks(&self) -> Result<Vec<Task>, DbError>;

    /// Save or update a task in storage
    ///
    /// If a task with the same ID already exists, it will be overwritten.
    /// The task ID serves as the primary key for storage operations.
    fn save_task(&mut self, task: &Task) -> Result<(), DbError>;

    /// Remove a task from storage by ID
    ///
    /// Returns an error if the task ID doesn't exist in storage.
    fn delete_task(&mut self, task_id: &str) -> Result<(), DbError>;

    /// Remove all tasks from storage
    ///
    /// This operation is irreversible and will permanently delete all stored tasks.
    fn clear(&mut self) -> Result<(), DbError>;
}

/// Persistent storage implementation using the Sled embedded database
///
/// SledStorage provides durable, ACID-compliant storage for tasks using
/// the Sled embedded key-value store. Data is automatically persisted to
/// disk and survives application restarts.
///
/// Features:
/// - ACID transactions
/// - Crash recovery
/// - Lock-free concurrent access
/// - Automatic compression
#[derive(Debug)]
pub struct SledStorage {
    /// The underlying Sled database instance
    inner: sled::Db,
}

impl SledStorage {
    /// Create a new Sled storage instance at the specified path
    ///
    /// The path can be a file or directory. Sled will create the necessary
    /// files and directory structure if they don't exist. The database
    /// will be opened with default configuration optimized for general use.
    ///
    /// # Arguments
    /// * `path` - File system path where the database should be stored
    ///
    /// # Errors
    /// Returns `DbError::ConnectionError` if the database cannot be opened,
    /// typically due to permission issues or invalid paths.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DbError> {
        let db = open(path).map_err(|e| DbError::ConnectionError(e.to_string()))?;
        Ok(Self { inner: db })
    }
}

/// In-memory storage implementation for tasks
///
/// MemoryStorage provides a simple, fast storage backend that keeps all
/// data in RAM. This is useful for testing, development, and scenarios
/// where persistence is not required. All data is lost when the application exits.
///
/// This implementation is thread-safe when used within a single thread,
/// but does not provide any synchronization for concurrent access.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryStorage {
    /// Internal hashmap storing tasks by ID
    tasks: HashMap<String, Task>,
}

impl MemoryStorage {
    /// Create a new memory storage instance with initial tasks
    ///
    /// # Arguments
    /// * `tasks` - Initial tasks to populate the storage with
    pub fn new(tasks: HashMap<String, Task>) -> Self {
        Self { tasks }
    }
}

impl Db for MemoryStorage {
    fn load_tasks(&self) -> Result<Vec<Task>, DbError> {
        // Convert hashmap values to vector, cloning each task
        Ok(self.tasks.values().cloned().collect())
    }

    fn save_task(&mut self, task: &Task) -> Result<(), DbError> {
        // Insert or update task using ID as key
        self.tasks.insert(task.id.clone(), task.clone());
        Ok(())
    }

    fn delete_task(&mut self, task_id: &str) -> Result<(), DbError> {
        // Remove task by ID, returning NotFound error if it doesn't exist
        self.tasks
            .remove(task_id)
            .ok_or_else(|| DbError::NotFound(task_id.to_string()))?;
        Ok(())
    }

    fn clear(&mut self) -> Result<(), DbError> {
        // Remove all tasks from memory
        self.tasks.clear();
        Ok(())
    }
}

impl Db for SledStorage {
    fn load_tasks(&self) -> Result<Vec<Task>, DbError> {
        // Iterate over all key-value pairs and collect the values
        let values = self
            .inner
            .iter()
            .values()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| DbError::OperationFailed(e.to_string()))?;

        // Deserialize each JSON value back into a Task struct
        values
            .iter()
            .map(|v| serde_json::from_slice(v).map_err(DbError::from))
            .collect::<Result<Vec<Task>, _>>()
    }

    fn save_task(&mut self, task: &Task) -> Result<(), DbError> {
        // Serialize task to JSON bytes for storage
        let serialized = serde_json::to_vec(task)?;
        // Insert into Sled database using task ID as key
        self.inner
            .insert(&task.id, serialized)
            .map_err(|e| DbError::OperationFailed(e.to_string()))?;
        Ok(())
    }

    fn delete_task(&mut self, task_id: &str) -> Result<(), DbError> {
        // Remove from database and verify the key existed
        self.inner
            .remove(task_id)
            .map_err(|e| DbError::OperationFailed(e.to_string()))?
            .ok_or_else(|| DbError::NotFound(task_id.to_string()))?;
        Ok(())
    }

    fn clear(&mut self) -> Result<(), DbError> {
        // Remove all key-value pairs from the database
        self.inner
            .clear()
            .map_err(|e| DbError::OperationFailed(e.to_string()))?;
        Ok(())
    }
}

/// Convert JSON serialization errors to database errors
///
/// This implementation allows automatic conversion from serde_json errors
/// to our DbError type, providing consistent error handling throughout
/// the storage layer.
impl From<serde_json::Error> for DbError {
    fn from(err: serde_json::Error) -> Self {
        DbError::SerdeError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::SystemTime;
    use tempfile::TempDir;

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
    fn test_db_error_display() {
        let connection_error = DbError::ConnectionError("connection failed".to_string());
        assert_eq!(
            format!("{connection_error}"),
            "Database connection error: connection failed"
        );

        let serde_error = DbError::SerdeError("serialization failed".to_string());
        assert_eq!(
            format!("{serde_error}"),
            "Serialization/Deserialization error: serialization failed"
        );

        let not_found_error = DbError::NotFound("task123".to_string());
        assert_eq!(format!("{not_found_error}"), "Task not found: task123");

        let operation_error = DbError::OperationFailed("operation failed".to_string());
        assert_eq!(
            format!("{operation_error}"),
            "Database operation failed: operation failed"
        );
    }

    #[test]
    fn test_db_error_from_serde_json() {
        // Create a serde error by trying to parse invalid JSON
        let invalid_json = "{invalid";
        let serde_error = serde_json::from_str::<serde_json::Value>(invalid_json).unwrap_err();
        let db_error: DbError = serde_error.into();

        match db_error {
            DbError::SerdeError(_) => {}
            _ => panic!("Expected SerdeError"),
        }
    }

    mod memory_storage_tests {
        use super::*;

        #[test]
        fn test_memory_storage_new() {
            let tasks = HashMap::new();
            let storage = MemoryStorage::new(tasks);
            assert!(storage.tasks.is_empty());
        }

        #[test]
        fn test_memory_storage_load_tasks_empty() {
            let storage = MemoryStorage::new(HashMap::new());
            let tasks = storage.load_tasks().unwrap();
            assert!(tasks.is_empty());
        }

        #[test]
        fn test_memory_storage_load_tasks_with_data() {
            let mut initial_tasks = HashMap::new();
            let task1 = create_test_task("1", "Task 1");
            let task2 = create_test_task("2", "Task 2");

            initial_tasks.insert("1".to_string(), task1.clone());
            initial_tasks.insert("2".to_string(), task2.clone());

            let storage = MemoryStorage::new(initial_tasks);
            let loaded_tasks = storage.load_tasks().unwrap();

            assert_eq!(loaded_tasks.len(), 2);
            assert!(
                loaded_tasks
                    .iter()
                    .any(|t| t.id == "1" && t.title == "Task 1")
            );
            assert!(
                loaded_tasks
                    .iter()
                    .any(|t| t.id == "2" && t.title == "Task 2")
            );
        }

        #[test]
        fn test_memory_storage_save_task() {
            let mut storage = MemoryStorage::new(HashMap::new());
            let task = create_test_task("test123", "Test Task");

            storage.save_task(&task).unwrap();

            let loaded_tasks = storage.load_tasks().unwrap();
            assert_eq!(loaded_tasks.len(), 1);
            assert_eq!(loaded_tasks[0].id, "test123");
            assert_eq!(loaded_tasks[0].title, "Test Task");
        }

        #[test]
        fn test_memory_storage_save_task_overwrite() {
            let mut storage = MemoryStorage::new(HashMap::new());
            let task1 = create_test_task("same_id", "Original Task");
            let mut task2 = create_test_task("same_id", "Updated Task");
            task2.completed = true;

            storage.save_task(&task1).unwrap();
            storage.save_task(&task2).unwrap();

            let loaded_tasks = storage.load_tasks().unwrap();
            assert_eq!(loaded_tasks.len(), 1);
            assert_eq!(loaded_tasks[0].title, "Updated Task");
            assert!(loaded_tasks[0].completed);
        }

        #[test]
        fn test_memory_storage_delete_task() {
            let mut initial_tasks = HashMap::new();
            let task1 = create_test_task("1", "Task 1");
            let task2 = create_test_task("2", "Task 2");

            initial_tasks.insert("1".to_string(), task1);
            initial_tasks.insert("2".to_string(), task2);

            let mut storage = MemoryStorage::new(initial_tasks);

            storage.delete_task("1").unwrap();

            let loaded_tasks = storage.load_tasks().unwrap();
            assert_eq!(loaded_tasks.len(), 1);
            assert_eq!(loaded_tasks[0].id, "2");
        }

        #[test]
        fn test_memory_storage_delete_task_not_found() {
            let mut storage = MemoryStorage::new(HashMap::new());

            let result = storage.delete_task("nonexistent");

            assert!(result.is_err());
            match result.unwrap_err() {
                DbError::NotFound(id) => assert_eq!(id, "nonexistent"),
                _ => panic!("Expected NotFound error"),
            }
        }

        #[test]
        fn test_memory_storage_clear() {
            let mut initial_tasks = HashMap::new();
            initial_tasks.insert("1".to_string(), create_test_task("1", "Task 1"));
            initial_tasks.insert("2".to_string(), create_test_task("2", "Task 2"));

            let mut storage = MemoryStorage::new(initial_tasks);

            storage.clear().unwrap();

            let loaded_tasks = storage.load_tasks().unwrap();
            assert!(loaded_tasks.is_empty());
        }
    }

    mod sled_storage_tests {
        use super::*;

        #[test]
        fn test_sled_storage_new() {
            let temp_dir = TempDir::new().unwrap();
            let db_path = temp_dir.path().join("test.db");

            let storage = SledStorage::new(&db_path);
            assert!(storage.is_ok());
        }

        #[test]
        fn test_sled_storage_new_invalid_path() {
            // Try to create database in a non-existent directory
            let result = SledStorage::new("/non/existent/path/test.db");
            assert!(result.is_err());

            match result.unwrap_err() {
                DbError::ConnectionError(_) => {}
                _ => panic!("Expected ConnectionError"),
            }
        }

        #[test]
        fn test_sled_storage_save_and_load_task() {
            let temp_dir = TempDir::new().unwrap();
            let db_path = temp_dir.path().join("test.db");

            let mut storage = SledStorage::new(&db_path).unwrap();
            let task = create_test_task("test123", "Test Task");

            storage.save_task(&task).unwrap();

            let loaded_tasks = storage.load_tasks().unwrap();
            assert_eq!(loaded_tasks.len(), 1);
            assert_eq!(loaded_tasks[0].id, "test123");
            assert_eq!(loaded_tasks[0].title, "Test Task");
        }

        #[test]
        fn test_sled_storage_multiple_tasks() {
            let temp_dir = TempDir::new().unwrap();
            let db_path = temp_dir.path().join("test.db");

            let mut storage = SledStorage::new(&db_path).unwrap();
            let task1 = create_test_task("1", "Task 1");
            let task2 = create_test_task("2", "Task 2");
            let task3 = create_test_task("3", "Task 3");

            storage.save_task(&task1).unwrap();
            storage.save_task(&task2).unwrap();
            storage.save_task(&task3).unwrap();

            let loaded_tasks = storage.load_tasks().unwrap();
            assert_eq!(loaded_tasks.len(), 3);

            let ids: Vec<String> = loaded_tasks.iter().map(|t| t.id.clone()).collect();
            assert!(ids.contains(&"1".to_string()));
            assert!(ids.contains(&"2".to_string()));
            assert!(ids.contains(&"3".to_string()));
        }

        #[test]
        fn test_sled_storage_delete_task() {
            let temp_dir = TempDir::new().unwrap();
            let db_path = temp_dir.path().join("test.db");

            let mut storage = SledStorage::new(&db_path).unwrap();
            let task1 = create_test_task("1", "Task 1");
            let task2 = create_test_task("2", "Task 2");

            storage.save_task(&task1).unwrap();
            storage.save_task(&task2).unwrap();

            storage.delete_task("1").unwrap();

            let loaded_tasks = storage.load_tasks().unwrap();
            assert_eq!(loaded_tasks.len(), 1);
            assert_eq!(loaded_tasks[0].id, "2");
        }

        #[test]
        fn test_sled_storage_delete_task_not_found() {
            let temp_dir = TempDir::new().unwrap();
            let db_path = temp_dir.path().join("test.db");

            let mut storage = SledStorage::new(&db_path).unwrap();

            let result = storage.delete_task("nonexistent");
            assert!(result.is_err());

            match result.unwrap_err() {
                DbError::NotFound(id) => assert_eq!(id, "nonexistent"),
                _ => panic!("Expected NotFound error"),
            }
        }

        #[test]
        fn test_sled_storage_clear() {
            let temp_dir = TempDir::new().unwrap();
            let db_path = temp_dir.path().join("test.db");

            let mut storage = SledStorage::new(&db_path).unwrap();
            let task1 = create_test_task("1", "Task 1");
            let task2 = create_test_task("2", "Task 2");

            storage.save_task(&task1).unwrap();
            storage.save_task(&task2).unwrap();

            storage.clear().unwrap();

            let loaded_tasks = storage.load_tasks().unwrap();
            assert!(loaded_tasks.is_empty());
        }

        #[test]
        fn test_sled_storage_persistence() {
            let temp_dir = TempDir::new().unwrap();
            let db_path = temp_dir.path().join("test.db");

            // Create storage, save task, and drop it
            {
                let mut storage = SledStorage::new(&db_path).unwrap();
                let task = create_test_task("persistent", "Persistent Task");
                storage.save_task(&task).unwrap();
            }

            // Create new storage instance with same path
            {
                let storage = SledStorage::new(&db_path).unwrap();
                let loaded_tasks = storage.load_tasks().unwrap();
                assert_eq!(loaded_tasks.len(), 1);
                assert_eq!(loaded_tasks[0].id, "persistent");
                assert_eq!(loaded_tasks[0].title, "Persistent Task");
            }
        }

        #[test]
        fn test_sled_storage_overwrite_task() {
            let temp_dir = TempDir::new().unwrap();
            let db_path = temp_dir.path().join("test.db");

            let mut storage = SledStorage::new(&db_path).unwrap();
            let task1 = create_test_task("same_id", "Original Task");
            let mut task2 = create_test_task("same_id", "Updated Task");
            task2.completed = true;

            storage.save_task(&task1).unwrap();
            storage.save_task(&task2).unwrap();

            let loaded_tasks = storage.load_tasks().unwrap();
            assert_eq!(loaded_tasks.len(), 1);
            assert_eq!(loaded_tasks[0].title, "Updated Task");
            assert!(loaded_tasks[0].completed);
        }
    }
}
