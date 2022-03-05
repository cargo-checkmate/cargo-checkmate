use crate::srcbundle::SourceBundle;
use crate::IOResult;
use std::path::PathBuf;

pub(super) fn source_bundle() -> IOResult<SourceBundle> {
    Ok(SourceBundle::new(
        "git-hook",
        "git-hook.pre-commit",
        hook_path()?,
        true,
    ))
}

fn hook_path() -> IOResult<PathBuf> {
    Ok(git_dir()?.join("hooks").join("pre-commit"))
}

fn git_dir() -> IOResult<PathBuf> {
    let s = crate::git::run(&["rev-parse", "--git-dir"])?;
    Ok(PathBuf::from(s.trim_end()))
}
