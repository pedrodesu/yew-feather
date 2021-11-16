use convert_case::{Case, Casing};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const SLUG_FILE: &str = include_str!("build/slug.rs");
const SLUG_LIB: &str = include_str!("build/lib_slug.rs");

fn to_type_name(icon_name: &str) -> String
{
    icon_name.from_case(Case::Kebab).to_case(Case::Pascal)
}
fn to_mod_name(icon_name: &str) -> String
{
    let preliminary = icon_name.from_case(Case::Kebab).to_case(Case::Snake);
    format!("r#{}", preliminary)
}
fn _to_mod_path(icon_name: &str) -> PathBuf
{
    PathBuf::from(format!("{}.rs", icon_name))
}

fn main() -> Result<()>
{
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=build/");
    println!("cargo:rerun-if-changed=package-lock.json");
    Command::new("npm")
        .args(["ci", "--audit=false"])
        .status()?
        .success()
        .then(|| ())
        .ok_or_else(|| Error::new(ErrorKind::Other, "failed to install npm dependencies"))?;
    let icons_json = Command::new("node")
        .args(["-pe", "require.resolve('feather-icons/dist/icons.json')"])
        .output()?
        .stdout;

    let icons_json = String::from_utf8(icons_json).expect("valid utf-8 path name");
    let icons_json = File::open(icons_json.trim()).expect("the icons.json file to exist");
    let icons_json: HashMap<String, String> =
        serde_json::from_reader(icons_json).expect("valid json in icons.json");

    let output_dir = env::var("OUT_DIR").expect("an OUT_DIR be configured");
    let output_dir = PathBuf::from(output_dir).join("feather-src");
    fs::create_dir_all(&output_dir).expect("creating output subdir successful");
    let mut lib_content = String::from(SLUG_LIB);
    lib_content.extend(icons_json.iter().map(|(icon_name, icon_contents)| {
        let icon_content = SLUG_FILE
            .replace("[name]", &to_type_name(icon_name))
            .replace("[markup]", icon_contents);
        let module_name = to_mod_name(icon_name);
        format!(
            r#"
pub mod {module_name} {{
{icon_content}
}}
"#,
            module_name = module_name,
            icon_content = icon_content
        )
    }));
    fs::write(output_dir.join("lib.rs"), lib_content.as_bytes()).expect("can write to lib.rs file");
    Ok(())
}
