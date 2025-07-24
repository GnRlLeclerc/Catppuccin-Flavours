//! Templates and how to fetch them.

use crate::{BUILTIN_TEMPLATES, config::TEMPLATES_DIR};

/// Try to read a template from the configuration directory
fn read_template_from_config(name: &str) -> Option<String> {
    let path = TEMPLATES_DIR.join(format!("{name}.tera"));
    if path.exists() {
        Some(std::fs::read_to_string(path).unwrap())
    } else {
        None
    }
}

/// Get a template by its name.
/// Custom templates from the config directory take precedence over builtin templates.
pub fn get_template(name: &str) -> Option<String> {
    let template = read_template_from_config(name);
    if template.is_some() {
        return template;
    }

    BUILTIN_TEMPLATES.get(name).map(|s| s.to_string())
}

/// List all available templates.
pub fn list_all() -> Vec<String> {
    let mut set: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Add builtin templates
    for template in BUILTIN_TEMPLATES.keys() {
        set.insert(template.to_string());
    }

    // Add templates from the config directory
    if TEMPLATES_DIR.exists() {
        for entry in std::fs::read_dir(&*TEMPLATES_DIR).unwrap() {
            let entry = entry.unwrap();
            if entry.path().extension().and_then(|s| s.to_str()) == Some("tera") {
                if let Some(name) = entry.path().file_stem().and_then(|s| s.to_str()) {
                    set.insert(name.to_string());
                }
            }
        }
    }

    let mut list: Vec<_> = set.into_iter().collect();
    list.sort();
    list
}
