//! Configuration & config directory

use std::path::PathBuf;

use directories_next::ProjectDirs;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    /// Configuration directory for the application
    pub static ref CONFIG_DIR: PathBuf = ProjectDirs::from("com", "GnRl Leclerc", "catppuccin-flavours")
        .unwrap()
        .config_dir()
        .to_path_buf();

    /// Custom themes directory
    pub static ref THEMES_DIR: PathBuf = CONFIG_DIR.join("themes");

    /// Custom templates directory
    pub static ref TEMPLATES_DIR: PathBuf = CONFIG_DIR.join("templates");

    /// Configuration file path
    pub static ref CONFIG_FILE: PathBuf = CONFIG_DIR.join("config.toml");
}

/// The main app configuration to apply and reload themes.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub entries: Vec<Entry>,
}

/// A config entry. It defines a template, a target location, and a command to run after applying a theme.
#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    /// Template name
    pub template: String,
    /// Path to the target file to be replaced by the rendered template
    pub target: String,
    /// Optional command to run after applying the theme
    pub command: Option<String>,
}
