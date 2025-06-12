use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use log::debug;
use uuid::Uuid;

use crate::{
    WimmError,
    db::Db,
    model::{Status, Task},
};

pub struct App {
    db: Db<'static>,
}

impl App {
    pub fn new(db_path: PathBuf, truncate_db: bool) -> Result<Self, WimmError> {
        Ok(App {
            db: Db::create(&db_path, truncate_db)?,
        })
    }

    pub fn add_task(&self, name: &str) -> Result<String, WimmError> {
        debug!("add_task(name: {name})");
        let task = new_task(name);
        self.db.insert_task(&task)?;
        Ok(task.id)
    }

    pub fn pause_task(&self, id: &str) -> Result<(), WimmError> {
        debug!("pause_task(id: {id})");
        self.db.update_task(id, |task| match task.status {
            Status::InProgress(start) => {
                debug!("Pausing task that was in progress since: {start}");
                Some(Task {
                    status: Status::OnHold,
                    time_spent: task.time_spent + since(start),
                    ..task.clone()
                })
            }
            _ => {
                debug!("Task is not in progress, skipping pause.");
                None
            }
        })
    }

    pub fn delete_task(&self, id: &str) -> Result<(), WimmError> {
        debug!("delete_task(id: {id})");
        self.db.delete_task(id)
    }

    pub fn complete_task(&self, id: &str) -> Result<(), WimmError> {
        debug!("complete_task(id: {id})");
        self.db.update_task(id, |task| match task.status {
            Status::Completed => {
                debug!("Task is already completed, skipping update.");
                None
            }
            Status::InProgress(start) => {
                debug!("Completing task that was in progress since: {start}");
                Some(Task {
                    status: Status::Completed,
                    time_spent: task.time_spent + since(start),
                    ..task.clone()
                })
            }
            _ => Some(Task {
                status: Status::Completed,
                ..task.clone()
            }),
        })
    }

    pub fn start_task(&self, id: &str) -> Result<(), WimmError> {
        debug!("start_task(id: {id})");
        self.db.update_task(id, |task| match task.status {
            Status::InProgress(since) => {
                debug!("Task is already in progress since: {since}, skipping update.");
                None
            }
            _ => Some(Task {
                status: Status::InProgress(now()),
                ..task.clone()
            }),
        })
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>, WimmError> {
        debug!("get_tasks()");
        self.db.get_tasks()
    }
}

fn new_task(name: &str) -> Task {
    Task {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        status: Status::Pending,
        created_at: now(),
        time_spent: 0,
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

fn since(time: u64) -> u64 {
    now() - time
}
