//! Configuration management for WIMM
//!
//! This module handles loading and saving application configuration including
//! color schemes, keymaps, and default settings for task management.

use chrono::{DateTime, Local, NaiveTime, TimeZone};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use thiserror::Error;

/// Configuration-related errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("Could not determine config directory")]
    NoConfigDir,
    #[error("Invalid time format: {0}")]
    InvalidTime(String),
}

/// Color scheme configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorScheme {
    /// Name of the color scheme
    pub name: String,
    /// Primary foreground color (text)
    pub fg: String,
    /// Primary background color
    pub bg: String,
    /// Accent color for highlights and selections
    pub accent: String,
    /// Color for completed tasks
    pub completed: String,
    /// Color for overdue tasks
    pub overdue: String,
    /// Color for deferred tasks
    pub deferred: String,
    /// Color for borders and UI elements
    pub border: String,
    /// Color for help text and secondary information
    pub help: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            fg: "#ffffff".to_string(),
            bg: "#000000".to_string(),
            accent: "#00ff00".to_string(),
            completed: "#888888".to_string(),
            overdue: "#ff0000".to_string(),
            deferred: "#ffff00".to_string(),
            border: "#444444".to_string(),
            help: "#cccccc".to_string(),
        }
    }
}

/// Keymap configuration for different modes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Keymap {
    /// Name of the keymap
    pub name: String,
    /// Normal mode key bindings
    pub normal: HashMap<String, String>,
    /// Insert mode key bindings
    pub insert: HashMap<String, String>,
}

impl Default for Keymap {
    fn default() -> Self {
        let mut normal = HashMap::new();
        normal.insert("q".to_string(), "quit".to_string());
        normal.insert("?".to_string(), "help".to_string());
        normal.insert("n".to_string(), "new_task".to_string());
        normal.insert("e".to_string(), "edit_task".to_string());
        normal.insert("d".to_string(), "delete_task".to_string());
        normal.insert("c".to_string(), "complete_task".to_string());
        normal.insert("j".to_string(), "move_down".to_string());
        normal.insert("k".to_string(), "move_up".to_string());
        normal.insert("Enter".to_string(), "edit_task".to_string());
        normal.insert("Esc".to_string(), "escape".to_string());

        let mut insert = HashMap::new();
        insert.insert("Esc".to_string(), "escape".to_string());
        insert.insert("Enter".to_string(), "confirm".to_string());
        insert.insert("Tab".to_string(), "next_field".to_string());
        insert.insert("Shift+Tab".to_string(), "prev_field".to_string());

        Self {
            name: "default".to_string(),
            normal,
            insert,
        }
    }
}

/// Time-related default settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeDefaults {
    /// Default hour (0-23) to use for defer dates when only date is specified
    pub defer_hour: u32,
    /// Default hour (0-23) to use for due dates when only date is specified
    pub due_hour: u32,
    /// Default timezone (use system timezone if None)
    pub timezone: Option<String>,
}

impl Default for TimeDefaults {
    fn default() -> Self {
        Self {
            defer_hour: 9,  // 9 AM
            due_hour: 17,   // 5 PM
            timezone: None, // Use system timezone
        }
    }
}

impl TimeDefaults {
    /// Get a DateTime for today at the defer hour
    pub fn defer_today(&self) -> Result<DateTime<Local>, ConfigError> {
        self.time_today(self.defer_hour)
    }

    /// Get a DateTime for today at the due hour
    pub fn due_today(&self) -> Result<DateTime<Local>, ConfigError> {
        self.time_today(self.due_hour)
    }

    /// Get a DateTime for today at the specified hour
    pub fn time_today(&self, hour: u32) -> Result<DateTime<Local>, ConfigError> {
        if hour > 23 {
            return Err(ConfigError::InvalidTime(format!(
                "Hour {hour} is invalid (must be 0-23)"
            )));
        }

        let time = NaiveTime::from_hms_opt(hour, 0, 0)
            .ok_or_else(|| ConfigError::InvalidTime(format!("Invalid time: {hour}:00:00")))?;

        let today = Local::now().date_naive();
        let datetime = today.and_time(time);

        Local
            .from_local_datetime(&datetime)
            .single()
            .ok_or_else(|| ConfigError::InvalidTime("Could not create local datetime".to_string()))
    }
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Color scheme settings
    pub colors: ColorScheme,
    /// Keymap settings
    pub keymap: Keymap,
    /// Time-related defaults
    pub time: TimeDefaults,
    /// Available color schemes
    pub color_schemes: Vec<ColorScheme>,
    /// Available keymaps
    pub keymaps: Vec<Keymap>,
}

