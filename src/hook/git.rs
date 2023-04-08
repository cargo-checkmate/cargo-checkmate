use crate::srcbundle::SourceBundle;

use std::path::PathBuf;

pub(super) fn source_bundle() -> std::io::Result<SourceBundle> {
    Ok(SourceBundle::new(
        "git-hook",
        "git-hook.pre-commit",
        hook_path()?,
        true,
    ))
}

fn hook_path() -> std::io::Result<PathBuf> {
    Ok(git_dir()?.join("hooks").join("pre-commit"))
}

fn git_dir() -> std::io::Result<PathBuf> {
    let s = crate::git::run(&["rev-parse", "--git-dir"])?;
    Ok(PathBuf::from(s.trim_end()))
}
