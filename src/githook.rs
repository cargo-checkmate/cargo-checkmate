use crate::cdcrate::change_directory_to_crate_root;
use crate::executable::Executable;
use crate::git;
use anyhow_std::PathAnyhow;
use std::path::PathBuf;

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
    #[cfg(target_family = "unix")]
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

    change_directory_to_crate_root()?;

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
    change_directory_to_crate_root()?;

    git::get_hook_path("pre-commit")
}

#[cfg(target_family = "unix")]
fn make_executable(p: &std::path::Path) -> anyhow::Result<()> {
    use anyhow::Context;
    use std::os::unix::fs::PermissionsExt;

    let mut perms = p.metadata_anyhow()?.permissions();
    // Set user read/execute perms on unix:
    perms.set_mode(perms.mode() | 0o500);
    std::fs::set_permissions(p, perms).with_context(|| format!("-for path {:?}", p.display()))?;
    Ok(())
}

fn check_dirty() -> anyhow::Result<()> {
    let dirty = get_git_unstaged()?;
    git_unstaged_to_error_message(dirty)?;
    Ok(())
}

fn get_git_unstaged() -> anyhow::Result<Vec<String>> {
    let output = git::run(&["status", "--porcelain"])?;
    parse_git_status_porcelain(&output)
}

fn parse_git_status_porcelain(output: &str) -> anyhow::Result<Vec<String>> {
    let mut dirty = vec![];
    for line in output.lines() {
        if line.is_char_boundary(3) {
            let (info, content) = line.split_at(3);
            if info.chars().nth(1) != Some(' ') {
                dirty.push(content.to_string());
            }
        } else {
            anyhow::bail!("unexpected output of `git status --porcelain`: {line:?}");
        }
    }
    Ok(dirty)
}

fn git_unstaged_to_error_message(dirty: Vec<String>) -> anyhow::Result<()> {
    use indoc::indoc;
    if dirty.is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "These files have changes that are not staged in git:\n  {}",
            dirty.join("\n  "),
        )
        .context(indoc! { r#"
            The checkmate tool validates the current filesystem, but something different
            is staged for git commit. Since checkmate cannot validate the commit contents,
            this git hook is aborting.

            To save unstaged changes for after a commit, see `git stash`.
        "# }))
    }
}

#[cfg(test)]
mod tests;
