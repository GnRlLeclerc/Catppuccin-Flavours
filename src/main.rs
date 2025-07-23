use clap::{Parser, Subcommand};
use colors::Colors;

pub mod colors;

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
            if let Some(theme) = BUILTIN_THEMES.get(theme.as_str()) {
                let theme: Colors = toml::from_str(theme).unwrap();
                println!("{:#?}", theme);
                return;
            }

            // TODO: search in config directory
            eprintln!("Theme '{theme}' not found.");
            std::process::exit(1);
        }
    }
}
