use crate::executable::Executable;
use anyhow_std::PathAnyhow;
use std::path::{Path, PathBuf};

const CONTENTS: &str = include_str!("gh/cargo-checkmate.yaml");

/// github integration
#[derive(Debug, PartialEq, Eq, clap::Subcommand)]
pub enum Gh {
    /// install github CI workflow, overwriting if present
    Install,
    /// delete the github CI workflow
    Uninstall,
    /// print the path of the github CI workflow
    Path,
    /// print the contents of the github CI workflow
    Print,
}

impl Executable for Gh {
    fn execute(&self) -> anyhow::Result<()> {
        use Gh::*;

        match self {
            Install => install(),
            Uninstall => uninstall(),
            Path => print_path(),
            Print => print_contents(),
        }
    }
}

pub fn install() -> anyhow::Result<()> {
    let p = get_path();
    println!("Installing: {:?}", p.display());
    p.write_anyhow(CONTENTS)?;
    Ok(())
}

pub fn uninstall() -> anyhow::Result<()> {
    let p = get_path();
    println!("Uninstalling (deleting): {:?}", p.display());
    p.remove_file_anyhow()
}

pub fn print_path() -> anyhow::Result<()> {
    println!("{}", get_path().display());
    Ok(())
}

pub fn print_contents() -> anyhow::Result<()> {
    println!("{}", CONTENTS);
    Ok(())
}

pub fn get_path() -> PathBuf {
    Path::new(".github")
        .join("workflows")
        .join("cargo-checkmate.yaml")
}
