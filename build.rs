//! Include the contents of the `themes/` folder in the binary.

use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut map = phf_codegen::Map::new();

    // Iterate over all files in the themes folder
    fs::read_dir("themes").unwrap().for_each(|entry| {
        let path = entry.unwrap().path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let theme_name = filename.strip_suffix(".toml").unwrap().to_string();
        let include = format!("include_str!(\"../../../../../themes/{filename}\")");

        map.entry(theme_name, include);
    });

    write!(
        &mut file,
        "static BUILTIN_THEMES: phf::Map<&'static str, &'static str> = {};",
        map.build()
    )
    .unwrap();
}
