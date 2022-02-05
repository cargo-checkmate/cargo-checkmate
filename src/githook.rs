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
        use crate::srcbundle::SourceBundle;
        use GitHook::*;

        let sb = SourceBundle {
            name: "git-hook",
            dest: hook_path()?,
            contents: include_bytes!("githook-pre-commit.sh"),
            executable: true,
        };

        match self {
            Install => sb.install(),
            Uninstall => sb.uninstall(),
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
