//! Command-line interface for WIMM
//!
//! This module defines the CLI using clap for parsing command-line arguments
//! and subcommands for configuration management.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// WIMM (Where is my mind) - A terminal-based task management application
#[derive(Parser, Debug)]
#[command(
    name = "wimm",
    version,
    about = "A terminal-based task management application",
    long_about = "WIMM (Where is my mind) is a terminal-based task management application \
                  inspired by Getting Things Done (GTD) methodology. It provides a simple \
                  and efficient way to manage tasks with support for due dates, defer dates, \
                  and customizable themes."
)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Subcommand to run
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Start the interactive TUI (default)
    Run,
}

/// Configuration subcommands
#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// List available color schemes
    ListColors,
    /// List available keymaps
    ListKeymaps,
    /// Set configuration values
    Set {
        /// Configuration key to set (when using key-value format)
        key: Option<String>,
        /// Value to set (when using key-value format)
        value: Option<String>,
        /// Set the color scheme
        #[arg(long, value_name = "NAME")]
        color_scheme: Option<String>,
        /// Set the keymap
        #[arg(long, value_name = "NAME")]
        keymap: Option<String>,
        /// Set the default defer hour (0-23)
        #[arg(long, value_name = "HOUR")]
        defer_hour: Option<u32>,
        /// Set the default due hour (0-23)
        #[arg(long, value_name = "HOUR")]
        due_hour: Option<u32>,
    },
    /// Reset configuration to defaults
    Reset,
    /// Show the path to the configuration file
    Path,
    /// Edit the configuration file in the default editor
    Edit,
}

impl Cli {
    /// Parse command line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Check if we should run the TUI or handle a subcommand
    pub fn should_run_tui(&self) -> bool {
        match &self.command {
            None | Some(Commands::Run) => true,
            Some(Commands::Config { .. }) => false,
        }
    }

    /// Check if any configuration changes were requested
    pub fn has_config_changes(&self) -> bool {
        false // No longer have config overrides at top level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_verify() {
        // Verify that the CLI is well-formed
        Cli::command().debug_assert();
    }

    #[test]
    fn test_has_config_changes() {
        let cli = Cli {
            config: None,
            verbose: false,
            command: None,
        };
        assert!(!cli.has_config_changes());
    }

    #[test]
    fn test_should_run_tui() {
        let cli = Cli {
            config: None,
            verbose: false,
            command: None,
        };
        assert!(cli.should_run_tui());

        let cli = Cli {
            config: None,
            verbose: false,
            command: Some(Commands::Run),
        };
        assert!(cli.should_run_tui());

        let cli = Cli {
            config: None,
            verbose: false,
            command: Some(Commands::Config {
                action: ConfigAction::Show,
            }),
        };
        assert!(!cli.should_run_tui());
    }

    #[test]
    fn test_config_set_with_flags() {
        // Test that we can parse config set with flags
        let args = vec![
            "wimm",
            "config",
            "set",
            "--color-scheme",
            "dark",
            "--defer-hour",
            "8",
        ];

        let cli = Cli::try_parse_from(args).unwrap();
        if let Some(Commands::Config {
            action:
                ConfigAction::Set {
                    color_scheme,
                    defer_hour,
                    ..
                },
        }) = cli.command
        {
            assert_eq!(color_scheme, Some("dark".to_string()));
            assert_eq!(defer_hour, Some(8));
        } else {
            panic!("Expected config set command");
        }
    }

    #[test]
    fn test_config_set_with_key_value() {
        // Test that we can parse config set with key-value format
        let args = vec!["wimm", "config", "set", "color-scheme", "light"];

        let cli = Cli::try_parse_from(args).unwrap();
        if let Some(Commands::Config {
            action: ConfigAction::Set { key, value, .. },
        }) = cli.command
        {
            assert_eq!(key, Some("color-scheme".to_string()));
            assert_eq!(value, Some("light".to_string()));
        } else {
            panic!("Expected config set command");
        }
    }
}
