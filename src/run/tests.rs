use anyhow_std::PathAnyhow;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[test]
fn help_outside_of_crate_path() -> anyhow::Result<()> {
    run("cargo", &["build", "-v"])?;

    let exe = find_executable()?;
    let td = tempfile::TempDir::new()?;
    td.as_ref().set_to_current_dir_anyhow()?;

    run(exe, &["--help"])
}

fn run<T>(exe: T, args: &[&str]) -> anyhow::Result<()>
where
    T: AsRef<OsStr>,
{
    use std::process::Command;

    let mut cmd = Command::new(exe);
    cmd.args(args);
    // cmd.stdout(std::process::Stdio::null());
    eprintln!("running: {:?}", &cmd);
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("exit: {status:?}");
    }
}

fn find_executable() -> anyhow::Result<PathBuf> {
    try_find_executable("target")?.map(Ok).unwrap_or_else(|| Err(anyhow::anyhow!("could not find executable".to_string())))
}

fn try_find_executable<P>(dir: P) -> anyhow::Result<Option<PathBuf>>
where P: AsRef<Path>,
{
    for dirres in dir.as_ref().read_dir_anyhow()? {
        let dirent = dirres?;
        let md = dirent.metadata()?;
        let childpath = dirent.path();
        if md.is_dir() {
            if let Some(p) = try_find_executable(&childpath)? {
                return Ok(Some(p));
            }
        } else if md.is_file() {
            use anyhow_std::OsStrAnyhow;

            if childpath.file_name_anyhow()?.to_str_anyhow()? == env!("CARGO_PKG_NAME") {
                return Ok(Some(childpath.canonicalize_anyhow()?));
            }
        }
    }
    Ok(None)
}
