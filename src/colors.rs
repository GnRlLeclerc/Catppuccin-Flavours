//! Color palette used in Tera templates, with different color formats.

use serde::Serialize;

use crate::{cli::AccentColor, themes::Theme};

/// Serializable color palette for injection into tera templates.
#[derive(Debug, Clone, Serialize)]
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
#[derive(Debug, Clone, Serialize)]
pub struct Color {
    /// Color hex value
    pub hex: String,
    /// Color hex value (without the leading `#`)
    pub rawhex: String,
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
        let rawhex = hex.trim_start_matches('#').to_string();
        Self {
            hex,
            rawhex,
            r,
            g,
            b,
        }
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

impl Palette {
    /// Get the full color value for a given accent color.
    pub fn accent(&self, accent: AccentColor) -> &Color {
        match accent {
            AccentColor::Rosewater => &self.rosewater,
            AccentColor::Flamingo => &self.flamingo,
            AccentColor::Pink => &self.pink,
            AccentColor::Mauve => &self.mauve,
            AccentColor::Red => &self.red,
            AccentColor::Maroon => &self.maroon,
            AccentColor::Peach => &self.peach,
            AccentColor::Yellow => &self.yellow,
            AccentColor::Green => &self.green,
            AccentColor::Teal => &self.teal,
            AccentColor::Sky => &self.sky,
            AccentColor::Sapphire => &self.sapphire,
            AccentColor::Blue => &self.blue,
            AccentColor::Lavender => &self.lavender,
        }
    }
}