impl Default for Config {
    fn default() -> Self {
        let default_colors = ColorScheme::default();
        let default_keymap = Keymap::default();

        // Create some built-in color schemes
        let mut color_schemes = vec![default_colors.clone()];

        // Dark theme
        color_schemes.push(ColorScheme {
            name: "dark".to_string(),
            fg: "#e0e0e0".to_string(),
            bg: "#1a1a1a".to_string(),
            accent: "#4a90e2".to_string(),
            completed: "#666666".to_string(),
            overdue: "#d32f2f".to_string(),
            deferred: "#ffa726".to_string(),
            border: "#333333".to_string(),
            help: "#b0b0b0".to_string(),
        });

        // Light theme
        color_schemes.push(ColorScheme {
            name: "light".to_string(),
            fg: "#333333".to_string(),
            bg: "#ffffff".to_string(),
            accent: "#1976d2".to_string(),
            completed: "#999999".to_string(),
            overdue: "#c62828".to_string(),
            deferred: "#ef6c00".to_string(),
            border: "#cccccc".to_string(),
            help: "#666666".to_string(),
        });

        // Create some built-in keymaps
        let mut keymaps = vec![default_keymap.clone()];

        // Vi-style keymap
        let mut vi_normal = HashMap::new();
        vi_normal.insert("q".to_string(), "quit".to_string());
        vi_normal.insert("?".to_string(), "help".to_string());
        vi_normal.insert("i".to_string(), "new_task".to_string());
        vi_normal.insert("e".to_string(), "edit_task".to_string());
        vi_normal.insert("dd".to_string(), "delete_task".to_string());
        vi_normal.insert("x".to_string(), "complete_task".to_string());
        vi_normal.insert("j".to_string(), "move_down".to_string());
        vi_normal.insert("k".to_string(), "move_up".to_string());
        vi_normal.insert("Enter".to_string(), "edit_task".to_string());
        vi_normal.insert("Esc".to_string(), "escape".to_string());
        vi_normal.insert("gg".to_string(), "move_top".to_string());
        vi_normal.insert("G".to_string(), "move_bottom".to_string());

        let mut vi_insert = HashMap::new();
        vi_insert.insert("Esc".to_string(), "escape".to_string());
        vi_insert.insert("Ctrl+[".to_string(), "escape".to_string());
        vi_insert.insert("Enter".to_string(), "confirm".to_string());
        vi_insert.insert("Tab".to_string(), "next_field".to_string());
        vi_insert.insert("Shift+Tab".to_string(), "prev_field".to_string());

        keymaps.push(Keymap {
            name: "vi".to_string(),
            normal: vi_normal,
            insert: vi_insert,
        });

        Self {
            colors: default_colors,
            keymap: default_keymap,
            time: TimeDefaults::default(),
            color_schemes,
            keymaps,
        }
    }
}

