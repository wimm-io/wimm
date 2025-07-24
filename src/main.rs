//! WIMM (Where is my mind) - A terminal-based task management application
//!
//! This is the main entry point for the WIMM application. It handles:
//! - Setting up the database storage location
//! - Initializing the persistent storage backend
//! - Loading existing tasks from storage
//! - Starting the terminal UI

use std::sync::OnceLock;

use directories::ProjectDirs;
use wimm::{
    storage::{Db, SledStorage},
    types::AppState,
    ui::Ui,
};

/// Global storage for project directories, computed once and cached
/// Uses the standard platform-specific application data directory
static PROJECT_PATH: OnceLock<Option<ProjectDirs>> = OnceLock::new();

/// Get the platform-specific project directory for storing application data
///
/// This function returns the appropriate directory based on the operating system:
/// - Linux: ~/.local/share/wimm/
/// - macOS: ~/Library/Application Support/wimm/
/// - Windows: %APPDATA%/wimm/
fn project_path() -> Option<&'static ProjectDirs> {
    PROJECT_PATH
        .get_or_init(|| directories::ProjectDirs::from("io", "wimm", "wimm"))
        .as_ref()
}

fn main() {
    // Determine where to store the database file
    // Falls back to current directory if platform directories aren't available
    let db_path = project_path().map(|pp| pp.data_dir()).unwrap_or_else(|| {
        eprintln!("Warning: Could not determine project directory. Using current directory.");
        std::path::Path::new(".")
    });

    // Initialize the persistent storage backend (Sled embedded database)
    // Exit with error if database cannot be opened
    let store = SledStorage::new(db_path.join("tasks.db")).unwrap_or_else(|e| {
        eprintln!("Error initializing database at {db_path:?}: {e}");
        std::process::exit(1);
    });

    // Load existing tasks from storage and start the UI
    // Even if loading fails, we still start the UI with an empty state
    match store.load_tasks() {
        Ok(tasks) => {
            // Successfully loaded tasks from storage
            let mut state = AppState::new(store);
            state.tasks = tasks;
            Ui::new(state)
                .run()
                .unwrap_or_else(|e| eprintln!("Error: {e}"));
        }
        Err(e) => {
            // Failed to load tasks, but continue with empty state
            // This allows users to start fresh if database is corrupted
            eprintln!("Error loading tasks from database: {e}");
            Ui::new(AppState::new(store))
                .run()
                .unwrap_or_else(|e| eprintln!("Error: {e}"));
        }
    }
}
