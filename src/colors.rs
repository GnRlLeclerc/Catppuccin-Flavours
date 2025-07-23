use serde::{Deserialize, Serialize};

/// Catppuccin color palette
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

/// Serializable color palette for injection into tera templates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    pub rosewater: Color,
    pub flamingo: Color,
    pub pink: Color,
    pub mauve: Color,
    pub red: Color,
    pub maroon: Color,
    pub peach: Color,
    pub yellow: Color,
    pub green: Color,
    pub teal: Color,
    pub sky: Color,
    pub sapphire: Color,
    pub blue: Color,
    pub lavender: Color,
    pub text: Color,
    pub subtext1: Color,
    pub subtext0: Color,
    pub overlay2: Color,
    pub overlay1: Color,
    pub overlay0: Color,
    pub surface2: Color,
    pub surface1: Color,
    pub surface0: Color,
    pub base: Color,
    pub mantle: Color,
    pub crust: Color,
}

/// A theme color
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    /// Color hex value (without the leading `#`)
    pub hex: String,
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
}

impl From<String> for Color {
    fn from(hex: String) -> Self {
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
        let hex = hex.trim_start_matches('#').to_string();
        Self { hex, r, g, b }
    }
}

impl From<Theme> for Palette {
    fn from(theme: Theme) -> Self {
        Self {
            rosewater: Color::from(theme.rosewater),
            flamingo: Color::from(theme.flamingo),
            pink: Color::from(theme.pink),
            mauve: Color::from(theme.mauve),
            red: Color::from(theme.red),
            maroon: Color::from(theme.maroon),
            peach: Color::from(theme.peach),
            yellow: Color::from(theme.yellow),
            green: Color::from(theme.green),
            teal: Color::from(theme.teal),
            sky: Color::from(theme.sky),
            sapphire: Color::from(theme.sapphire),
            blue: Color::from(theme.blue),
            lavender: Color::from(theme.lavender),
            text: Color::from(theme.text),
            subtext1: Color::from(theme.subtext1),
            subtext0: Color::from(theme.subtext0),
            overlay2: Color::from(theme.overlay2),
            overlay1: Color::from(theme.overlay1),
            overlay0: Color::from(theme.overlay0),
            surface2: Color::from(theme.surface2),
            surface1: Color::from(theme.surface1),
            surface0: Color::from(theme.surface0),
            base: Color::from(theme.base),
            mantle: Color::from(theme.mantle),
            crust: Color::from(theme.crust),
        }
    }
}
