use convert_case::{Case, Casing};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Result};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use which::which;

const LIB_SLUG: &str = include_str!("build/lib_slug.rs");
const SLUG: &str = include_str!("build/slug.rs");

const PACKAGE_MANAGERS: &[&str] = &["pnpm", "yarn", "npm"];

#[inline]
fn get_package_manager() -> PathBuf {
    PACKAGE_MANAGERS
        .into_iter()
        .find_map(|&m| which(m).ok())
        .unwrap()
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build/");
    println!("cargo:rerun-if-changed=package-lock.json");

    Command::new(get_package_manager())
        .arg("install")
        .status()?;

    let file = File::open(
        ["node_modules", "feather-icons", "dist", "icons.json"]
            .into_iter()
            .fold(env::current_dir()?, |acc, path| acc.join(path)),
    )?;

    let collection: HashMap<String, String> = serde_json::from_reader(BufReader::new(file))?;

    let lib_content = [
        LIB_SLUG.to_owned(),
        collection
            .into_iter()
            .map(|(name, markup)| {
                let markup = SLUG
                    .replace("__ComponentName", name.to_case(Case::Pascal).as_str())
                    .replace(
                        "__component_func",
                        format!("r#{}", name.to_case(Case::Snake)).as_str(),
                    )
                    .replace("__component_markup", markup.as_str());

                format!("\n{}\n", markup)
            })
            .collect(),
    ]
    .concat();

    fs::write("src/lib.rs", lib_content.as_bytes()).expect("can write to lib.rs file");

    Ok(())
}
