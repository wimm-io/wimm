//! WIMM (Where is my mind) - A terminal-based task management application
//!
//! This is the main entry point for the WIMM application. It handles:
//! - Command-line argument parsing
//! - Configuration loading and management
//! - Setting up the database storage location
//! - Initializing the persistent storage backend
//! - Loading existing tasks from storage
//! - Starting the terminal UI or handling subcommands

use std::{process, sync::OnceLock};

use directories::ProjectDirs;
use wimm::{
    cli::{Cli, Commands, ConfigAction},
    config::Config,
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
    // Parse command-line arguments
    let cli = Cli::parse_args();

    // Handle subcommands first
    if let Some(command) = &cli.command {
        if let Err(e) = handle_command(command, &cli) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        return;
    }

    // Load configuration
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!(
                "Warning: Could not load configuration: {}. Using defaults.",
                e
            );
            Config::default()
        }
    };

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
        process::exit(1);
    });

    // Load existing tasks from storage and start the UI
    // Even if loading fails, we still start the UI with an empty state
    match store.load_tasks() {
        Ok(tasks) => {
            // Successfully loaded tasks from storage
            let mut state = AppState::new(store);
            state.tasks = tasks;
            state.config = config;
            Ui::new(state)
                .run()
                .unwrap_or_else(|e| eprintln!("Error: {e}"));
        }
        Err(e) => {
            // Failed to load tasks, but continue with empty state
            // This allows users to start fresh if database is corrupted
            eprintln!("Error loading tasks from database: {e}");
            let mut state = AppState::new(store);
            state.config = config;
            Ui::new(state)
                .run()
                .unwrap_or_else(|e| eprintln!("Error: {e}"));
        }
    }
}

/// Handle CLI subcommands
fn handle_command(command: &Commands, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Config { action } => handle_config_command(action, cli),
        Commands::Run => {
            // This should not happen as we check for this case earlier
            unreachable!("Run command should be handled in main function");
        }
    }
}

/// Handle configuration subcommands
fn handle_config_command(
    action: &ConfigAction,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ConfigAction::Show => {
            let config = Config::load().unwrap_or_default();
            println!("Current configuration:");
            println!("  Color scheme: {}", config.colors.name);
            println!("  Keymap: {}", config.keymap.name);
            println!("  Default defer hour: {}", config.time.defer_hour);
            println!("  Default due hour: {}", config.time.due_hour);
            if let Some(ref tz) = config.time.timezone {
                println!("  Timezone: {}", tz);
            } else {
                println!("  Timezone: (system default)");
            }
        }
        ConfigAction::ListColors => {
            let config = Config::load().unwrap_or_default();
            println!("Available color schemes:");
            for scheme in config.list_color_schemes() {
                let marker = if scheme == config.colors.name {
                    " (current)"
                } else {
                    ""
                };
                println!("  {}{}", scheme, marker);
            }
        }
        ConfigAction::ListKeymaps => {
            let config = Config::load().unwrap_or_default();
            println!("Available keymaps:");
            for keymap in config.list_keymaps() {
                let marker = if keymap == config.keymap.name {
                    " (current)"
                } else {
                    ""
                };
                println!("  {}{}", keymap, marker);
            }
        }
        ConfigAction::Set {
            key,
            value,
            color_scheme,
            keymap,
            defer_hour,
            due_hour,
        } => {
            let mut config = Config::load().unwrap_or_default();
            let mut changes_made = false;

            // Handle key-value pairs (original format)
            if let (Some(k), Some(v)) = (key, value) {
                match k.as_str() {
                    "color-scheme" | "colors" => {
                        config
                            .set_color_scheme(v)
                            .map_err(|e| format!("Failed to set color scheme: {}", e))?;
                        println!("Configuration updated: {} = {}", k, v);
                        changes_made = true;
                    }
                    "keymap" => {
                        config
                            .set_keymap(v)
                            .map_err(|e| format!("Failed to set keymap: {}", e))?;
                        println!("Configuration updated: {} = {}", k, v);
                        changes_made = true;
                    }
                    "defer-hour" => {
                        let hour: u32 = v
                            .parse()
                            .map_err(|_| "Defer hour must be a number between 0 and 23")?;
                        if hour > 23 {
                            return Err("Defer hour must be between 0 and 23".into());
                        }
                        config.time.defer_hour = hour;
                        println!("Configuration updated: {} = {}", k, v);
                        changes_made = true;
                    }
                    "due-hour" => {
                        let hour: u32 = v
                            .parse()
                            .map_err(|_| "Due hour must be a number between 0 and 23")?;
                        if hour > 23 {
                            return Err("Due hour must be between 0 and 23".into());
                        }
                        config.time.due_hour = hour;
                        println!("Configuration updated: {} = {}", k, v);
                        changes_made = true;
                    }
                    "timezone" => {
                        config.time.timezone = if v.is_empty() || v == "system" {
                            None
                        } else {
                            Some(v.clone())
                        };
                        println!("Configuration updated: {} = {}", k, v);
                        changes_made = true;
                    }
                    _ => {
                        return Err(format!("Unknown configuration key: {}. Available keys: color-scheme, keymap, defer-hour, due-hour, timezone", k).into());
                    }
                }
            }

            // Handle flag-style options (new format)
            if let Some(scheme) = color_scheme {
                config
                    .set_color_scheme(scheme)
                    .map_err(|e| format!("Failed to set color scheme: {}", e))?;
                println!("Configuration updated: color-scheme = {}", scheme);
                changes_made = true;
            }

            if let Some(km) = keymap {
                config
                    .set_keymap(km)
                    .map_err(|e| format!("Failed to set keymap: {}", e))?;
                println!("Configuration updated: keymap = {}", km);
                changes_made = true;
            }

            if let Some(hour) = defer_hour {
                if *hour > 23 {
                    return Err("Defer hour must be between 0 and 23".into());
                }
                config.time.defer_hour = *hour;
                println!("Configuration updated: defer-hour = {}", hour);
                changes_made = true;
            }

            if let Some(hour) = due_hour {
                if *hour > 23 {
                    return Err("Due hour must be between 0 and 23".into());
                }
                config.time.due_hour = *hour;
                println!("Configuration updated: due-hour = {}", hour);
                changes_made = true;
            }

            if !changes_made {
                return Err("No configuration changes specified. Use either 'key value' format or flags like --color-scheme".into());
            }

            config.save()?;
            if cli.verbose {
                println!("Configuration saved to: {:?}", Config::config_path()?);
            }
        }
        ConfigAction::Reset => {
            let config = Config::default();
            config.save()?;
            println!("Configuration reset to defaults");
            if cli.verbose {
                println!("Configuration saved to: {:?}", Config::config_path()?);
            }
        }
        ConfigAction::Path => {
            println!("{}", Config::config_path()?.display());
        }
        ConfigAction::Edit => {
            let config_path = Config::config_path()?;

            // Ensure config file exists
            if !config_path.exists() {
                let config = Config::default();
                config.save()?;
                println!("Created default configuration file");
            }

            // Try to find an editor
            let editor = std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| {
                    if cfg!(target_os = "windows") {
                        "notepad".to_string()
                    } else {
                        "nano".to_string()
                    }
                });

            let status = std::process::Command::new(&editor)
                .arg(&config_path)
                .status()?;

            if !status.success() {
                return Err(format!("Editor '{}' exited with non-zero status", editor).into());
            }

            if cli.verbose {
                println!("Configuration file edited: {}", config_path.display());
            }
        }
    }
    Ok(())
}
