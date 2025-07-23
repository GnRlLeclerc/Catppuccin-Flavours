//! Template utilities

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use crate::colors::{Palette, Theme};

// BUG : functions ? css_hsl ?

/// Render a Tera template with the given theme.
pub fn render_template(
    theme_name: &str,
    theme: Theme,
    template: &str,
) -> Result<String, tera::Error> {
    let palette = Palette::from(theme);

    let mut tera = Tera::default();
    let mut context = Context::new();
    context.insert("accent", &palette.blue);
    context.insert("palette", &palette);
    context.insert("flavor", &Flavor::new(theme_name, palette));

    tera.render_str(template, &context)
}

/// Theme metadata for Tera templates
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Flavor {
    identifier: String,
    name: String,
    colors: Palette,
}

impl Flavor {
    pub fn new(theme_name: &str, colors: Palette) -> Self {
        let name = capitalize_words(
            &theme_name
                .replace('_', " ")
                .replace('-', " ")
                .to_lowercase(),
        );
        Self {
            identifier: theme_name.to_string(),
            colors,
            name,
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
