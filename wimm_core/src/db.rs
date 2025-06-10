use std::{fs, path::Path, sync::OnceLock};

use crate::{error::WimmError, model::Task};
use log::debug;
use native_db::{Builder, Database, Models, db_type, transaction::RwTransaction};

static MODELS: OnceLock<Models> = OnceLock::new();

fn get_models() -> &'static Models {
    MODELS.get_or_init(|| {
        let mut models = Models::new();
        models
            .define::<Task>()
            .expect("Failed to define Task model");
        models
    })
}

pub struct Db<'a> {
    inner: Database<'a>,
}

impl<'a> Db<'a> {
    pub fn create(path: &Path, truncate_db: bool) -> Result<Db<'a>, WimmError> {
        let data_dir = path
            .parent()
            .ok_or_else(|| WimmError::DbError(format!("Invalid DB path: {}", path.display())))?;
        debug!("data_dir: {}", data_dir.display());

        if !data_dir.exists() {
            debug!("Creating data_dir: {}", data_dir.display());
            fs::create_dir_all(data_dir).map_err(|io_error| {
                WimmError::DbError(format!("Failed to create DB path: {io_error}"))
            })?;
        }

        if truncate_db {
            debug!("Truncating database at: {}", path.display());
            fs::remove_file(path).map_err(|io_error| {
                WimmError::DbError(format!("Failed to truncate DB path: {io_error}"))
            })?;
        }

        Ok(Db {
            inner: Builder::new().create(&get_models(), &path)?,
        })
    }

    pub fn delete_task(&self, id: &str) -> Result<(), WimmError> {
        let t = self.inner.rw_transaction()?;
        let task = get_task(id, &t)?;
        t.remove(task.clone())?;
        t.commit()?;
        Ok(())
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>, WimmError> {
        let t = self.inner.r_transaction()?;
        let tasks: Vec<Task> = t.scan().primary()?.all()?.collect::<Result<Vec<_>, _>>()?;
        Ok(tasks)
    }

    pub fn insert_task(&self, task: &Task) -> Result<(), WimmError> {
        debug!("insert_task(task: {task:?})");
        let t = self.inner.rw_transaction()?;
        t.insert(task.clone())?;
        t.commit()?;

        Ok(())
    }

    pub fn update_task<F>(&self, id: &str, task_updater: F) -> Result<(), WimmError>
    where
        F: FnOnce(&Task) -> Option<Task>,
    {
        let t = self.inner.rw_transaction()?;
        let task: Task = get_task(id, &t)?;
        debug!("Found task: {task:?}");
        if let Some(updated_task) = task_updater(&task) {
            debug!("Updating task to: {updated_task:?}");
            t.update(task, updated_task)?;
            t.commit()?;
        }
        Ok(())
    }
}

fn get_task(id: &str, t: &RwTransaction) -> Result<Task, WimmError> {
    t.get()
        .primary::<Task>(id.to_string())?
        .ok_or(WimmError::DbError(format!("Task not found for ID: {id}")))
}

impl From<db_type::Error> for WimmError {
    fn from(error: db_type::Error) -> Self {
        WimmError::DbError(format!("Database error: {error:?}"))
    }
}
