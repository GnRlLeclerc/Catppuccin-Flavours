//! Template utilities

use serde::Serialize;
use tera::{Context, Tera};

use crate::{cli::AccentColor, colors::Palette};

/// Render a Tera template with the given theme.
pub fn render_template(
    theme_name: &str,
    palette: &Palette,
    accent: AccentColor,
    template: &str,
) -> Result<String, tera::Error> {
    let accent = palette.accent(accent);
    let flavor = Flavor::new(theme_name);

    let mut tera = Tera::default();
    let mut context = Context::new();

    context.insert("accent", accent);
    context.insert("palette", &palette);
    context.insert("flavor", &flavor);

    tera.render_str(template, &context)
}

/// Theme metadata for Tera templates
#[derive(Debug, Clone, Serialize)]
struct Flavor {
    identifier: String,
    name: String,
    dark: bool,
}

impl Flavor {
    pub fn new(theme_name: &str) -> Self {
        let name = capitalize_words(
            &theme_name
                .replace('_', " ")
                .replace('-', " ")
                .to_lowercase(),
        );
        Self {
            identifier: theme_name.to_string(),
            name,
            dark: true, // TODO: add this to theme configurations!
        }
    }
}

fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
