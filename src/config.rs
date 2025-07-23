//! Configuration & config directory

use std::path::PathBuf;

use directories_next::ProjectDirs;
use lazy_static::lazy_static;

use crate::colors::Colors;

lazy_static! {
    /// Configuration directory for the application
    static ref CONFIG_DIR: PathBuf = ProjectDirs::from("com", "GnRl Leclerc", "cfl")
        .unwrap()
        .config_dir()
        .to_path_buf();

    /// Custom themes directory
    static ref THEMES_DIR: PathBuf = CONFIG_DIR.join("themes");
}

/// Try to read a color theme from the configuration directory
pub fn read_theme_from_config(theme: &str) -> Option<Colors> {
    let theme_path = THEMES_DIR.join(format!("{theme}.toml"));
    if theme_path.exists() {
        let theme_content = std::fs::read_to_string(theme_path).unwrap();
        Some(toml::from_str(&theme_content).unwrap())
    } else {
        None
    }
}
