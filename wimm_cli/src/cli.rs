use log::debug;
use std::{ffi::OsString, path::PathBuf, sync::OnceLock};

use anyhow::Result;
use clap::{Command, arg, command};
use directories::ProjectDirs;
use wimm_core::{Action, WimmError};

pub struct Args {
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

pub fn get_args<I, T>(args: I) -> Result<Args>
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
            Command::new("init")
                .about("initialize the database")
                .arg(arg!(-f --force "Force initialization, overwriting existing database")),
        )
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
        _ => {
            panic!("No valid subcommand provided.");
        }
    };

    let force_init = matches.get_flag("force");

    let db_path = matches
        .get_one::<PathBuf>("db")
        .cloned()
        .or(default_db_path())
        .ok_or(WimmError::DbError(String::from("No DB path specified")))?;

    debug!("Using database path: {}", db_path.display());

    Ok(Args {
        action,
        db_path,
        force_init,
    })
}
