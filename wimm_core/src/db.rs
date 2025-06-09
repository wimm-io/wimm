use std::{fs, path::PathBuf, sync::OnceLock};

use crate::{
    error::WimmError,
    model::{Status, Task},
};
use native_db::{Builder, Database, Models, db_type, transaction::RwTransaction};
use uuid::Uuid;

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

pub struct Config {
    pub path: PathBuf,
}

pub struct Db<'a> {
    inner: Database<'a>,
}

impl<'a> Db<'a> {
    pub fn create(config: &Config) -> Result<Db<'a>, WimmError> {
        let data_dir = config.path.parent().ok_or_else(|| {
            WimmError::DbError(format!("Invalid DB path: {}", &config.path.display()))
        })?;
        if !data_dir.exists() {
            fs::create_dir_all(data_dir).map_err(|io_error| {
                WimmError::DbError(format!("Failed to create DB path: {io_error}"))
            })?;
        }

        Ok(Db {
            inner: Builder::new().create(&get_models(), &config.path)?,
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

    pub fn create_task(&self, name: &str) -> Result<Task, WimmError> {
        let task = Task {
            id: Uuid::new_v4().to_string(),
            name: String::from(name),
            status: Status::NotStarted,
        };

        let t = self.inner.rw_transaction()?;
        t.insert(task.clone())?;
        t.commit()?;

        Ok(task)
    }

    pub fn start_task(&self, id: &str) -> Result<(), WimmError> {
        let t = self.inner.rw_transaction()?;
        let task: Task = get_task(id, &t)?;
        let update = Task {
            status: Status::InProgress,
            ..task.clone()
        };
        t.update(task, update)?;
        t.commit()?;

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
