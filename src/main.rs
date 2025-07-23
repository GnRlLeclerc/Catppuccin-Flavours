use std::fs;

use clap::{Parser, Subcommand};
use colors::Colors;
use config::read_theme_from_config;
use tera::Tera;
use themes::get_theme;

pub mod colors;
pub mod config;
pub mod themes;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Generate themes from Catppuccin templates.
#[derive(Parser, Debug)]
struct Args {
    /// The command to run
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// List builtin themes
    ListBuiltin,
    /// Print a theme
    Print {
        /// Theme name
        theme: String,
    },
    /// Fill a template with a theme
    Template {
        /// Template name
        template: String,
        /// Theme name
        theme: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::ListBuiltin => {
            let mut themes: Vec<_> = BUILTIN_THEMES.keys().collect();
            themes.sort();

            themes.iter().for_each(|&theme| {
                println!("{theme}");
            });
        }
        Command::Print { theme } => {
            if let Some(theme) = get_theme(&theme) {
                println!("{:#?}", theme);
                return;
            }

            eprintln!("Theme '{theme}' not found.");
            std::process::exit(1);
        }
    }
}
