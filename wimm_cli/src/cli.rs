use log::debug;
use std::{ffi::OsString, path::PathBuf, sync::OnceLock};

use anyhow::{Result, anyhow};
use clap::{Command, arg, command};
use directories::ProjectDirs;
use wimm_core::{WimmError, app::App};

#[derive(Debug)]
struct Args {
    pub action: Action,
    pub db_path: PathBuf,
    pub force_init: bool,
}

static PROJECT_DIRS: OnceLock<Option<ProjectDirs>> = OnceLock::new();

fn project_dirs() -> &'static Option<ProjectDirs> {
    PROJECT_DIRS.get_or_init(|| ProjectDirs::from("io", "wimm", "wimm"))
}

fn default_db_path() -> Option<PathBuf> {
    project_dirs()
        .as_ref()
        .map(|pd| pd.data_dir().join("wimm.db"))
}

fn get_args<I, T>(args: I) -> Result<Args>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = command!()
        .arg(arg!(--db <DB_PATH> "Path to the database file"))
        .arg(arg!(--force "Force initialization, overwriting existing database"))
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .alias("a")
                .about("add a new task")
                .arg(arg!(<TASK> "name of the task")),
        )
        .subcommand(
            Command::new("start")
                .alias("s")
                .about("start a new task")
                .arg(arg!(<ID> "ID of the task")),
        )
        .subcommand(
            Command::new("remove")
                .alias("rm")
                .about("remove a task")
                .arg(arg!(<ID> "ID of the task")),
        )
        .subcommand(Command::new("list").alias("ls").about("list all tasks"))
        .subcommand(
            Command::new("complete")
                .alias("c")
                .about("complete a task")
                .arg(arg!(<ID> "ID of the task")),
        )
        .subcommand(
            Command::new("pause")
                .alias("p")
                .about("pause a task")
                .arg(arg!(<ID> "ID of the task")),
        )
        .get_matches_from(args);

    let force_init = matches.get_flag("force");
    let db_path = matches
        .get_one::<PathBuf>("db")
        .cloned()
        .or(default_db_path())
        .ok_or(WimmError::DbError(String::from("No DB path specified")))?;
    debug!("Using database path: {}", db_path.display());

    let action = match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let task_name = sub_matches
                .get_one::<String>("TASK")
                .expect("TASK argument is required");
            Action::Add(task_name.clone())
        }
        Some(("start", sub_matches)) => {
            let task_id = sub_matches
                .get_one::<String>("ID")
                .expect("ID argument is required");
            Action::Start(task_id.clone())
        }
        Some(("remove", sub_matches)) => {
            let task_id = sub_matches
                .get_one::<String>("ID")
                .expect("ID argument is required");
            Action::Delete(task_id.clone())
        }
        Some(("list", _)) => Action::List,
        Some(("complete", sub_matches)) => {
            let task_id = sub_matches
                .get_one::<String>("ID")
                .expect("ID argument is required");
            Action::Complete(task_id.clone())
        }
        Some(("pause", sub_matches)) => {
            let task_id = sub_matches
                .get_one::<String>("ID")
                .expect("ID argument is required");
            Action::Pause(task_id.clone())
        }
        _ => return Err(anyhow!("Subcommand required")),
    };

    Ok(Args {
        action,
        db_path,
        force_init,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Start(String),
    List,
    Add(String),
    Delete(String),
    Complete(String),
    Pause(String),
}

pub fn run<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = get_args(args)?;
    debug!("Parsed arguments: {:?}", args);

    let app = App::new(args.db_path, args.force_init)?;
    match args.action {
        Action::Start(id) => {
            app.start_task(&id)?;
            println!("Started task ID: {id}");
            Ok(())
        }
        Action::List => {
            let tasks = app.get_tasks()?;
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
            let id = app.add_task(&name)?;
            println!("Added task: {id}");
            Ok(())
        }
        Action::Delete(id) => {
            app.delete_task(&id)?;
            println!("Deleted task: {id}");
            Ok(())
        }
        Action::Complete(id) => {
            app.complete_task(&id)?;
            println!("Completed task: {id}");
            Ok(())
        }
        Action::Pause(id) => {
            app.pause_task(&id)?;
            println!("Pause task: {id}");
            Ok(())
        }
    }
}
