//! Configuration & config directory

use std::path::PathBuf;

use directories_next::ProjectDirs;
use lazy_static::lazy_static;

lazy_static! {
    /// Configuration directory for the application
    pub static ref CONFIG_DIR: PathBuf = ProjectDirs::from("com", "GnRl Leclerc", "cfl")
        .unwrap()
        .config_dir()
        .to_path_buf();

    /// Custom themes directory
    pub static ref THEMES_DIR: PathBuf = CONFIG_DIR.join("themes");
}
