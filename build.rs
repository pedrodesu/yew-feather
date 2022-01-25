use convert_case::{Case, Casing};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const LIB_SLUG: &str = include_str!("build/lib_slug.rs");
const SLUG: &str = include_str!("build/slug.rs");

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build/");
    println!("cargo:rerun-if-changed=package-lock.json");

    Command::new(if cfg!(target_os = "windows") {
        "npm.cmd"
    } else {
        "npm"
    })
    .args(["ci", "--audit=false"])
    .status()?
    .success()
    .then(|| ())
    .ok_or_else(|| Error::new(ErrorKind::Other, "failed to install npm dependencies"))?;

    let collection: HashMap<String, String> =
        serde_json::from_reader(File::open(PathBuf::from(format!(
            "{}\\node_modules\\feather-icons\\dist\\icons.json",
            env::current_dir().unwrap().to_str().unwrap()
        )))?)?;

    let lib_content = [
        LIB_SLUG.to_owned(),
        collection
            .iter()
            .map(|(name, markup)| {
                let safe_snake_name = format!("r#{}", name.to_case(Case::Snake));

                let markup = SLUG
                    .replace("__ComponentName", &name.to_case(Case::Pascal))
                    .replace("__component_func", &safe_snake_name)
                    .replace("__component_markup", markup);

                format!("\npub mod {} {{\n{}\n}}\n", safe_snake_name, markup)
            })
            .collect::<String>(),
    ]
    .concat();

    fs::write("src/lib.rs", lib_content.as_bytes()).expect("can write to lib.rs file");

    Ok(())
}
