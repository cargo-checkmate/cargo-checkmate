use super::run_with_args;
use anyhow_std::PathAnyhow;

#[test]
#[ignore]
fn help_outside_of_crate_path() -> anyhow::Result<()> {
    let td = tempfile::TempDir::new()?;
    td.as_ref().set_to_current_dir_anyhow()?;

    match run_with_args(["cargo-checkmate", "--help"]) {
        Ok(()) => panic!("expected help output"),
        Err(_) => todo!("verify help output"),
    }
}
