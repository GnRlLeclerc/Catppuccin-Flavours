use std::{fs, process::Stdio};

use clap::{CommandFactory, Parser};
use cli::{AccentColor, Args, Command, print_completions};
use colors::Palette;
use config::{CONFIG_FILE, Config, Entry};
use rayon::prelude::*;
use render::render_template;
use themes::get_theme;

pub mod cli;
pub mod colors;
pub mod config;
pub mod render;
pub mod template;
pub mod themes;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Completions { shell } => {
            let mut cmd = Args::command();
            print_completions(shell, &mut cmd);
        }
        Command::ListThemes => {
            themes::list_all().iter().for_each(|theme| {
                println!("{theme}");
            });
        }
        Command::ListTemplates => {
            template::list_all().iter().for_each(|template| {
                println!("{template}");
            });
        }
        Command::Template {
            template,
            theme: name,
            accent,
        } => {
            let theme = get_theme(&name).unwrap_or_else(|| {
                eprintln!("Theme '{name}' not found.");
                std::process::exit(1);
            });
            let palette = Palette::from(theme);

            // Read the template file
            let template = fs::read_to_string(template).expect("Failed to read template file");

            let rendered = render_template(&name, &palette, accent, &template)
                .expect("Failed to render template");

            println!("Rendered template:\n{rendered}");
        }
        Command::Apply {
            theme: name,
            accent,
        } => {
            let theme = get_theme(&name).expect("Theme not found");
            let palette = Palette::from(theme);

            let config =
                fs::read_to_string(&*CONFIG_FILE).expect("Failed to read configuration file");
            let config: Config =
                toml::from_str(&config).expect("Failed to parse configuration file");

            // Parallel iteration using rayon for commands that might take time
            config.entries.into_par_iter().for_each(|entry| {
                if let Err(e) = process_entry(&name, &palette, accent, entry) {
                    eprintln!("{}", e);
                }
            });
        }
    }
}

/// Process an entry from the config file.
fn process_entry(
    theme_name: &str,
    palette: &Palette,
    accent: AccentColor,
    entry: Entry,
) -> Result<(), String> {
    let template = template::get_template(&entry.template)?;
    let rendered = render_template(theme_name, palette, accent, &template)
        .map_err(|e| format!("Failed to render template: {}", e))?;

    // Write the rendered template to the target file
    let target = shellexpand::tilde(&entry.target);
    fs::create_dir_all(&*target)
        .map_err(|e| format!("Failed to create target directory: {}", e))?;
    fs::write(&*target, rendered).map_err(|e| format!("Failed to write to target file: {}", e))?;

    // If a command is specified, run it
    if let Some(command) = entry.command {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdout(Stdio::null()) // Suppress output
            .status()
            .map_err(|e| format!("Failed to run command '{}': {}", command, e))?;
    }

    Ok(())
}
