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
    let gitdir = git_dir()?;
    let hookpath = gitdir.join("hooks").join(hookname!());

    {
        use std::io::Write;

        let mut f = std::fs::File::create(&hookpath)?;
        f.write_all(HOOK_BODY)?;

        {
            use std::os::unix::fs::PermissionsExt;

            let mut perms = f.metadata()?.permissions();
            // Set user read/write perms on unix:
            perms.set_mode(perms.mode() | 0500);
            f.set_permissions(perms)?;
        }
    }

    println!("git-hook installed: {:?}", hookpath);
    Ok(())
}

fn uninstall() -> IOResult<()> {
    unimplemented!("uninstall");
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

    if gitout.status.success() && errbytes.len() == 0 {
        let s = std::str::from_utf8(outbytes)
            .map_err(|e| ioerror!("{:?} git-dir not utf8; bytes: {:?}", e, outbytes))?;
        let stripped = s.strip_suffix('\n').ok_or(ioerror!(
            "git-dir missing expected terminal newline: {:?}",
            s
        ))?;
        Ok(PathBuf::from(stripped))
    } else {
        Err(ioerror!(
            "git rev-parse --git-dir exit {:?}{}",
            gitout.status,
            if errbytes.len() == 0 {
                ""
            } else {
                " with stderr noise."
            }
        ))
    }
}
