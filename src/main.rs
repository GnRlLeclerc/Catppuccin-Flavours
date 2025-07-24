use std::fs;

use clap::{CommandFactory, Parser};
use cli::{Args, Command, print_completions};
use config::{CONFIG_FILE, Config};
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
        } => {
            let theme = get_theme(&name).unwrap_or_else(|| {
                eprintln!("Theme '{name}' not found.");
                std::process::exit(1);
            });

            // Read the template file
            let template = fs::read_to_string(template).expect("Failed to read template file");

            let rendered =
                render_template(&name, theme, &template).expect("Failed to render template");

            println!("Rendered template:\n{rendered}");
        }
        Command::Apply { theme: name } => {
            let theme = get_theme(&name).expect("Theme not found");

            let config =
                fs::read_to_string(&*CONFIG_FILE).expect("Failed to read configuration file");
            let config: Config =
                toml::from_str(&config).expect("Failed to parse configuration file");

            // Parallel iteration using rayon for commands that might take time
            config.entries.into_par_iter().for_each(|entry| {
                let template = template::get_template(&entry.template).expect("Template not found");

                let rendered = render_template(&name, theme.clone(), &template)
                    .expect("Failed to render template");

                // Write the rendered template to the target file
                let target = shellexpand::tilde(&entry.target);
                println!("target: {}", target);

                fs::write(&*target, rendered).expect("Failed to write to target file");

                // If a command is specified, run it
                if let Some(command) = entry.command {
                    std::process::Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .status()
                        .expect("Failed to run command");
                }
            });
        }
    }
}
