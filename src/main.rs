use std::sync::OnceLock;

use directories::ProjectDirs;
use wimm::{storage::SledStorage, types::AppState, ui::Ui};

static PROJECT_PATH: OnceLock<Option<ProjectDirs>> = OnceLock::new();

fn project_path() -> Option<&'static ProjectDirs> {
    PROJECT_PATH
        .get_or_init(|| directories::ProjectDirs::from("io", "wimm", "wimm"))
        .as_ref()
}

fn main() {
    let db_path = project_path().map(|pp| pp.data_dir()).unwrap_or_else(|| {
        eprintln!("Warning: Could not determine project directory. Using current directory.");
        std::path::Path::new(".")
    });

    let store = SledStorage::new(db_path.join("tasks.db")).unwrap_or_else(|e| {
        eprintln!("Error initializing database at {:?}: {}", db_path, e);
        std::process::exit(1);
    });

    Ui::new(AppState::new(store))
        .run()
        .unwrap_or_else(|e| eprintln!("Error: {}", e));
}
