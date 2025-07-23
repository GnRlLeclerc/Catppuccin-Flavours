use crate::{BUILTIN_THEMES, colors::Colors, config::THEMES_DIR};

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

/// Get a theme by its name.
/// Custom themes from the config directory take precedence over builtin themes.
pub fn get_theme(theme_name: &str) -> Option<Colors> {
    let theme = read_theme_from_config(theme_name);
    if theme.is_some() {
        return theme;
    }

    if let Some(theme) = BUILTIN_THEMES.get(theme_name) {
        return Some(toml::from_str(theme).unwrap());
    }

    None
}
