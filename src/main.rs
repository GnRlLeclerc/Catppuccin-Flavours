use std::fs;

use clap::{Parser, Subcommand};
use template::render_template;
use themes::get_theme;

pub mod colors;
pub mod config;
pub mod template;
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
        Command::Template { template, theme } => {
            let theme_name = theme;
            let theme = get_theme(&theme_name).unwrap_or_else(|| {
                eprintln!("Theme '{theme_name}' not found.");
                std::process::exit(1);
            });

            // Read the template file
            let template = fs::read_to_string(template).expect("Failed to read template file");

            let rendered =
                render_template(&theme_name, theme, &template).expect("Failed to render template");

            println!("Rendered template:\n{rendered}");
        }
    }
}
