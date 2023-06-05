use anyhow_std::PathAnyhow;
use std::ffi::OsStr;
use std::path::Path;

#[test]
fn help_outside_of_crate_path() -> anyhow::Result<()> {
    run("cargo", "build")?;
    let exe = Path::new("target")
        .join("debug")
        .join(env!("CARGO_PKG_NAME"))
        .canonicalize_anyhow()?;

    let td = tempfile::TempDir::new()?;
    td.as_ref().set_to_current_dir_anyhow()?;

    run(exe, "--help")
}

fn run<T>(exe: T, arg: &str) -> anyhow::Result<()>
where
    T: AsRef<OsStr>,
{
    use std::process::{Command, Stdio};

    let mut cmd = Command::new(exe);
    cmd.arg(arg);
    cmd.stdout(Stdio::null());
    println!("running: {:?}", &cmd);
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("exit: {status:?}");
    }
}