impl Config {
    /// Load configuration from the standard config file location
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config and save it
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to the standard config file location
    pub fn save(&self) -> Result<(), ConfigError> {
        let config_path = Self::config_path()?;

        // Ensure the config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    /// Get the standard configuration file path
    pub fn config_path() -> Result<PathBuf, ConfigError> {
        let project_dirs =
            ProjectDirs::from("io", "wimm", "wimm").ok_or(ConfigError::NoConfigDir)?;
        Ok(project_dirs.config_dir().join("config.toml"))
    }

    /// Get a color scheme by name
    pub fn get_color_scheme(&self, name: &str) -> Option<&ColorScheme> {
        self.color_schemes.iter().find(|cs| cs.name == name)
    }

    /// Get a keymap by name
    pub fn get_keymap(&self, name: &str) -> Option<&Keymap> {
        self.keymaps.iter().find(|km| km.name == name)
    }

    /// Set the active color scheme by name
    pub fn set_color_scheme(&mut self, name: &str) -> Result<(), ConfigError> {
        if let Some(color_scheme) = self.get_color_scheme(name).cloned() {
            self.colors = color_scheme;
            Ok(())
        } else {
            Err(ConfigError::InvalidTime(format!(
                "Color scheme '{name}' not found"
            )))
        }
    }

    /// Set the active keymap by name
    pub fn set_keymap(&mut self, name: &str) -> Result<(), ConfigError> {
        if let Some(keymap) = self.get_keymap(name).cloned() {
            self.keymap = keymap;
            Ok(())
        } else {
            Err(ConfigError::InvalidTime(format!(
                "Keymap '{name}' not found"
            )))
        }
    }

    /// List available color scheme names
    pub fn list_color_schemes(&self) -> Vec<&str> {
        self.color_schemes
            .iter()
            .map(|cs| cs.name.as_str())
            .collect()
    }

    /// List available keymap names
    pub fn list_keymaps(&self) -> Vec<&str> {
        self.keymaps.iter().map(|km| km.name.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_default() {
        let colors = ColorScheme::default();
        assert_eq!(colors.name, "default");
        assert_eq!(colors.fg, "#ffffff");
        assert_eq!(colors.bg, "#000000");
    }

    #[test]
    fn test_keymap_default() {
        let keymap = Keymap::default();
        assert_eq!(keymap.name, "default");
        assert!(keymap.normal.contains_key("q"));
        assert!(keymap.insert.contains_key("Esc"));
    }

    #[test]
    fn test_time_defaults() {
        let time = TimeDefaults::default();
        assert_eq!(time.defer_hour, 9);
        assert_eq!(time.due_hour, 17);
        assert!(time.timezone.is_none());
    }

    #[test]
    fn test_time_defaults_validation() {
        let time = TimeDefaults::default();
        assert!(time.time_today(9).is_ok());
        assert!(time.time_today(23).is_ok());
        assert!(time.time_today(24).is_err());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.colors.name, "default");
        assert_eq!(config.keymap.name, "default");
        assert!(config.color_schemes.len() >= 3); // default, dark, light
        assert!(config.keymaps.len() >= 2); // default, vi
    }

    #[test]
    fn test_config_get_color_scheme() {
        let config = Config::default();
        assert!(config.get_color_scheme("default").is_some());
        assert!(config.get_color_scheme("dark").is_some());
        assert!(config.get_color_scheme("nonexistent").is_none());
    }

    #[test]
    fn test_config_get_keymap() {
        let config = Config::default();
        assert!(config.get_keymap("default").is_some());
        assert!(config.get_keymap("vi").is_some());
        assert!(config.get_keymap("nonexistent").is_none());
    }

    #[test]
    fn test_config_set_color_scheme() {
        let mut config = Config::default();
        assert!(config.set_color_scheme("dark").is_ok());
        assert_eq!(config.colors.name, "dark");
        assert!(config.set_color_scheme("nonexistent").is_err());
    }

    #[test]
    fn test_config_set_keymap() {
        let mut config = Config::default();
        assert!(config.set_keymap("vi").is_ok());
        assert_eq!(config.keymap.name, "vi");
        assert!(config.set_keymap("nonexistent").is_err());
    }

    #[test]
    fn test_config_list_schemes_and_keymaps() {
        let config = Config::default();
        let color_schemes = config.list_color_schemes();
        let keymaps = config.list_keymaps();

        assert!(color_schemes.contains(&"default"));
        assert!(color_schemes.contains(&"dark"));
        assert!(color_schemes.contains(&"light"));

        assert!(keymaps.contains(&"default"));
        assert!(keymaps.contains(&"vi"));
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string(&config).unwrap();
        assert!(serialized.contains("[colors]"));
        assert!(serialized.contains("[keymap]"));
        assert!(serialized.contains("[time]"));

        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.colors.name, config.colors.name);
        assert_eq!(deserialized.keymap.name, config.keymap.name);
    }
}
