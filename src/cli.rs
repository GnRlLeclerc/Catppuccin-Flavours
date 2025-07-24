//! CLI commands

use std::io;

use clap::{Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};

/// Generate themes from Catppuccin templates.
#[derive(Parser, Debug)]
pub struct Args {
    /// The command to run
    #[clap(subcommand)]
    pub command: Command,
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
    },
    /// Apply a theme to the current configuration
    Apply {
        /// Theme name
        theme: String,
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
