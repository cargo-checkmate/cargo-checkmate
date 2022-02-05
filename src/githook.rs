use crate::executable::Executable;
use crate::IOResult;
use std::path::PathBuf;
use structopt::StructOpt;

/// git-hook support.
#[derive(Debug, StructOpt)]
pub enum GitHook {
    /// install git-hook.
    Install,
    /// uninstall git-hook.
    Uninstall,
}

impl Executable for GitHook {
    fn execute(&self) -> IOResult<()> {
        use crate::srcbundle::{install, uninstall};
        use GitHook::*;

        const HOOK_BODY: &[u8] = include_bytes!("githook-pre-commit.sh");
        let dest = hook_path()?;

        match self {
            Install => install("git-hook", &dest, HOOK_BODY, true),
            Uninstall => uninstall("git-hook", &dest, HOOK_BODY),
        }
    }
}

fn hook_path() -> IOResult<PathBuf> {
    Ok(git_dir()?.join("hooks").join("pre-commit"))
}

fn git_dir() -> IOResult<PathBuf> {
    let s = crate::git::run(&["rev-parse", "--git-dir"])?;
    Ok(PathBuf::from(s.trim_end()))
}
