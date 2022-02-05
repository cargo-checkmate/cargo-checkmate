use crate::executable::Executable;
use crate::IOResult;
use std::path::PathBuf;
use structopt::StructOpt;

/// GitHub CI support.
#[derive(Debug, StructOpt)]
pub enum GithubCI {
    /// install github CI.
    Install,
    /// uninstall github CI.
    Uninstall,
}

impl Executable for GithubCI {
    fn execute(&self) -> IOResult<()> {
        use crate::srcbundle::SourceBundle;
        use GithubCI::*;

        let sb = SourceBundle {
            name: "GitHub CI",
            dest: yaml_path()?,
            contents: include_bytes!("github-ci.yaml"),
            executable: false,
        };

        match self {
            Install => sb.install(),
            Uninstall => sb.uninstall(),
        }
    }
}

fn yaml_path() -> IOResult<PathBuf> {
    use crate::CMDNAME;

    Ok(git_toplevel()?
        .join(".github")
        .join("workflows")
        .join(format!("{}.yaml", CMDNAME)))
}

fn git_toplevel() -> IOResult<PathBuf> {
    let s = crate::git::run(&["rev-parse", "--show-toplevel"])?;
    Ok(PathBuf::from(s.trim_end()))
}
