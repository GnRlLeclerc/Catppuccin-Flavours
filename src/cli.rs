//! CLI commands

use std::io;

use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::{Generator, Shell, generate};

/// Generate themes from Catppuccin templates.
#[derive(Parser, Debug)]
pub struct Args {
    /// The command to run
    #[clap(subcommand)]
    pub command: Command,
}

/// Catppuccin accent color.
#[derive(Copy, Clone, Debug, ValueEnum, Default)]
pub enum AccentColor {
    Rosewater,
    Flamingo,
    Pink,
    #[default]
    Mauve,
    Red,
    Maroon,
    Peach,
    Yellow,
    Green,
    Teal,
    Sky,
    Sapphire,
    Blue,
    Lavender,
}

impl ToString for AccentColor {
    fn to_string(&self) -> String {
        match self {
            AccentColor::Rosewater => "rosewater".to_string(),
            AccentColor::Flamingo => "flamingo".to_string(),
            AccentColor::Pink => "pink".to_string(),
            AccentColor::Mauve => "mauve".to_string(),
            AccentColor::Red => "red".to_string(),
            AccentColor::Maroon => "maroon".to_string(),
            AccentColor::Peach => "peach".to_string(),
            AccentColor::Yellow => "yellow".to_string(),
            AccentColor::Green => "green".to_string(),
            AccentColor::Teal => "teal".to_string(),
            AccentColor::Sky => "sky".to_string(),
            AccentColor::Sapphire => "sapphire".to_string(),
            AccentColor::Blue => "blue".to_string(),
            AccentColor::Lavender => "lavender".to_string(),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// List all available themes
    ListThemes,
    /// List all available templates
    ListTemplates,
    /// Render a template with a theme
    Template {
        /// Template name
        template: String,
        /// Theme name
        theme: String,
        /// Accent color
        #[clap(long, short, default_value_t = AccentColor::Mauve)]
        accent: AccentColor,
    },
    /// Apply a theme to the current configuration
    Apply {
        /// Theme name
        theme: String,
        /// Accent color
        #[clap(long, short, default_value_t = AccentColor::Mauve)]
        accent: AccentColor,
    },
    /// Generate shell completions
    Completions {
        /// The shell to generate completions for
        shell: Shell,
    },
}

/// Print completions for the given command to stdout
pub fn print_completions<G: Generator>(generator: G, cmd: &mut clap::Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}
