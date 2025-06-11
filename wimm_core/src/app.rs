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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Start(String),
    Stop(String),
    Status(String),
    List,
    Add(String),
    Delete(String),
    Complete(String),
    Pause(String),
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub struct App {
    db: Db<'static>,
}

impl App {
    pub fn new(db_path: PathBuf, truncate_db: bool) -> Result<Self, WimmError> {
        Ok(App {
            db: Db::create(&db_path, truncate_db)?,
        })
    }

    pub fn run(&self, action: &Action) -> Result<(), WimmError> {
        match action {
            Action::Start(id) => {
                self.start_task(id)?;
                println!("Started task ID: {id}");
                Ok(())
            }
            Action::Stop(name) => {
                println!("Stop: {name} not implemented yet");
                Ok(())
            }
            Action::Status(name) => {
                println!("Status: {name} not implemented yet");
                Ok(())
            }
            Action::List => {
                let tasks = self.db.get_tasks()?;
                if tasks.is_empty() {
                    println!("No tasks found.");
                } else {
                    for task in tasks {
                        println!("{task}");
                    }
                }
                Ok(())
            }
            Action::Add(name) => {
                let task = new_task(name);
                self.db.insert_task(&task)?;
                println!("Added task: {task}");
                Ok(())
            }
            Action::Delete(id) => {
                self.db.delete_task(id)?;
                println!("Deleted task: {id}");
                Ok(())
            }
            Action::Complete(id) => {
                self.complete_task(id)?;
                println!("Completed task: {id}");
                Ok(())
            }
            Action::Pause(id) => {
                self.pause_task(id)?;
                println!("Pause task: {id}");
                Ok(())
            }
        }
    }

    fn pause_task(&self, id: &str) -> Result<(), WimmError> {
        debug!("pause_task(id: {id})");
        let pause_time = now();
        self.db.update_task(id, |task| match task.status {
            Status::InProgress(since) => {
                debug!("Pausing task that was in progress since: {since}");
                Some(Task {
                    status: Status::OnHold(pause_time),
                    time_spent: task.time_spent + (pause_time - since),
                    ..task.clone()
                })
            }
            _ => {
                debug!("Task is not in progress, skipping pause.");
                None
            }
        })
    }

    fn start_task(&self, id: &str) -> Result<(), WimmError> {
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

    fn complete_task(&self, id: &str) -> Result<(), WimmError> {
        debug!("complete_task(id: {id})");
        let completed_at = now();
        self.db.update_task(id, |task| match task.status {
            Status::Completed(_) => {
                debug!("Task is already completed, skipping update.");
                None
            }
            Status::InProgress(since) => {
                debug!("Completing task that was in progress since: {since}");
                Some(Task {
                    status: Status::Completed(completed_at),
                    time_spent: task.time_spent + (completed_at - since),
                    ..task.clone()
                })
            }
            _ => Some(Task {
                status: Status::Completed(completed_at),
                ..task.clone()
            }),
        })
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
