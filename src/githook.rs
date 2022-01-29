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

macro_rules! hookname {
    () => {
        "pre-commit"
    };
}
const HOOK_BODY: &[u8] = include_bytes!(concat!("githook-", hookname!(), ".sh"));

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
    let mut pb = git_dir()?;
    pb.push("hooks");
    pb.push(hookname!());
    Ok(pb)
}

fn git_dir() -> IOResult<PathBuf> {
    use crate::ioerror;
    use std::io::Write;
    use std::process::Command;

    let gitout = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .output()?;

    let outbytes = &gitout.stdout[..];
    let errbytes = &gitout.stderr[..];

    // Echo any stderr output:
    std::io::stderr().write_all(errbytes)?;

    if gitout.status.success() && errbytes.is_empty() {
        let s = std::str::from_utf8(outbytes)
            .map_err(|e| ioerror!("{:?} git-dir not utf8; bytes: {:?}", e, outbytes))?;
        let stripped = s
            .strip_suffix('\n')
            .ok_or_else(|| ioerror!("git-dir missing expected terminal newline: {:?}", s))?;
        Ok(PathBuf::from(stripped))
    } else {
        Err(ioerror!(
            "git rev-parse --git-dir exit {:?}{}",
            gitout.status,
            if errbytes.is_empty() {
                ""
            } else {
                " with stderr noise."
            }
        ))
    }
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
