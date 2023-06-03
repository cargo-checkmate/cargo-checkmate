use crate::executable::Executable;
use crate::git;
use anyhow_std::PathAnyhow;
use std::path::{Path, PathBuf};

const CONTENTS: &str = include_str!("githook/pre-commit.sh");

/// git hook
#[derive(Debug, PartialEq, Eq, clap::Subcommand)]
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
    use crate::phase::Phase;

    println!("cargo checkmate git-hook:");
    check_dirty()?;
    Phase::execute_everything()
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
    git::get_hook_path("pre-commit")
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

fn check_dirty() -> anyhow::Result<()> {
    use indoc::indoc;

    let output = git::run(&["status", "--porcelain"])?;
    let mut dirty = vec![];
    for line in output.lines() {
        if line.chars().nth(1) != Some(' ') {
            dirty.push(line);
        }
    }
    if dirty.is_empty() {
        Ok(())
    } else {
        println!();
        println!("These changes are not staged in git:");
        for line in dirty {
            println!("  {}", line.get(3..).unwrap());
        }
        println!();
        println!(indoc! { "
            The checkmate tool validates the current filesystem, but something different is staged for git commit. Since checkmate cannot validate the commit contents, this git hook is aborting.

            To save unstaged changes for after a commit, see `git stash`.
        "});
        Err(anyhow::anyhow!("mixed git commit"))
    }
}
