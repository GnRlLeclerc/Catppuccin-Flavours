use colors::Colors;

pub mod colors;

fn main() {
    // TODO:
    // - add builtin ones within (like, print them on demand just to see)
    // - print themes for debug
    // - use clap for pretty CLI
    // - process tera templates
    println!("Hello, world!");

    let path = "themes/catppuccin-frappe.toml";
    let content = std::fs::read_to_string(path).expect("Failed to read the file");

    println!("Content of {}: \n{}", path, content);

    let colors: Colors =
        toml::from_str(&content).expect("Failed to parse TOML content into Colors struct");

    println!("Parsed Colors: {:#?}", colors);
}
