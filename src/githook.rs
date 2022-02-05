use crate::executable::Executable;
use crate::IOResult;
use std::path::{Path, PathBuf};
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
        use GitHook::*;

        match self {
            Install => install(),
            Uninstall => uninstall(),
        }
    }
}

const HOOK_BODY: &[u8] = include_bytes!("githook-pre-commit.sh");

fn install() -> IOResult<()> {
    use crate::CMDNAME;
    use std::io::Write;

    let hookpath = hook_path()?;

    let openres = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&hookpath);

    match openres {
        Ok(mut f) => {
            f.write_all(HOOK_BODY)?;
            make_executable(f)?;
            println!("{} git-hook installed: {:?}", CMDNAME, &hookpath);
            Ok(())
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => {
                if hook_contents_recognized(&hookpath)? {
                    println!("{} git-hook already installed: {:?}", CMDNAME, &hookpath);
                    Ok(())
                } else {
                    unrecognized_hook_contents(&hookpath)
                }
            }
            _ => Err(e),
        },
    }
}

fn uninstall() -> IOResult<()> {
    let hookpath = hook_path()?;

    match hook_contents_recognized(&hookpath) {
        Ok(true) => {
            use crate::CMDNAME;
            std::fs::remove_file(&hookpath)?;
            println!("{} git-hook uninstalled: {:?}", CMDNAME, &hookpath);
            Ok(())
        }
        Ok(false) => unrecognized_hook_contents(&hookpath),
        Err(e) => Err(e),
    }
}

fn hook_path() -> IOResult<PathBuf> {
    Ok(git_dir()?.join("hooks").join("pre-commit"))
}

fn git_dir() -> IOResult<PathBuf> {
    let s = crate::git::run(&["rev-parse", "--git-dir"])?;
    Ok(PathBuf::from(s.trim_end()))
}

fn make_executable(f: std::fs::File) -> IOResult<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = f.metadata()?.permissions();
    // Set user read/write perms on unix:
    perms.set_mode(perms.mode() | 0o500);
    f.set_permissions(perms)?;
    Ok(())
}

fn hook_contents_recognized(hookpath: &Path) -> IOResult<bool> {
    let contents = std::fs::read(&hookpath)?;
    Ok(contents == HOOK_BODY)
}

fn unrecognized_hook_contents(hookpath: &Path) -> IOResult<()> {
    use crate::{ioerror, CMDNAME};

    println!("{} unrecognized git-hook: {:?}", CMDNAME, &hookpath);
    Err(ioerror!("Unrecongized git-hook: {:?}", &hookpath))
}
