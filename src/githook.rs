use crate::executable::Executable;
use anyhow_std::PathAnyhow;
use std::path::{Path, PathBuf};

const CONTENTS: &str = include_str!("githook/pre-commit.sh");

/// git hook
#[derive(Debug, clap::Subcommand)]
pub enum GitHook {
    /// install git hook, overwriting if present
    Install,
    /// delete the git hook
    Uninstall,
    /// run git hook
    Run,
    /// print the path of the hook
    Path,
    /// print the contents of the hook
    Print,
}

impl Executable for GitHook {
    fn execute(&self) -> anyhow::Result<()> {
        use GitHook::*;

        match self {
            Install => install(),
            Uninstall => uninstall(),
            Run => run(),
            Path => print_path(),
            Print => print_contents(),
        }
    }
}

pub fn install() -> anyhow::Result<()> {
    let p = get_path()?;
    println!("Installing: {:?}", p.display());
    p.write_anyhow(CONTENTS)?;
    make_executable(&p)?;
    Ok(())
}

pub fn uninstall() -> anyhow::Result<()> {
    let p = get_path()?;
    println!("Uninstalling (deleting): {:?}", p.display());
    p.remove_file_anyhow()
}

pub fn run() -> anyhow::Result<()> {
    use crate::options::Subcommand::Everything;

    println!("cargo checkmate git-hook:");

    // TODO: Ensure the repo is clean first.
    Everything.execute()
}

pub fn print_path() -> anyhow::Result<()> {
    println!("{}", get_path()?.display());
    Ok(())
}

pub fn print_contents() -> anyhow::Result<()> {
    println!("{}", CONTENTS);
    Ok(())
}

pub fn get_path() -> anyhow::Result<PathBuf> {
    crate::git::get_hook_path("pre-commit")
}

// TODO: implement for other platforms:
fn make_executable(p: &Path) -> anyhow::Result<()> {
    use anyhow::Context;
    use std::os::unix::fs::PermissionsExt;

    let mut perms = p.metadata_anyhow()?.permissions();
    // Set user read/execute perms on unix:
    perms.set_mode(perms.mode() | 0o500);
    std::fs::set_permissions(p, perms).with_context(|| format!("-for path {:?}", p.display()))?;
    Ok(())
}
