use std::path::PathBuf;

use crate::{
    WimmError,
    db::{self, Db},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Start(String),
    Stop(String),
    Status(String),
    List,
    Add(String),
    Delete(String),
}

pub struct App {
    db: Db<'static>,
}

impl App {
    pub fn new(db_path: PathBuf) -> Result<Self, WimmError> {
        Ok(App {
            db: Db::create(&db::Config { path: db_path })?,
        })
    }

    pub fn run(&self, action: &Action) -> Result<(), WimmError> {
        match action {
            Action::Start(id) => {
                self.db.start_task(id)?;
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
                let task = self.db.create_task(name)?;
                println!("Added task: {task}");
                Ok(())
            }
            Action::Delete(id) => {
                self.db.delete_task(id)?;
                println!("Deleted task: {id}");
                Ok(())
            }
        }
    }
}
