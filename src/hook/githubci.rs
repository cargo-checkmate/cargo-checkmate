use crate::srcbundle::SourceBundle;
use crate::IOResult;
use std::path::PathBuf;

pub(super) fn source_bundle() -> IOResult<SourceBundle> {
    Ok(SourceBundle {
        name: "GitHub CI",
        dest: yaml_path()?,
        contents: include_bytes!("github-ci.yaml"),
        executable: false,
    })
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
