use crate::srcbundle::SourceBundle;

use std::path::PathBuf;

pub(super) fn source_bundle() -> std::io::Result<SourceBundle> {
    Ok(SourceBundle::new(
        "GitHub CI",
        "github-ci.yaml",
        yaml_path()?,
        false,
    ))
}

fn yaml_path() -> std::io::Result<PathBuf> {
    use crate::CMDNAME;

    Ok(git_toplevel()?
        .join(".github")
        .join("workflows")
        .join(format!("{}.yaml", CMDNAME)))
}

fn git_toplevel() -> std::io::Result<PathBuf> {
    let s = crate::git::run(&["rev-parse", "--show-toplevel"])?;
    Ok(PathBuf::from(s.trim_end()))
}
