use std::io::{Error, ErrorKind, Result};
use std::process::Command;

fn main() -> Result<()>
{
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=bin/");
    println!("cargo:rerun-if-changed=package-lock.json");
    Command::new("npm")
        .args(["ci", "--audit=false"])
        .status()?
        .success()
        .then(|| ())
        .ok_or(Error::new(
            ErrorKind::Other,
            "failed to install npm dependencies",
        ))?;
    Command::new("npm")
        .args(["run-script", "build"])
        .status()?
        .success()
        .then(|| ())
        .ok_or(Error::new(
            ErrorKind::Other,
            "failed to install npm dependencies",
        ))?;
    Ok(())
}
