//! Color themes and how to fetch them

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{BUILTIN_THEMES, config::THEMES_DIR};

/// Catppuccin color theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub rosewater: String,
    pub flamingo: String,
    pub pink: String,
    pub mauve: String,
    pub red: String,
    pub maroon: String,
    pub peach: String,
    pub yellow: String,
    pub green: String,
    pub teal: String,
    pub sky: String,
    pub sapphire: String,
    pub blue: String,
    pub lavender: String,
    pub text: String,
    pub subtext1: String,
    pub subtext0: String,
    pub overlay2: String,
    pub overlay1: String,
    pub overlay0: String,
    pub surface2: String,
    pub surface1: String,
    pub surface0: String,
    pub base: String,
    pub mantle: String,
    pub crust: String,
}

/// Try to read a color theme from the configuration directory
fn read_theme_from_config(name: &str) -> Option<Theme> {
    let path = THEMES_DIR.join(format!("{name}.toml"));
    if path.exists() {
        let theme = std::fs::read_to_string(path).unwrap();
        Some(toml::from_str(&theme).unwrap())
    } else {
        None
    }
}

/// Get a theme by its name.
/// Custom themes from the config directory take precedence over builtin themes.
pub fn get_theme(name: &str) -> Option<Theme> {
    let theme = read_theme_from_config(name);
    if theme.is_some() {
        return theme;
    }

    if let Some(theme) = BUILTIN_THEMES.get(name) {
        return Some(toml::from_str(theme).unwrap());
    }

    None
}

/// List all available themes.
pub fn list_all() -> Vec<String> {
    let mut set: HashSet<String> = HashSet::new();

    // Add builtin themes
    for theme in BUILTIN_THEMES.keys() {
        set.insert(theme.to_string());
    }

    // Add themes from the config directory
    if THEMES_DIR.exists() {
        for entry in std::fs::read_dir(&*THEMES_DIR).unwrap() {
            let entry = entry.unwrap();
            if entry.path().extension().and_then(|s| s.to_str()) == Some("toml") {
                if let Some(name) = entry.path().file_stem().and_then(|s| s.to_str()) {
                    set.insert(name.to_string());
                }
            }
        }
    }

    set.into_iter().collect()
}
