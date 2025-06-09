use log::debug;
use std::{ffi::OsString, path::PathBuf, sync::OnceLock};

use anyhow::Result;
use clap::{Command, arg, command};
use directories::ProjectDirs;
use wimm_core::{Action, WimmError};

pub struct Args {
    pub action: Action,
    pub db_path: PathBuf,
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

pub fn get_args<I, T>(args: I) -> Result<Args>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = command!()
        .arg(arg!(--db <DB_PATH> "Path to the database file"))
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
        .get_matches_from(args);

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
        _ => {
            panic!("No valid subcommand provided.");
        }
    };

    let db_path = matches
        .get_one::<PathBuf>("db")
        .cloned()
        .or(default_db_path())
        .ok_or(WimmError::DbError(String::from("No DB path specified")))?;

    debug!("Using database path: {}", db_path.display());

    Ok(Args { action, db_path })
}
