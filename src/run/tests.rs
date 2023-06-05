use super::run_with_args;
use anyhow_std::PathAnyhow;

#[test]
fn help_outside_of_crate_path() -> anyhow::Result<()> {
    let td = tempfile::TempDir::new()?;
    td.as_ref().set_to_current_dir_anyhow()?;

    match run_with_args(["--help"]) {
        Ok(()) => panic!("expected help output"),
        Err(_) => todo!("verify help output"),
    }
}
